//! Resolving the note / folder identifiers accepted on the command line.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Result, bail};
use nucleo_matcher::Matcher;

use crate::index::{self, Filter};
use crate::note::Note;
use crate::store::{Store, TRASH_DIR};

fn stem(path: &Path) -> &str {
    path.file_stem().and_then(|s| s.to_str()).unwrap_or("")
}

/// Note index for a path, an id (`root/sub/stem`, `.md` tolerated) or a unique file stem.
pub(super) fn find(store: &Store, key: &str) -> Result<usize> {
    find_in(store, &store.notes, key)
}

/// Like `find`, in the trash; the `.Trash/` component of the id may be omitted.
pub(super) fn find_trashed(store: &Store, key: &str) -> Result<usize> {
    find_in(store, &store.trash, key)
}

fn find_in(store: &Store, notes: &[Note], key: &str) -> Result<usize> {
    if let Ok(p) = fs::canonicalize(key)
        && let Some(i) = notes.iter().position(|n| n.path == p)
    {
        return Ok(i);
    }
    let bare = key.trim_end_matches(".md");
    let trash_seg = format!("/{TRASH_DIR}/");
    if let Some(i) = notes.iter().position(|n| {
        let id = store.note_id(n);
        let plain = id.replacen(&trash_seg, "/", 1);
        id == key || id == bare || plain == key || plain == bare
    }) {
        return Ok(i);
    }
    let by_stem: Vec<usize> = (0..notes.len())
        .filter(|&i| stem(&notes[i].path) == bare)
        .collect();
    match by_stem.as_slice() {
        [i] => Ok(*i),
        [] => bail!("no note {key}"),
        many => {
            let ids: Vec<String> = many.iter().map(|&i| store.note_id(&notes[i])).collect();
            bail!("ambiguous: {}", ids.join(", "))
        }
    }
}

/// Folder path for an existing path or a `root` / `root/sub` label.
pub(super) fn find_folder(store: &Store, key: &str) -> Result<PathBuf> {
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

/// Visible notes for `ls`/`search`: tag and folder filters, fuzzy `query`, best first.
pub(super) fn list(
    store: &Store,
    query: &str,
    tag: Option<&str>,
    folder: Option<&str>,
    limit: Option<usize>,
) -> Result<Vec<usize>> {
    let filter = tag
        .map(|t| Filter::Tag(t.trim().trim_start_matches('#').to_lowercase()))
        .unwrap_or(Filter::All);
    let mut matcher = Matcher::new(nucleo_matcher::Config::DEFAULT);
    let mut idx = index::visible(&store.notes, &filter, query, &mut matcher);
    if let Some(f) = folder {
        let dir = find_folder(store, f)?;
        idx.retain(|&i| store.notes[i].path.starts_with(&dir));
    }
    if let Some(n) = limit {
        idx.truncate(n);
    }
    Ok(idx)
}
