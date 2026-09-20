//! Tab lifecycle: open/close/activate, autosave, dirty tracking, preview.

use super::*;

impl App {
    /// Open `path` in a tab: existing tab → activate; else reuse the preview tab or push.
    pub(super) fn show_note(&mut self, path: PathBuf, pin: bool) {
        if let Some(i) = self.tabs.iter().position(|t| t.path == path) {
            self.active = i;
            if pin {
                self.tabs[i].pinned = true;
            }
        } else {
            let Some((note, in_trash)) = self.note_by_path(&path) else {
                return;
            };
            let text = note.text.clone();
            let tab = Tab {
                path,
                editor: self.new_editor(&text),
                pinned: pin,
                dirty: false,
                last_edit: Instant::now(),
                preview: in_trash,
                preview_scroll: 0,
                preview_max: 0,
            };
            if let Some(i) = self.tabs.iter().position(|t| !t.pinned) {
                self.save_tab(i);
                self.tabs[i] = tab;
                self.active = i;
            } else {
                self.tabs.push(tab);
                self.active = self.tabs.len() - 1;
            }
        }
        self.popup = None;
        self.refresh();
        self.persist_session();
    }

    /// Append a tab for `path` as-is (session restore): no preview replacement, no persist.
    pub(super) fn push_tab(&mut self, path: PathBuf, pinned: bool) -> bool {
        let Some((note, in_trash)) = self.note_by_path(&path) else {
            return false;
        };
        let text = note.text.clone();
        let editor = self.new_editor(&text);
        self.tabs.push(Tab {
            path,
            editor,
            pinned,
            dirty: false,
            last_edit: Instant::now(),
            preview: in_trash,
            preview_scroll: 0,
            preview_max: 0,
        });
        true
    }

    pub(super) fn close_tab(&mut self, i: usize) {
        if i >= self.tabs.len() {
            return;
        }
        self.save_tab(i);
        self.tabs.remove(i);
        if i <= self.active {
            self.active = self.active.saturating_sub(1);
        }
        self.active = self.active.min(self.tabs.len().saturating_sub(1));
        self.popup = None;
        self.refresh();
        self.persist_session();
    }

    pub(super) fn activate_tab(&mut self, i: usize) {
        if i < self.tabs.len() && i != self.active {
            self.active = i;
            self.popup = None;
            self.refresh();
            self.persist_session();
        }
    }

    pub(super) fn next_tab(&mut self) {
        if !self.tabs.is_empty() {
            self.activate_tab((self.active + 1) % self.tabs.len());
        }
    }

    pub(super) fn prev_tab(&mut self) {
        if !self.tabs.is_empty() {
            self.activate_tab((self.active + self.tabs.len() - 1) % self.tabs.len());
        }
    }
    pub(super) fn pin_active(&mut self) {
        if let Some(t) = self.active_tab_mut()
            && !t.pinned
        {
            t.pinned = true;
            self.persist_session();
        }
    }

    pub(super) fn save_tab(&mut self, i: usize) {
        let Some(tab) = self.tabs.get(i) else { return };
        if !tab.dirty {
            return;
        }
        let path = tab.path.clone();
        let text = tab.editor.lines.to_string();
        let Some(idx) = self.store.notes.iter().position(|n| n.path == path) else {
            // The note vanished from the store (folder moved/removed externally) while the
            // tab held edits: write them back rather than dropping them.
            match self.store.create_at(&path, &text) {
                Ok(_) => {
                    self.tabs[i].dirty = false;
                    self.set_status(
                        StatusKind::Reloaded,
                        format!("re-created {}", file_name(&path)),
                    );
                    self.refresh();
                }
                Err(e) => {
                    self.fail("save", e);
                    self.tabs[i].last_edit = Instant::now();
                }
            }
            return;
        };
        if text == self.store.notes[idx].text {
            self.tabs[i].dirty = false;
            return;
        }
        match self.store.save(idx, &text) {
            Ok(SaveOutcome::Saved) => {
                self.tabs[i].dirty = false;
                self.tabs[i].path = self.store.notes[idx].path.clone();
                self.refresh();
            }
            Ok(SaveOutcome::Conflict(p)) => {
                self.tabs[i].dirty = false;
                self.set_status(
                    StatusKind::Conflict,
                    format!("conflict — saved as {}", file_name(&p)),
                );
                let external = self
                    .note_by_path(&path)
                    .map(|(n, _)| n.text.clone())
                    .unwrap_or_default();
                self.tabs[i].editor = self.new_editor(&external);
                self.refresh();
            }
            Err(e) => {
                self.fail("save", e);
                self.tabs[i].last_edit = Instant::now();
            }
        }
    }

    pub(super) fn save_all_tabs(&mut self) {
        for i in 0..self.tabs.len() {
            self.save_tab(i);
        }
    }

    /// Called after every key forwarded to the active editor.
    pub(super) fn after_edit(&mut self) {
        let Some(i) = self.tabs.get(self.active).map(|_| self.active) else {
            return;
        };
        if !self.tabs[i].dirty {
            let text = self.tabs[i].editor.lines.to_string();
            let unchanged = self.tab_note(i).is_some_and(|n| n.text == text);
            if unchanged {
                return;
            }
        }
        let tab = &mut self.tabs[i];
        markdown::refresh(&mut tab.editor);
        tab.dirty = true;
        tab.last_edit = Instant::now();
        if !tab.pinned {
            tab.pinned = true;
            self.persist_session();
        }
    }

    pub(super) fn reload_tab(&mut self, i: usize, text: &str) {
        let editor = self.new_editor(text);
        let tab = &mut self.tabs[i];
        tab.editor = editor;
        tab.dirty = false;
    }

    pub(super) fn toggle_preview(&mut self) {
        if self.tab_in_trash(self.active) {
            return;
        }
        if let Some(t) = self.active_tab_mut() {
            t.preview = !t.preview;
            t.preview_scroll = 0;
        }
        self.popup = None;
    }

    pub(super) fn scroll_preview(&mut self, delta: i32) {
        if let Some(t) = self.active_tab_mut() {
            let max = t.preview_max as i32;
            t.preview_scroll = (t.preview_scroll as i32 + delta).clamp(0, max) as u16;
        }
    }
}
