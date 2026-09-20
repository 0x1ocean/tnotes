//! Root folders: add (with path completion), remove, reorder.

use super::*;

impl App {
    pub(in crate::app) fn prompt_add_root(&mut self) {
        self.open_browser();
    }

    pub(in crate::app) fn confirm_remove_root(&mut self, i: usize) {
        if self.cfg.roots_fixed {
            return;
        }
        if self.cfg.roots.len() <= 1 {
            self.set_status(StatusKind::Info, "keep at least one folder".into());
        } else {
            self.overlay = Overlay::Confirm(ConfirmAction::RemoveRoot(i));
        }
    }

    pub(in crate::app) fn selected_root(&self) -> Option<usize> {
        let page = self.settings?;
        match self.settings_rows().get(page.row).map(|r| &r.id) {
            Some(SettingId::Root(i)) => Some(*i),
            _ => None,
        }
    }

    pub(in crate::app) fn move_root(&mut self, delta: isize) {
        let Some(i) = self.selected_root() else {
            return;
        };
        let j = i as isize + delta;
        if j < 0 || j as usize >= self.cfg.roots.len() {
            return;
        }
        self.cfg.roots.swap(i, j as usize);
        self.settings_page_mut().row = j as usize;
        self.reload_store();
    }

    pub(in crate::app) fn add_root(&mut self, text: &str) {
        self.overlay = Overlay::None;
        if text.is_empty() {
            return;
        }
        let dir = config::expand_tilde(text);
        match config::resolve_root(&dir) {
            Ok(root) => {
                let nested = self
                    .cfg
                    .roots
                    .iter()
                    .find(|r| root.starts_with(r) || r.starts_with(&root))
                    .cloned();
                if self.cfg.roots.contains(&root) {
                    self.set_status(StatusKind::Info, "already added".into());
                } else if let Some(other) = nested {
                    self.set_status(
                        StatusKind::Failed,
                        format!(
                            "overlaps with {} — folders can't nest",
                            config::contract_tilde(&other)
                        ),
                    );
                } else {
                    self.cfg.roots.push(root.clone());
                    if let Err(e) = config::save(&self.cfg) {
                        self.fail("config save", e);
                    }
                    self.reload_store();
                    self.browser.first_run = false;
                    self.select_root_in_tree(&root);
                    let n = self
                        .store
                        .notes
                        .iter()
                        .filter(|n| n.path.starts_with(&root))
                        .count();
                    self.set_status(
                        StatusKind::Info,
                        format!("added {} · {n} notes", config::contract_tilde(&root)),
                    );
                }
                if let (Some(page), Some(i)) = (
                    self.settings.as_mut(),
                    self.cfg.roots.iter().position(|r| *r == root),
                ) {
                    page.row = i;
                }
            }
            Err(e) => self.fail("add folder", e),
        }
    }

    /// Filter by `root` and put the tree cursor on it (visible once settings close).
    pub(in crate::app) fn select_root_in_tree(&mut self, root: &Path) {
        self.filter = Filter::Folder(root.to_path_buf());
        self.list_sel = 0;
        self.list_offset = 0;
        self.refresh();
        if let Some(i) = self
            .tree_rows
            .iter()
            .position(|r| r.filter().as_ref() == Some(&self.filter))
        {
            self.tree_sel = i;
        }
        self.persist_session();
    }
}
