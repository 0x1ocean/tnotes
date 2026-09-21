//! Reacting to external filesystem changes and reloading the store.

use super::*;

impl App {
    pub(super) fn on_external(&mut self, path: PathBuf) {
        let tab_i = self.tabs.iter().position(|t| t.path == path);
        let dirty = tab_i.is_some_and(|i| self.tabs[i].dirty);
        if dirty && path.exists() {
            return; // the pending save will detect the conflict
        }
        let created = self.note_by_path(&path).map(|(n, _)| n.created);
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
                    if self.tabs[i].dirty {
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
                    } else if let Some(new) = created.and_then(|c| self.renamed_to(&path, c)) {
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
                    } else {
                        self.close_tab(i);
                    }
                }
            }
            Reload::Added => {}
        }
        self.refresh();
    }

    /// The note `old` was renamed to: a sibling no tab shows yet that either kept the birth
    /// time (plain `mv`) or was written within the last two seconds (`tnotes write`, which
    /// saves through a temp file and so gets a new inode).
    fn renamed_to(&self, old: &Path, created: SystemTime) -> Option<PathBuf> {
        let now = SystemTime::now();
        let fresh = |t: SystemTime| now.duration_since(t).is_ok_and(|d| d < RENAME_WINDOW);
        self.store
            .notes
            .iter()
            .filter(|n| n.path != old && n.path.parent() == old.parent())
            .filter(|n| n.created == created || fresh(n.modified))
            .filter(|n| !self.tabs.iter().any(|t| t.path == n.path))
            .map(|n| n.path.clone())
            .next()
    }

    pub(super) fn reload_store(&mut self) {
        self.save_all_tabs();
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
