//! Headless subcommands over `Store`; a running TUI sees the changes through its watcher.

use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use anyhow::{Result, bail};
use nucleo_matcher::Matcher;

use crate::config::Config;
use crate::index::{self, Filter};
use crate::note::link_key;
use crate::store::{SaveOutcome, Store};

#[derive(clap::Subcommand)]
pub enum Cmd {
    /// List notes (newest first)
    Ls {
        /// Only notes with this tag (or a nested one)
        #[arg(long)]
        tag: Option<String>,
        /// Only notes under this folder (path or `root/sub` label)
        #[arg(long)]
        folder: Option<String>,
    },
    /// Fuzzy-search titles and bodies (best match first)
    Search {
        query: String,
        #[arg(long)]
        tag: Option<String>,
        #[arg(long)]
        folder: Option<String>,
    },
    /// Print a note's text
    Cat {
        /// Note id (`root/sub/stem`), path, or unique file stem
        note: String,
    },
    /// Create a note; body from --stdin is appended below the title
    New {
        title: Option<String>,
        /// Folder to create in (path or `root/sub` label); default: first root
        #[arg(long)]
        folder: Option<String>,
        /// Tags to put on the line below the title (repeatable)
        #[arg(long = "tag")]
        tags: Vec<String>,
        /// Read the body from standard input
        #[arg(long)]
        stdin: bool,
    },
    /// Replace a note's text with standard input; a changed title renames the file and updates [[links]]
    Write {
        /// Note id (`root/sub/stem`), path, or unique file stem
        note: String,
        /// Read the new text from standard input (required; makes the intent explicit)
        #[arg(long, required = true)]
        stdin: bool,
    },
    /// Append text to the end of a note
    Append {
        /// Note id (`root/sub/stem`), path, or unique file stem
        note: String,
        /// Text to append (alternative to --stdin)
        #[arg(conflicts_with = "stdin", allow_hyphen_values = true)]
        text: Option<String>,
        /// Read the text to append from standard input
        #[arg(long)]
        stdin: bool,
    },
    /// Move a note to the root's .Trash
    Trash {
        /// Note id (`root/sub/stem`), path, or unique file stem
        note: String,
    },
}

#[derive(serde::Serialize)]
struct NoteOut {
    id: String,
    path: PathBuf,
    title: String,
    folder: String,
    tags: Vec<String>,
    links: Vec<String>,
    backlinks: Vec<String>,
    created: String,
    modified: String,
    preview: String,
    /// Full text; only for single-note commands.
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
}

fn rfc3339(t: SystemTime) -> String {
    chrono::DateTime::<chrono::Local>::from(t).to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
}

fn note_out(store: &Store, i: usize, with_text: bool) -> NoteOut {
    let n = &store.notes[i];
    NoteOut {
        id: store.note_id(n),
        path: n.path.clone(),
        title: n.title.clone(),
        folder: n
            .path
            .parent()
            .map(|p| store.folder_label(p))
            .unwrap_or_default(),
        tags: n.tags.clone(),
        links: n.links.clone(),
        backlinks: store
            .backlinks(i)
            .into_iter()
            .map(|j| store.note_id(&store.notes[j]))
            .collect(),
        created: rfc3339(n.created),
        modified: rfc3339(n.modified),
        preview: n.preview(),
        text: with_text.then(|| n.text.clone()),
    }
}

fn print_json<T: serde::Serialize>(v: &T) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(v)?);
    Ok(())
}

/// `#a #b` line for the note body: leading `#` and blanks stripped, empty tags dropped.
fn tag_line(tags: &[String]) -> Option<String> {
    let tags: Vec<String> = tags
        .iter()
        .map(|t| t.trim().trim_start_matches('#').trim())
        .filter(|t| !t.is_empty())
        .map(|t| format!("#{t}"))
        .collect();
    (!tags.is_empty()).then(|| tags.join(" "))
}

fn print_line(store: &Store, i: usize) {
    let n = &store.notes[i];
    let tags: Vec<String> = n.tags.iter().map(|t| format!("#{t}")).collect();
    println!("{}\t{}\t{}", store.note_id(n), n.title, tags.join(" "));
}

fn stem(path: &Path) -> &str {
    path.file_stem().and_then(|s| s.to_str()).unwrap_or("")
}

/// Note index for a path, an id (`root/sub/stem`, `.md` tolerated) or a unique file stem.
fn find(store: &Store, key: &str) -> Result<usize> {
    if let Ok(p) = fs::canonicalize(key)
        && let Some(i) = store.notes.iter().position(|n| n.path == p)
    {
        return Ok(i);
    }
    let bare = key.trim_end_matches(".md");
    if let Some(i) = store.notes.iter().position(|n| {
        let id = store.note_id(n);
        id == key || id == bare
    }) {
        return Ok(i);
    }
    let by_stem: Vec<usize> = (0..store.notes.len())
        .filter(|&i| stem(&store.notes[i].path) == bare)
        .collect();
    match by_stem.as_slice() {
        [i] => Ok(*i),
        [] => bail!("no note {key}"),
        many => {
            let ids: Vec<String> = many
                .iter()
                .map(|&i| store.note_id(&store.notes[i]))
                .collect();
            bail!("ambiguous: {}", ids.join(", "))
        }
    }
}

