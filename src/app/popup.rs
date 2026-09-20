//! Completion popup in the editor: `#tag` paths and `[[note title` links.

use super::*;

impl App {
    pub(super) fn recompute_popup(&mut self) {
        self.popup = None;
        let Some(tab) = self.active_tab() else { return };
        if tab.preview || tab.editor.mode != EditorMode::Insert {
            return;
        }
        let cursor = tab.editor.cursor;
        let Some(row) = tab.editor.lines.get(RowIndex::new(cursor.row)) else {
            return;
        };
        let before: &[char] = &row[..cursor.col.min(row.len())];
        self.popup = self
            .tag_candidates(before)
            .or_else(|| self.link_candidates(before, &tab.path));
    }

    /// `#pre` before the cursor → tag paths starting with `pre`.
    fn tag_candidates(&self, before: &[char]) -> Option<Popup> {
        let hash = before.iter().rposition(|&c| c == '#')?;
        if hash > 0 && !before[hash - 1].is_whitespace() {
            return None;
        }
        let body: String = before[hash + 1..].iter().collect();
        if !body.chars().all(note::is_tag_char) {
            return None;
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
        (!candidates.is_empty()).then(|| Popup {
            kind: PopupKind::Tag,
            candidates,
            sel: 0,
            typed: body.chars().count(),
        })
    }

    /// `[[part` before the cursor → note titles containing `part` (prefix matches first).
    fn link_candidates(&self, before: &[char], own: &Path) -> Option<Popup> {
        let open = before.windows(2).rposition(|w| w == ['[', '['])?;
        let body = &before[open + 2..];
        if body.iter().any(|&c| c == '[' || c == ']') {
            return None;
        }
        let body: String = body.iter().collect();
        let key = note::link_key(&body);
        let mut seen: HashSet<String> = HashSet::new();
        let mut candidates: Vec<(bool, String, String)> = self
            .store
            .notes
            .iter()
            .filter(|n| n.path != own && n.title != "Untitled")
            .filter_map(|n| {
                let k = note::link_key(&n.title);
                (k.contains(&key) && seen.insert(k.clone()))
                    .then(|| (!k.starts_with(&key), k, n.title.clone()))
            })
            .collect();
        candidates.sort();
        let candidates: Vec<String> = candidates
            .into_iter()
            .take(TAG_POPUP_MAX)
            .map(|(_, _, t)| t)
            .collect();
        (!candidates.is_empty()).then(|| Popup {
            kind: PopupKind::Link,
            candidates,
            sel: 0,
            typed: body.chars().count(),
        })
    }

    pub(super) fn accept_popup(&mut self, sel: Option<usize>) {
        let Some(popup) = self.popup.take() else {
            return;
        };
        let sel = sel.unwrap_or(popup.sel);
        let Some(cand) = popup.candidates.get(sel) else {
            return;
        };
        let Some(tab) = self.tabs.get_mut(self.active) else {
            return;
        };
        match popup.kind {
            PopupKind::Tag => {
                for c in cand.chars().skip(popup.typed) {
                    tab.editor.execute(InsertChar(c));
                }
            }
            PopupKind::Link => {
                tab.editor.execute(DeleteChar(popup.typed));
                for c in cand.chars().chain("]]".chars()) {
                    tab.editor.execute(InsertChar(c));
                }
            }
        }
        self.after_edit();
    }
}
