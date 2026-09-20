//! Tag autocomplete popup in the editor.

use super::*;

impl App {
    pub(super) fn recompute_tag_popup(&mut self) {
        self.tag_popup = None;
        let Some(tab) = self.active_tab() else { return };
        if tab.preview || tab.editor.mode != EditorMode::Insert {
            return;
        }
        let cursor = tab.editor.cursor;
        let Some(row) = tab.editor.lines.get(RowIndex::new(cursor.row)) else {
            return;
        };
        let before: &[char] = &row[..cursor.col.min(row.len())];
        let Some(hash) = before.iter().rposition(|&c| c == '#') else {
            return;
        };
        if hash > 0 && !before[hash - 1].is_whitespace() {
            return;
        }
        let body: String = before[hash + 1..].iter().collect();
        if !body.chars().all(note::is_tag_char) {
            return;
        }
        let lower = body.to_lowercase();
        let mut candidates: Vec<String> = build_tag_tree(&self.store.notes, &HashSet::new())
            .rows
            .into_iter()
            .map(|r| r.path)
            .filter(|p| p.starts_with(&lower) && *p != lower)
            .collect();
        candidates.sort();
        candidates.truncate(TAG_POPUP_MAX);
        if candidates.is_empty() {
            return;
        }
        self.tag_popup = Some(TagPopup {
            candidates,
            sel: 0,
            typed: body.chars().count(),
        });
    }

    pub(super) fn accept_tag(&mut self, sel: Option<usize>) {
        let Some(popup) = self.tag_popup.take() else {
            return;
        };
        let sel = sel.unwrap_or(popup.sel);
        let Some(cand) = popup.candidates.get(sel) else {
            return;
        };
        let rest: Vec<char> = cand.chars().skip(popup.typed).collect();
        let Some(tab) = self.tabs.get_mut(self.active) else {
            return;
        };
        for c in rest {
            tab.editor.execute(InsertChar(c));
        }
        self.after_edit();
    }
}
