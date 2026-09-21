//! Reacting to external filesystem changes and reloading the store.

use super::*;

/// `dir/stem (conflict <stamp>).md` next to `path`.
fn conflict_path(path: &Path) -> PathBuf {
    let stamp = chrono::Local::now().format("%Y-%m-%dT%H-%M-%S");
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("note");
    path.with_file_name(format!("{stem} (conflict {stamp}).md"))
}

impl App {
    /// Handle one batch of watcher events. Paths that still exist go first so a rename's new
    /// file is in the store before the old path is handled; `batch_added` remembers what
    /// appeared in this batch so `renamed_to` matches only genuinely new files.
    pub(super) fn on_external_batch(&mut self, mut paths: Vec<PathBuf>) {
        paths.sort_by_key(|p| !p.exists());
        self.batch_added.clear();
        for p in paths {
            self.on_external(p);
        }
    }

    pub(super) fn on_external(&mut self, path: PathBuf) {
        let tab_i = self.tabs.iter().position(|t| t.path == path);
        let dirty = tab_i.is_some_and(|i| self.tabs[i].dirty);
        if dirty && path.exists() {
            return; // the pending save will detect the conflict
        }
        let outcome = self.store.reload_path(&path);
        match outcome {
            Reload::Ignored => return,
            Reload::Rescan => {
                if let Err(e) = self.store.rescan() {
                    self.fail("rescan", e);
                }
                let mut i = 0;
                while i < self.tabs.len() {
                    match self.tab_note(i) {
                        Some(n) => {
                            let text = n.text.clone();
                            if !self.tabs[i].dirty && self.tabs[i].editor.lines.to_string() != text
                            {
                                self.reload_tab(i, &text);
                            }
                            i += 1;
                        }
                        None if self.tabs[i].dirty => i += 1,
                        None => self.close_tab(i),
                    }
                }
            }
            Reload::Updated => {
                if let Some(i) = tab_i {
                    let text = self.tab_note(i).map(|n| n.text.clone()).unwrap_or_default();
                    self.reload_tab(i, &text);
                }
                self.set_status(
                    StatusKind::Reloaded,
                    format!("reloaded {}", file_name(&path)),
                );
            }
            Reload::Removed => {
                if let Some(i) = tab_i {
                    let new = self.renamed_to(&path);
                    match (self.tabs[i].dirty, new) {
                        (false, Some(new)) => {
                            // A rename (e.g. `tnotes write` changing the title): follow it.
                            let text = self
                                .note_by_path(&new)
                                .map(|(n, _)| n.text.clone())
                                .unwrap_or_default();
                            self.tabs[i].path = new.clone();
                            self.reload_tab(i, &text);
                            self.set_status(
                                StatusKind::Reloaded,
                                format!("renamed to {}", file_name(&new)),
                            );
                            self.persist_session();
                        }
                        (true, Some(new)) => {
                            // The rename wins; the unsaved text becomes a conflict copy.
                            let text = self.tabs[i].editor.lines.to_string();
                            let copy = conflict_path(&new);
                            match self.store.create_at(&copy, &text) {
                                Ok(_) => {
                                    let disk = self
                                        .note_by_path(&new)
                                        .map(|(n, _)| n.text.clone())
                                        .unwrap_or_default();
                                    self.tabs[i].path = new.clone();
                                    self.reload_tab(i, &disk);
                                    self.set_status(
                                        StatusKind::Conflict,
                                        format!("conflict — saved as {}", file_name(&copy)),
                                    );
                                    self.persist_session();
                                }
                                Err(e) => self.fail("save conflict copy", e),
                            }
                        }
                        (true, None) => {
                            let text = self.tabs[i].editor.lines.to_string();
                            match self.store.create_at(&path, &text) {
                                Ok(_) => {
                                    self.tabs[i].dirty = false;
                                    self.set_status(
                                        StatusKind::Reloaded,
                                        format!("re-created {}", file_name(&path)),
                                    );
                                }
                                Err(e) => self.fail("re-create", e),
                            }
                        }
                        (false, None) => self.close_tab(i),
                    }
                }
            }
            Reload::Added => self.batch_added.push(path.clone()),
        }
        self.refresh();
    }

    /// The note `old` was renamed to: a file that appeared in the same watcher batch, in the
    /// same folder, and that no tab shows yet.
    fn renamed_to(&self, old: &Path) -> Option<PathBuf> {
        self.batch_added
            .iter()
            .filter(|p| p.as_path() != old && p.parent() == old.parent())
            .filter(|p| self.note_by_path(p).is_some())
            .find(|p| !self.tabs.iter().any(|t| &t.path == *p))
            .cloned()
    }

    pub(super) fn reload_store(&mut self) {
        self.save_all_tabs();
        if self.tabs.iter().any(|t| t.dirty) {
            return; // a save failed; reloading would drop that text
        }
        match Store::load(&self.cfg.roots) {
            Ok(s) => self.store = s,
            Err(e) => {
                self.fail("reload", e);
                return;
            }
        }
        match watch::spawn(&self.cfg.roots) {
            Ok((watchers, rx)) => {
                self.watchers = watchers;
                self.watch_rx = rx;
            }
            Err(e) => self.fail("watch", e),
        }
        let known: Vec<bool> = self
            .tabs
            .iter()
            .map(|t| self.note_by_path(&t.path).is_some())
            .collect();
        let active_path = self.active_tab().map(|t| t.path.clone());
        let mut keep = known.into_iter();
        self.tabs.retain(|_| keep.next().unwrap_or(false));
        self.active = active_path
            .and_then(|p| self.tabs.iter().position(|t| t.path == p))
            .unwrap_or(self.active.min(self.tabs.len().saturating_sub(1)));
        self.refresh();
        self.persist_session();
    }
}
