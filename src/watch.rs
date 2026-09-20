use std::path::{Path, PathBuf};
use std::sync::mpsc::{Receiver, Sender, channel};

use anyhow::{Context, Result};
use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher};

use crate::store::TRASH_DIR;

/// True when no component of `path` below `root` is dot-prefixed, except a leading `.Trash`.
fn visible(root: &Path, path: &Path) -> bool {
    let Ok(rel) = path.strip_prefix(root) else {
        return false;
    };
    let mut comps = rel
        .components()
        .map(|c| c.as_os_str().to_string_lossy().into_owned());
    let mut first = comps.next();
    if first.as_deref() == Some(TRASH_DIR) {
        first = comps.next();
    }
    !first.into_iter().chain(comps).any(|c| c.starts_with('.'))
}

fn watcher_for(root: PathBuf, tx: Sender<PathBuf>) -> Result<RecommendedWatcher> {
    let dir = root.clone();
    let mut watcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
        let Ok(event) = res else { return };
        if !matches!(
            event.kind,
            EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)
        ) {
            return;
        }
        for path in event.paths {
            if visible(&root, &path) {
                let _ = tx.send(path);
            }
        }
    })
    .context("creating file watcher")?;
    watcher
        .watch(&dir, RecursiveMode::Recursive)
        .with_context(|| format!("watching {}", dir.display()))?;
    Ok(watcher)
}

/// Watch every root recursively; forwards paths of created/modified/removed visible entries.
pub fn spawn(roots: &[PathBuf]) -> Result<(Vec<RecommendedWatcher>, Receiver<PathBuf>)> {
    let (tx, rx) = channel::<PathBuf>();
    let watchers = roots
        .iter()
        .map(|r| watcher_for(r.clone(), tx.clone()))
        .collect::<Result<Vec<_>>>()?;
    Ok((watchers, rx))
}
