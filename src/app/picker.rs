//! Move-note folder picker overlay.

use super::*;

impl App {
    pub(super) fn open_picker(&mut self) {
        if self.filter == Filter::Trash {
            return;
        }
        let Some(&idx) = self.visible.get(self.list_sel) else {
            return;
        };
        let path = self.store.notes[idx].path.clone();
        self.open_picker_for(path);
    }

    pub(super) fn open_picker_for(&mut self, path: PathBuf) {
        self.picker_query = single_line("");
        self.picker_sel = 0;
        self.overlay = Overlay::Picker(path);
        self.refilter_picker();
    }

    pub(super) fn refilter_picker(&mut self) {
        let Overlay::Picker(note) = &self.overlay else {
            return;
        };
        let parent = note.parent().map(Path::to_path_buf);
        let dirs: Vec<PathBuf> = self
            .store
            .roots
            .iter()
            .map(|r| r.dir.clone())
            .chain(self.store.folders.iter().cloned())
            .filter(|d| Some(d) != parent.as_ref())
            .collect();
        let query = self.picker_query.lines.to_string();
        let query = query.trim();
        if query.is_empty() {
            self.picker_rows = dirs;
        } else {
            let pattern = Pattern::parse(query, CaseMatching::Ignore, Normalization::Smart);
            let mut buf = Vec::new();
            let mut scored: Vec<(u32, PathBuf)> = dirs
                .into_iter()
                .filter_map(|d| {
                    let label = self.store.folder_label(&d);
                    pattern
                        .score(Utf32Str::new(&label, &mut buf), &mut self.matcher)
                        .map(|s| (s, d))
                })
                .collect();
            scored.sort_by_key(|(s, _)| std::cmp::Reverse(*s));
            self.picker_rows = scored.into_iter().map(|(_, d)| d).collect();
        }
        self.picker_sel = self
            .picker_sel
            .min(self.picker_rows.len().saturating_sub(1));
    }

    pub(super) fn submit_picker(&mut self, mut note: PathBuf) {
        let Some(dir) = self.picker_rows.get(self.picker_sel).cloned() else {
            return;
        };
        self.overlay = Overlay::None;
        if let Some(i) = self.tabs.iter().position(|t| t.path == note) {
            self.save_tab(i);
            // Saving may have renamed the file after a title change.
            note = self.tabs[i].path.clone();
        }
        let Some(idx) = self.store.notes.iter().position(|n| n.path == note) else {
            return;
        };
        match self.store.move_note(idx, &dir) {
            Ok(()) => {
                let new = self.store.notes[idx].path.clone();
                if let Some(t) = self.tabs.iter_mut().find(|t| t.path == note) {
                    t.path = new;
                }
                self.refresh();
                self.persist_session();
                self.set_status(
                    StatusKind::Info,
                    format!("moved to {}", self.store.folder_label(&dir)),
                );
            }
            Err(e) => self.fail("move", e),
        }
    }
}
