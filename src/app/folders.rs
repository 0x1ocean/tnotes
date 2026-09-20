//! Folder prompts and confirmations.

use super::*;

impl App {
    pub(super) fn prompt_new_folder(&mut self, dir: PathBuf) {
        self.prompt = single_line("");
        self.prompt_candidates.clear();
        self.overlay = Overlay::Prompt(PromptKind::NewFolder(dir));
    }

    pub(super) fn prompt_rename_folder(&mut self, dir: PathBuf) {
        if self.is_root(&dir) {
            self.set_status(
                StatusKind::Info,
                "root folders are managed in settings".into(),
            );
            return;
        }
        self.prompt = single_line(&file_name(&dir));
        self.prompt_candidates.clear();
        self.overlay = Overlay::Prompt(PromptKind::RenameFolder(dir));
    }

    pub(super) fn confirm_delete_folder(&mut self, dir: PathBuf) {
        if self.is_root(&dir) {
            self.set_status(
                StatusKind::Info,
                "root folders are managed in settings".into(),
            );
            return;
        }
        self.overlay = Overlay::Confirm(ConfirmAction::DeleteFolder(dir));
    }

    pub(super) fn submit_prompt(&mut self, kind: PromptKind) {
        let text = self.prompt.lines.to_string().trim().to_string();
        self.overlay = Overlay::None;
        match kind {
            PromptKind::NewFolder(parent) => match self.store.create_folder(&parent, &text) {
                Ok(dir) => {
                    self.collapsed.remove(&dir_key(&parent));
                    self.refresh();
                    self.set_status(
                        StatusKind::Info,
                        format!("created {}", self.store.folder_label(&dir)),
                    );
                }
                Err(e) => self.fail("new folder", e),
            },
            PromptKind::RenameFolder(dir) => match self.store.rename_folder(&dir, &text) {
                Ok(new) => {
                    if let Some(TreeRow::Folder(f)) = self.tree_rows.get_mut(self.tree_sel)
                        && f.dir == dir
                    {
                        f.dir = new.clone();
                    }
                    for t in &mut self.tabs {
                        if let Ok(rest) = t.path.strip_prefix(&dir) {
                            t.path = new.join(rest);
                        }
                    }
                    if let Filter::Folder(f) = &self.filter
                        && let Ok(rest) = f.strip_prefix(&dir)
                    {
                        self.filter = Filter::Folder(new.join(rest));
                    }
                    self.refresh();
                    self.persist_session();
                }
                Err(e) => self.fail("rename", e),
            },
            PromptKind::NewRoot(parent) => self.add_new_root(&parent, &text),
            PromptKind::Template => self.set_template(&text),
        }
    }

    pub(super) fn run_confirm(&mut self, action: ConfirmAction) {
        match action {
            ConfirmAction::Purge(p) => self.purge(p),
            ConfirmAction::DeleteFolder(dir) => {
                for i in 0..self.tabs.len() {
                    if self.tabs[i].path.starts_with(&dir) {
                        self.save_tab(i);
                    }
                }
                match self.store.delete_folder(&dir) {
                    Ok(()) => {
                        let mut i = 0;
                        while i < self.tabs.len() {
                            if self.tabs[i].path.starts_with(&dir) {
                                self.tabs[i].dirty = false;
                                self.close_tab(i);
                            } else {
                                i += 1;
                            }
                        }
                        if matches!(&self.filter, Filter::Folder(f) if f.starts_with(&dir)) {
                            self.filter = Filter::All;
                        }
                        if let Some(TreeRow::Folder(f)) = self.tree_rows.get_mut(self.tree_sel)
                            && f.dir == dir
                        {
                            f.dir = dir.parent().map(Path::to_path_buf).unwrap_or_default();
                        }
                        self.refresh();
                        self.set_status(
                            StatusKind::Info,
                            format!("deleted folder {}", file_name(&dir)),
                        );
                    }
                    Err(e) => self.fail("delete folder", e),
                }
            }
            ConfirmAction::RemoveRoot(i) => {
                if i < self.cfg.roots.len() && self.cfg.roots.len() > 1 {
                    self.cfg.roots.remove(i);
                    if let Err(e) = config::save(&self.cfg) {
                        self.fail("config save", e);
                    }
                    self.reload_store();
                }
                if let Some(p) = &mut self.settings {
                    p.row = i.min(self.cfg.roots.len().saturating_sub(1));
                }
            }
        }
    }
}
