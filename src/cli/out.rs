//! Plain and `--json` output.

use std::path::PathBuf;
use std::time::SystemTime;

use anyhow::Result;

use crate::store::Store;

#[derive(serde::Serialize)]
pub(super) struct NoteOut {
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

fn note_out(store: &Store, i: usize, with_text: bool, backlinks: &[usize]) -> NoteOut {
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
        backlinks: backlinks
            .iter()
            .map(|&j| store.note_id(&store.notes[j]))
            .collect(),
        created: rfc3339(n.created),
        modified: rfc3339(n.modified),
        preview: n.preview(),
        text: with_text.then(|| n.text.clone()),
    }
}

pub(super) fn print_json<T: serde::Serialize>(v: &T) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(v)?);
    Ok(())
}

/// One note after a single-note command: the full `NoteOut` (with text) under `--json`,
/// else `plain(id)` on one line.
pub(super) fn emit(
    store: &Store,
    i: usize,
    json: bool,
    plain: impl FnOnce(&str) -> String,
) -> Result<()> {
    if json {
        return print_json(&note_out(store, i, true, &store.backlinks(i)));
    }
    println!("{}", plain(&store.note_id(&store.notes[i])));
    Ok(())
}

/// `ls`/`search` output: a JSON array or `id \t title \t #tags` lines.
pub(super) fn print_list(store: &Store, idx: &[usize], json: bool) -> Result<()> {
    if json {
        let all = store.backlinks_all();
        let out: Vec<NoteOut> = idx
            .iter()
            .map(|&i| note_out(store, i, false, &all[i]))
            .collect();
        return print_json(&out);
    }
    for &i in idx {
        let n = &store.notes[i];
        let tags: Vec<String> = n.tags.iter().map(|t| format!("#{t}")).collect();
        println!("{}\t{}\t{}", store.note_id(n), n.title, tags.join(" "));
    }
    Ok(())
}
