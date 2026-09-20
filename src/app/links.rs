//! [[link]] navigation and rewriting links after a title change.

use super::*;

impl App {
    /// `alt+enter`: open the link under the cursor.
    pub(super) fn follow_link_at_cursor(&mut self) {
        let Some(tab) = self.active_tab() else { return };
        let link = (!tab.preview)
            .then(|| {
                let cur = tab.editor.cursor;
                tab.editor
                    .lines
                    .get(RowIndex::new(cur.row))
                    .and_then(|l| markdown::link_at(l, cur.col))
            })
            .flatten();
        match link {
            Some(target) => {
                let from = tab.path.clone();
                self.open_link(&target, &from);
            }
            None => self.set_status(StatusKind::Info, "no link under cursor".into()),
        }
    }

    /// Open the note a `[[target]]` resolves to; create `# target` in `from`'s folder when none does.
    pub(super) fn open_link(&mut self, target: &str, from: &Path) {
        if let Some(i) = self.store.resolve_link(target) {
            let path = self.store.notes[i].path.clone();
            self.show_note(path, true);
            self.focus = Pane::Editor;
            return;
        }
        let Some(dir) = from.parent() else { return };
        let title = target.trim();
        let text = format!("# {title}\n");
        match self.store.create(dir, &text) {
            Ok(i) => {
                let path = self.store.notes[i].path.clone();
                self.refresh();
                self.show_note(path, true);
                let col = text.lines().next().map_or(0, |l| l.chars().count());
                if let Some(t) = self.active_tab_mut() {
                    t.editor.cursor = Index2::new(0, col);
                }
                self.focus = Pane::Editor;
                self.set_status(StatusKind::Info, format!("created {title}"));
            }
            Err(e) => self.fail("create", e),
        }
    }

    /// After a save changed the title: rewrite links on disk and in open editors.
    pub(super) fn relink(&mut self, old: &str, new: &str, saved: &Path) {
        let skip: Vec<PathBuf> = std::iter::once(saved.to_path_buf())
            .chain(
                self.tabs
                    .iter()
                    .filter(|t| t.dirty && t.path != saved)
                    .map(|t| t.path.clone()),
            )
            .collect();
        let rewritten = match self.store.relink(old, new, &skip) {
            Ok(paths) => paths,
            Err(e) => {
                self.fail("update links", e);
                return;
            }
        };
        let mut n = rewritten.len();
        for path in &rewritten {
            if let Some(i) = self.tabs.iter().position(|t| &t.path == path)
                && let Some(text) = self.note_by_path(path).map(|(n, _)| n.text.clone())
            {
                self.reload_tab(i, &text);
            }
        }
        for tab in self.tabs.iter_mut().filter(|t| t.dirty && t.path != saved) {
            let before = tab.editor.lines.to_string();
            let after = note::replace_links(&before, old, new);
            if after == before {
                continue;
            }
            let cur = tab.editor.cursor;
            tab.editor.lines = Lines::from(after.as_str());
            let row = cur.row.min(tab.editor.lines.len().saturating_sub(1));
            let col = cur.col.min(tab.editor.lines.len_col(row).unwrap_or(0));
            tab.editor.cursor = Index2::new(row, col);
            markdown::refresh(&mut tab.editor);
            n += 1;
        }
        if n > 0 {
            self.set_status(StatusKind::Info, format!("updated links in {n} note(s)"));
        }
        self.refresh();
    }
}
