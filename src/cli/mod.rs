//! Headless subcommands over `Store`; a running TUI sees the changes through its watcher.

mod edit;
mod lookup;
mod out;

use anyhow::{Result, bail};

use crate::config::Config;
use crate::note::link_key;
use crate::store::Store;

use edit::{appended, new_text, read_stdin, save};
use lookup::{find, find_folder, find_trashed, list};
use out::{emit, print_json, print_list};

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
        /// At most this many notes
        #[arg(long)]
        limit: Option<usize>,
    },
    /// Fuzzy-search titles and bodies (best match first)
    Search {
        query: String,
        #[arg(long)]
        tag: Option<String>,
        #[arg(long)]
        folder: Option<String>,
        #[arg(long)]
        limit: Option<usize>,
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
        /// Create even if a note with this title exists
        #[arg(long)]
        duplicate: bool,
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
    /// Move a note back from .Trash to its folder
    Restore {
        /// Trashed note id (`root/sub/stem`, with or without `.Trash/`), path, or unique stem
        note: String,
    },
}

pub fn run(cmd: Cmd, cfg: &Config, json: bool) -> Result<()> {
    if cfg.roots.is_empty() {
        bail!("no notes folder configured: run `tnotes` once or pass --dir");
    }
    let mut store = Store::load(&cfg.roots)?;
    match cmd {
        Cmd::Ls { tag, folder, limit } => {
            let idx = list(&store, "", tag.as_deref(), folder.as_deref(), limit)?;
            print_list(&store, &idx, json)
        }
        Cmd::Search {
            query,
            tag,
            folder,
            limit,
        } => {
            let idx = list(&store, &query, tag.as_deref(), folder.as_deref(), limit)?;
            print_list(&store, &idx, json)
        }
        Cmd::Cat { note } => {
            let i = find(&store, &note)?;
            if json {
                emit(&store, i, true, |_| String::new())
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
            duplicate,
        } => {
            let title = title
                .map(|t| t.trim().to_string())
                .filter(|t| !t.is_empty());
            if title.is_none() && !stdin {
                bail!("give a title or --stdin");
            }
            if !duplicate
                && let Some(t) = &title
                && let Some(j) = store.resolve_link(t)
                && link_key(&store.notes[j].title) == link_key(t)
            {
                bail!(
                    "\"{t}\" already exists: {} (use --duplicate to create anyway)",
                    store.note_id(&store.notes[j])
                );
            }
            let dir = match folder {
                Some(f) => find_folder(&store, &f)?,
                None => cfg.roots[0].clone(),
            };
            let body = if stdin { Some(read_stdin()?) } else { None };
            let text = new_text(title.as_deref(), &tags, body.as_deref());
            let i = store.create(&dir, &text)?;
            emit(&store, i, json, |id| id.to_string())
        }
        Cmd::Write { note, stdin: _ } => {
            let i = find(&store, &note)?;
            let text = read_stdin()?;
            if text.trim().is_empty() {
                bail!("empty text; use `trash` to remove a note");
            }
            save(&mut store, i, &text)?;
            emit(&store, i, json, |id| format!("wrote {id}"))
        }
        Cmd::Append { note, text, stdin } => {
            let i = find(&store, &note)?;
            let body = match text {
                Some(t) => t,
                None if stdin => read_stdin()?,
                None => bail!("give text or --stdin"),
            };
            let text = appended(&store.notes[i].text, &body);
            save(&mut store, i, &text)?;
            emit(&store, i, json, |id| format!("appended to {id}"))
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
        Cmd::Restore { note } => {
            let t = find_trashed(&store, &note)?;
            store.restore(t)?;
            emit(&store, 0, json, |id| format!("restored {id}"))
        }
    }
}
