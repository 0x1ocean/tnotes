//! Note actions: create, trash/undo, restore, purge.

use super::*;

impl App {
    pub(super) fn new_note(&mut self) {
        self.new_note_in(None, None);
    }

    /// Create a note in `dir` (default: current folder filter or first root),
    /// optionally pre-filled with `#tag` below the title.
    pub(super) fn new_note_in(&mut self, dir: Option<PathBuf>, tag: Option<String>) {
        let Some(first) = self.store.roots.first().map(|r| r.dir.clone()) else {
            self.open_first_run();
            return;
        };
        let dir = dir.unwrap_or_else(|| match &self.filter {
            Filter::Folder(d) => d.clone(),
            _ => first,
        });
        let template = self.cfg.editor.template.clone();
        match self.store.create(&dir, &template) {
            Ok(idx) => {
                let path = self.store.notes[idx].path.clone();
                let keep = match &self.filter {
                    Filter::Folder(d) => dir.starts_with(d),
                    Filter::Tag(t) => tag.as_deref() == Some(t.as_str()),
                    _ => false,
                };
                if !keep {
                    self.filter = Filter::All;
                }
                self.search_active = false;
                self.query = single_line("");
                self.refresh();
                self.show_note(path, true);
                if let Some(tag) = tag {
                    let text = format!("{}\n\n#{tag}\n", template.trim_end_matches('\n'));
                    if let Some(t) = self.active_tab_mut() {
                        t.editor = EditorState::new(Lines::from(text.as_str()));
                        t.editor.mode = EditorMode::Insert;
                    }
                    self.after_edit();
                }
                let col = template
                    .lines()
                    .next()
                    .map(|l| l.chars().count())
                    .unwrap_or(0);
                if let Some(t) = self.active_tab_mut() {
                    t.editor.cursor = Index2::new(0, col);
                }
                self.focus = Pane::Editor;
                self.overlay = Overlay::None;
            }
            Err(e) => self.fail("create", e),
        }
    }

    pub(super) fn trash_selected(&mut self) {
        let Some(&idx) = self.visible.get(self.list_sel) else {
            return;
        };
        if self.filter == Filter::Trash {
            let path = self.store.trash[idx].path.clone();
            self.overlay = Overlay::Confirm(ConfirmAction::Purge(path));
            return;
        }
        let path = self.store.notes[idx].path.clone();
        self.trash_path(path);
    }

    pub(super) fn trash_path(&mut self, mut path: PathBuf) {
        if let Some(i) = self.tabs.iter().position(|t| t.path == path) {
            self.save_tab(i);
            if self.tabs[i].dirty {
                return; // the save failed (status already set); keep the text where it is
            }
            // Saving may have renamed the file after a title change.
            path = self.tabs[i].path.clone();
        }
        let Some(idx) = self.store.notes.iter().position(|n| n.path == path) else {
            return;
        };
        match self.store.trash(idx) {
            Ok(()) => {
                let trash_path = self.store.trash[0].path.clone();
                if let Some(i) = self.tabs.iter().position(|t| t.path == path) {
                    self.tabs[i].dirty = false;
                    self.close_tab(i);
                }
                self.last_trash = Some((trash_path, path, Instant::now()));
                self.refresh();
                self.set_status_ttl(
                    StatusKind::Info,
                    "moved to trash · u undo".into(),
                    UNDO_TRASH_WINDOW,
                );
            }
            Err(e) => self.fail("trash", e),
        }
    }

    pub(super) fn undo_trash(&mut self) {
        let Some((trash_path, _, at)) = self.last_trash.take() else {
            return;
        };
        if at.elapsed() > UNDO_TRASH_WINDOW {
            return;
        }
        let Some(tidx) = self.store.trash.iter().position(|n| n.path == trash_path) else {
            return;
        };
        match self.store.restore(tidx) {
            Ok(()) => {
                self.refresh();
                self.set_status(StatusKind::Info, "restored".into());
            }
            Err(e) => self.fail("restore", e),
        }
    }

    pub(super) fn restore_selected(&mut self) {
        if self.filter != Filter::Trash {
            return;
        }
        let Some(&idx) = self.visible.get(self.list_sel) else {
            return;
        };
        let path = self.store.trash[idx].path.clone();
        match self.store.restore(idx) {
            Ok(()) => {
                if let Some(i) = self.tabs.iter().position(|t| t.path == path) {
                    self.tabs[i].dirty = false;
                    self.close_tab(i);
                }
                self.refresh();
                self.set_status(StatusKind::Info, "restored".into());
            }
            Err(e) => self.fail("restore", e),
        }
    }

    pub(super) fn purge(&mut self, path: PathBuf) {
        // Resolve by path at confirm time: the trash may have shifted meanwhile.
        let Some(tidx) = self.store.trash.iter().position(|n| n.path == path) else {
            return;
        };
        match self.store.purge(tidx) {
            Ok(()) => {
                if let Some(i) = self.tabs.iter().position(|t| t.path == path) {
                    self.tabs[i].dirty = false;
                    self.close_tab(i);
                }
                self.refresh();
                self.set_status(StatusKind::Info, "deleted permanently".into());
            }
            Err(e) => self.fail("delete", e),
        }
    }
}