/// Folder path for an existing path or a `root` / `root/sub` label.
fn find_folder(store: &Store, key: &str) -> Result<PathBuf> {
    if let Ok(p) = fs::canonicalize(key)
        && (store.roots.iter().any(|r| r.dir == p) || store.folders.contains(&p))
    {
        return Ok(p);
    }
    store
        .roots
        .iter()
        .map(|r| &r.dir)
        .chain(store.folders.iter())
        .find(|d| store.folder_label(d) == key)
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("no folder {key}"))
}

fn list(store: &Store, query: &str, tag: Option<&str>, folder: Option<&str>) -> Result<Vec<usize>> {
    let filter = tag
        .map(|t| Filter::Tag(t.to_lowercase()))
        .unwrap_or(Filter::All);
    let mut matcher = Matcher::new(nucleo_matcher::Config::DEFAULT);
    let mut idx = index::visible(&store.notes, &filter, query, &mut matcher);
    if let Some(f) = folder {
        let dir = find_folder(store, f)?;
        idx.retain(|&i| store.notes[i].path.starts_with(&dir));
    }
    Ok(idx)
}

fn print_list(store: &Store, idx: &[usize], json: bool) -> Result<()> {
    if json {
        let out: Vec<NoteOut> = idx.iter().map(|&i| note_out(store, i, false)).collect();
        return print_json(&out);
    }
    for &i in idx {
        print_line(store, i);
    }
    Ok(())
}

pub fn run(cmd: Cmd, cfg: &Config, json: bool) -> Result<()> {
    if cfg.roots.is_empty() {
        bail!("no notes folder configured: run `tnotes` once or pass --dir");
    }
    let mut store = Store::load(&cfg.roots)?;
    match cmd {
        Cmd::Ls { tag, folder } => {
            let idx = list(&store, "", tag.as_deref(), folder.as_deref())?;
            print_list(&store, &idx, json)
        }
        Cmd::Search { query, tag, folder } => {
            let idx = list(&store, &query, tag.as_deref(), folder.as_deref())?;
            print_list(&store, &idx, json)
        }
        Cmd::Cat { note } => {
            let i = find(&store, &note)?;
            if json {
                print_json(&note_out(&store, i, true))
            } else {
                print!("{}", store.notes[i].text);
                Ok(())
            }
        }
        Cmd::New {
            title,
            folder,
            tags,
            stdin,
        } => {
            let title = title
                .map(|t| t.trim().to_string())
                .filter(|t| !t.is_empty());
            if title.is_none() && !stdin {
                bail!("give a title or --stdin");
            }
            let dir = match folder {
                Some(f) => find_folder(&store, &f)?,
                None => cfg.roots[0].clone(),
            };
            let mut text = title.map(|t| format!("# {t}\n")).unwrap_or_default();
            if let Some(line) = tag_line(&tags) {
                text.push_str(&format!("\n{line}\n"));
            }
            if stdin {
                let body = read_stdin()?;
                if !text.is_empty() {
                    text.push('\n');
                }
                text.push_str(&body);
            }
            let i = store.create(&dir, &text)?;
            if json {
                print_json(&note_out(&store, i, true))
            } else {
                println!("{}", store.note_id(&store.notes[i]));
                Ok(())
            }
        }
        Cmd::Write { note, stdin: _ } => {
            let i = find(&store, &note)?;
            let text = read_stdin()?;
            save(&mut store, i, &text)?;
            if json {
                print_json(&note_out(&store, i, true))
            } else {
                println!("wrote {}", store.note_id(&store.notes[i]));
                Ok(())
            }
        }
        Cmd::Append { note, text, stdin } => {
            let i = find(&store, &note)?;
            let body = match text {
                Some(t) => t,
                None if stdin => read_stdin()?,
                None => bail!("give text or --stdin"),
            };
            let mut t = store.notes[i].text.clone();
            if !t.is_empty() && !t.ends_with('\n') {
                t.push('\n');
            }
            t.push_str(&body);
            if !body.ends_with('\n') {
                t.push('\n');
            }
            save(&mut store, i, &t)?;
            if json {
                print_json(&note_out(&store, i, true))
            } else {
                println!("appended to {}", store.note_id(&store.notes[i]));
                Ok(())
            }
        }
        Cmd::Trash { note } => {
            let i = find(&store, &note)?;
            let id = store.note_id(&store.notes[i]);
            store.trash(i)?;
            if json {
                print_json(&serde_json::json!({ "id": id, "trash_path": store.trash[0].path }))
            } else {
                println!("trashed {id}");
                Ok(())
            }
        }
    }
}

fn read_stdin() -> Result<String> {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s)?;
    Ok(s)
}

/// Save `text` into note `i` the way the TUI does: a conflict with a concurrent writer is an
/// error (the text survives as the conflict copy), a changed title rewrites `[[links]]` in
/// the other notes. Mirrors `App::save_tab`.
fn save(store: &mut Store, i: usize, text: &str) -> Result<()> {
    let old_title = store.notes[i].title.clone();
    match store.save(i, text)? {
        SaveOutcome::Saved => {}
        SaveOutcome::Conflict(p) => bail!(
            "note changed on disk meanwhile; your text was saved as {}",
            p.display()
        ),
    }
    let new_title = store.notes[i].title.clone();
    if old_title != "Untitled" && link_key(&old_title) != link_key(&new_title) {
        let own = store.notes[i].path.clone();
        store.relink(&old_title, &new_title, &[own])?;
    }
    Ok(())
}
