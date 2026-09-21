//! Right-click context menu: building, running, key handling.

use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MenuAction {
    OpenPinned(PathBuf),
    MoveNote(PathBuf),
    TrashNote(PathBuf),
    RestoreNote(PathBuf),
    PurgeNote(PathBuf),
    CopyPath(PathBuf),
    NewNoteIn(Option<PathBuf>),
    NewNoteTag(String),
    NewFolder(PathBuf),
    RenameFolder(PathBuf),
    DeleteFolder(PathBuf),
    RemoveRoot(usize),
    ToggleFold(usize),
    CloseTab(usize),
    CloseOthers(usize),
    CloseAll,
    TogglePin(usize),
    Cut,
    Copy,
    Paste,
    TogglePreview,
    Backlinks,
    InsertTag,
    SortBy(SortMode),
    Settings,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MenuItem {
    pub label: String,
    /// Hotkey shown on the right; pressing it runs the item.
    pub key: Option<char>,
    pub action: MenuAction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Menu {
    pub items: Vec<MenuItem>,
    pub sel: usize,
    /// Screen position of the right-click.
    pub at: Position,
}

fn item(label: &str, key: Option<char>, action: MenuAction) -> MenuItem {
    MenuItem {
        label: label.to_string(),
        key,
        action,
    }
}

impl App {
    /// Build the right-click menu for whatever is under `pos`.
    pub(super) fn menu_at(&self, pos: Position) -> Vec<MenuItem> {
        use MenuAction as A;
        if let Some(&(i, _)) = self.hits.tabs.iter().find(|(_, r)| r.contains(pos)) {
            let tab = &self.tabs[i];
            return vec![
                item("close", Some('w'), A::CloseTab(i)),
                item("close others", None, A::CloseOthers(i)),
                item("close all", None, A::CloseAll),
                item(
                    if tab.pinned {
                        "unpin (make preview)"
                    } else {
                        "pin"
                    },
                    Some('p'),
                    A::TogglePin(i),
                ),
                item("copy path", Some('y'), A::CopyPath(tab.path.clone())),
            ];
        }
        if let Some(i) = self.hits.tree_rows.iter().position(|r| r.contains(pos)) {
            return match self.tree_rows.get(self.tree_offset + i) {
                Some(TreeRow::Folder(f)) if self.is_root(&f.dir) => vec![
                    item(
                        "new note here",
                        Some('n'),
                        A::NewNoteIn(Some(f.dir.clone())),
                    ),
                    item("new subfolder…", Some('N'), A::NewFolder(f.dir.clone())),
                    item("remove from tnotes…", Some('D'), A::RemoveRoot(f.root)),
                    item(
                        if f.has_children {
                            "collapse / expand"
                        } else {
                            ""
                        },
                        Some(' '),
                        A::ToggleFold(self.tree_offset + i),
                    ),
                ],
                Some(TreeRow::Folder(f)) => vec![
                    item(
                        "new note here",
                        Some('n'),
                        A::NewNoteIn(Some(f.dir.clone())),
                    ),
                    item("new subfolder…", Some('N'), A::NewFolder(f.dir.clone())),
                    item("rename…", Some('R'), A::RenameFolder(f.dir.clone())),
                    item("delete", Some('D'), A::DeleteFolder(f.dir.clone())),
                    item(
                        if f.has_children {
                            "collapse / expand"
                        } else {
                            ""
                        },
                        Some(' '),
                        A::ToggleFold(self.tree_offset + i),
                    ),
                ],
                Some(TreeRow::Tag(t)) => vec![
                    item(
                        &format!("new note with #{}", t.path),
                        Some('n'),
                        A::NewNoteTag(t.path.clone()),
                    ),
                    item(
                        if t.has_children {
                            "collapse / expand"
                        } else {
                            ""
                        },
                        Some(' '),
                        A::ToggleFold(self.tree_offset + i),
                    ),
                ],
                Some(TreeRow::All) => vec![item("new note", Some('n'), A::NewNoteIn(None))],
                _ => Vec::new(),
            };
        }
        if let Some(i) = self.hits.list_rows.iter().position(|r| r.contains(pos)) {
            let Some(&idx) = self.visible.get(self.list_offset + i) else {
                return Vec::new();
            };
            let path = self.active_notes()[idx].path.clone();
            return if self.filter == Filter::Trash {
                vec![
                    item("restore", Some('r'), A::RestoreNote(path.clone())),
                    item("delete permanently…", Some('d'), A::PurgeNote(path.clone())),
                    item("copy path", Some('y'), A::CopyPath(path)),
                ]
            } else {
                vec![
                    item("open in new tab", Some('o'), A::OpenPinned(path.clone())),
                    item("move to folder…", Some('m'), A::MoveNote(path.clone())),
                    item("trash", Some('d'), A::TrashNote(path.clone())),
                    item("copy path", Some('y'), A::CopyPath(path)),
                ]
            };
        }
        if self.hits.editor.contains(pos) && !self.tabs.is_empty() {
            let preview = self.active_tab().is_some_and(|t| t.preview);
            let mut items = Vec::new();
            if !preview {
                items.push(item("cut", Some('x'), A::Cut));
                items.push(item("copy", Some('c'), A::Copy));
                items.push(item("paste", Some('v'), A::Paste));
                items.push(item("insert tag…", Some('#'), A::InsertTag));
            }
            items.push(item(
                if preview { "edit" } else { "preview" },
                Some('l'),
                A::TogglePreview,
            ));
            items.push(item("backlinks", Some('b'), A::Backlinks));
            return items;
        }
        // Empty space: list body, tab bar, status bar.
        let mut items = vec![item("new note", Some('n'), A::NewNoteIn(None))];
        for (mode, key) in [
            (SortMode::Modified, '1'),
            (SortMode::Title, '2'),
            (SortMode::Created, '3'),
        ] {
            let mark = if self.sort == mode { "▎" } else { " " };
            items.push(item(
                &format!("{mark}sort by {}", mode.name()),
                Some(key),
                A::SortBy(mode),
            ));
        }
        items.push(item("settings", Some(','), A::Settings));
        items
    }

    pub(super) fn open_menu(&mut self, pos: Position) {
        let items: Vec<MenuItem> = self
            .menu_at(pos)
            .into_iter()
            .filter(|i| !i.label.is_empty())
            .collect();
        if items.is_empty() {
            self.overlay = Overlay::None;
            return;
        }
        self.popup = None;
        self.overlay = Overlay::Menu(Menu {
            items,
            sel: 0,
            at: pos,
        });
    }

    pub(super) fn run_menu(&mut self, action: MenuAction) {
        use MenuAction as A;
        self.overlay = Overlay::None;
        match action {
            A::OpenPinned(p) => {
                self.show_note(p, true);
                self.focus = Pane::Editor;
            }
            A::MoveNote(p) => self.open_picker_for(p),
            A::TrashNote(p) => self.trash_path(p),
            A::RestoreNote(p) => {
                if let Some(t) = self.store.trash.iter().position(|n| n.path == p) {
                    match self.store.restore(t) {
                        Ok(()) => {
                            if let Some(i) = self.tabs.iter().position(|t| t.path == p) {
                                self.tabs[i].dirty = false;
                                self.close_tab(i);
                            }
                            self.refresh();
                            self.set_status(StatusKind::Info, "restored".into());
                        }
                        Err(e) => self.fail("restore", e),
                    }
                }
            }
            A::PurgeNote(p) => self.overlay = Overlay::Confirm(ConfirmAction::Purge(p)),
            A::CopyPath(p) => self.copy_to_clipboard(&p.display().to_string()),
            A::NewNoteIn(d) => self.new_note_in(d, None),
            A::NewNoteTag(t) => self.new_note_in(None, Some(t)),
            A::NewFolder(d) => self.prompt_new_folder(d),
            A::RenameFolder(d) => self.prompt_rename_folder(d),
            A::DeleteFolder(d) => self.confirm_delete_folder(d),
            A::RemoveRoot(i) => {
                if self.cfg.roots_fixed {
                    self.set_status(StatusKind::Info, "folders are fixed by --dir".into());
                } else if self.cfg.roots.len() <= 1 {
                    self.set_status(StatusKind::Info, "keep at least one folder".into());
                } else {
                    self.overlay = Overlay::Confirm(ConfirmAction::RemoveRoot(i));
                }
            }
            A::ToggleFold(i) => self.toggle_collapse(i),
            A::CloseTab(i) => self.close_tab(i),
            A::CloseOthers(i) => {
                let mut j = 0;
                while j < self.tabs.len() {
                    if j == i {
                        j += 1;
                    } else {
                        self.close_tab(j);
                        if j < i {
                            // `i` shifted left after removing an earlier tab.
                            return self.run_menu(A::CloseOthers(i - 1));
                        }
                    }
                }
            }
            A::CloseAll => {
                while !self.tabs.is_empty() {
                    self.close_tab(0);
                }
                if self.sidebar {
                    self.focus = Pane::List;
                }
            }
            A::TogglePin(i) => {
                // Only one preview tab may exist: unpinning pins any other preview tab.
                let make_preview = self.tabs.get(i).is_some_and(|t| t.pinned);
                if make_preview {
                    for t in &mut self.tabs {
                        t.pinned = true;
                    }
                }
                if let Some(t) = self.tabs.get_mut(i) {
                    t.pinned = !make_preview;
                }
                self.persist_session();
            }
            A::Cut | A::Copy | A::Paste => {
                self.focus = Pane::Editor;
                let keys = self.keys;
                let Some(tab) = self.tabs.get_mut(self.active) else {
                    return;
                };
                let has_sel = tab.editor.selection.is_some();
                let msg = match action {
                    A::Copy if has_sel => {
                        tab.editor.execute(CopySelection);
                        "copied to clipboard"
                    }
                    A::Copy => {
                        tab.editor.execute(CopyLine);
                        "line copied to clipboard"
                    }
                    // `DeleteSelection` copies the removed text itself.
                    A::Cut if has_sel => {
                        tab.editor.execute(DeleteSelection);
                        "cut to clipboard"
                    }
                    A::Cut => {
                        tab.editor.execute(CopyLine);
                        tab.editor.execute(DeleteLine(1));
                        "line cut to clipboard"
                    }
                    _ => {
                        tab.editor.execute(Paste);
                        ""
                    }
                };
                if keys == EditorKeys::Emacs {
                    tab.editor.mode = EditorMode::Insert;
                }
                self.after_edit();
                if !msg.is_empty() {
                    self.set_status(StatusKind::Info, msg.into());
                }
            }
            A::TogglePreview => {
                self.focus = Pane::Editor;
                self.toggle_preview();
            }
            A::Backlinks => {
                if let Some(p) = self.active_tab().map(|t| t.path.clone()) {
                    self.open_backlinks(p);
                }
            }
            A::InsertTag => {
                self.focus = Pane::Editor;
                if let Some(tab) = self.tabs.get_mut(self.active) {
                    tab.editor.mode = EditorMode::Insert;
                    tab.editor.execute(InsertChar('#'));
                }
                self.after_edit();
                self.recompute_popup();
            }
            A::SortBy(m) => {
                self.sort = m;
                self.refresh();
                self.persist_session();
            }
            A::Settings => self.open_settings(),
        }
    }

    /// OSC 52: hand the text to the terminal's clipboard (works through tmux/ssh).
    pub(super) fn copy_to_clipboard(&mut self, text: &str) {
        use std::io::Write;
        let encoded = base64(text.as_bytes());
        let mut out = std::io::stdout();
        let res = out
            .write_all(format!("\x1b]52;c;{encoded}\x07").as_bytes())
            .and_then(|()| out.flush());
        match res {
            Ok(()) => self.set_status(StatusKind::Info, "path copied".into()),
            Err(e) => self.fail("copy", e),
        }
    }

    pub(super) fn menu_key(&mut self, k: KeyEvent, mut menu: Menu) {
        let n = menu.items.len();
        match k.code {
            KeyCode::Esc => self.overlay = Overlay::None,
            KeyCode::Enter => self.run_menu(menu.items[menu.sel].action.clone()),
            KeyCode::Char('j') | KeyCode::Down => {
                menu.sel = (menu.sel + 1) % n;
                self.overlay = Overlay::Menu(menu);
            }
            KeyCode::Char('k') | KeyCode::Up => {
                menu.sel = (menu.sel + n - 1) % n;
                self.overlay = Overlay::Menu(menu);
            }
            KeyCode::Char(c) => {
                if let Some(it) = menu.items.iter().find(|i| i.key == Some(c)) {
                    self.run_menu(it.action.clone());
                }
            }
            _ => {}
        }
    }
}
