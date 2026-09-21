//! Mouse routing and hit-testing against `Hits`.

use ratatui::layout::Rect;

use super::*;

/// What a left click landed on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Target {
    PopupRow(usize),
    Tab(usize),
    TabClose,
    TabNew,
    PanelSwitch,
    StatusKeys,
    StatusHelp,
    StatusSort,
    StatusFilter,
    StatusBacklinks,
    SettingsLink,
    TreeHeader,
    Tree,
    Search,
    ListRow(usize),
    Editor,
}

impl App {
    pub(super) fn on_mouse(&mut self, m: MouseEvent) {
        let pos = Position::new(m.column, m.row);
        let down = matches!(m.kind, MouseEventKind::Down(MouseButton::Left));
        match self.layer() {
            Layer::Overlay => return self.overlay_mouse(m, pos, down),
            Layer::Settings => {
                if down {
                    self.settings_click(pos);
                }
                return;
            }
            // Popup rows are hit-tested as `Target::PopupRow` in the main path.
            Layer::Popup | Layer::Main => {}
        }

        if matches!(m.kind, MouseEventKind::Down(MouseButton::Right)) {
            self.open_menu(pos);
            return;
        }

        let in_editor = self.hits.editor.contains(pos);
        let preview = self.active_tab().is_some_and(|t| t.preview);
        match m.kind {
            MouseEventKind::ScrollDown | MouseEventKind::ScrollUp => {
                let dir: isize = if m.kind == MouseEventKind::ScrollDown {
                    1
                } else {
                    -1
                };
                if self.hits.list.contains(pos) {
                    self.move_list(dir * WHEEL_STEP as isize);
                } else if self.hits.tree.contains(pos) {
                    self.move_tree(dir * WHEEL_STEP as isize);
                } else if in_editor {
                    if preview {
                        self.scroll_preview(dir as i32 * WHEEL_STEP as i32);
                    } else if let Some(t) = self.tabs.get_mut(self.active) {
                        self.editor_handler.on_event(Event::Mouse(m), &mut t.editor);
                    }
                }
            }
            MouseEventKind::Down(MouseButton::Left) => match self.click_target(pos) {
                Some(Target::PopupRow(i)) => self.accept_popup(Some(i)),
                Some(Target::Tab(i)) => self.activate_tab(i),
                Some(Target::TabClose) => self.close_tab(self.active),
                Some(Target::TabNew) => self.new_note(),
                Some(Target::PanelSwitch) => self.toggle_sidebar(),
                Some(Target::StatusKeys) => self.toggle_editor_keys(),
                Some(Target::StatusHelp) => self.open_help(),
                Some(Target::StatusSort) => self.cycle_sort(),
                Some(Target::StatusFilter) => self.reveal_filter(),
                Some(Target::StatusBacklinks) => {
                    if let Some(p) = self.active_tab().map(|t| t.path.clone()) {
                        self.open_backlinks(p);
                    }
                }
                Some(Target::SettingsLink) => self.open_settings(),
                Some(Target::TreeHeader) => {
                    self.tree_folded = !self.tree_folded;
                    self.persist_session();
                }
                Some(Target::Tree) => self.tree_click(pos),
                Some(Target::Search) => {
                    self.search_active = true;
                    self.focus = Pane::List;
                }
                Some(Target::ListRow(i)) => {
                    self.search_active = false;
                    self.focus = Pane::List;
                    let row = self.list_offset + i;
                    self.select_list(row);
                    // Double-click pins the tab and focuses the editor.
                    let now = Instant::now();
                    let double = self
                        .last_click
                        .is_some_and(|(r, t)| r == row && now.duration_since(t) < DOUBLE_CLICK);
                    self.last_click = Some((row, now));
                    if double || (self.layout == Layout::Narrow && !self.tabs.is_empty()) {
                        if double {
                            self.pin_active();
                        }
                        self.focus = Pane::Editor;
                    }
                }
                Some(Target::Editor) => {
                    self.focus = Pane::Editor;
                    self.search_active = false;
                    self.popup = None;
                    if !preview && self.active < self.tabs.len() {
                        self.editor_mouse(m);
                        let t = &mut self.tabs[self.active];
                        let cur = t.editor.cursor;
                        let line = t.editor.lines.get(RowIndex::new(cur.row)).cloned();
                        // Clicking inside `[ ]` / `[x]` toggles the task.
                        let on_box = line
                            .as_deref()
                            .and_then(markdown::list_line)
                            .and_then(|it| it.checkbox)
                            .is_some_and(|b| (b..b + 3).contains(&cur.col));
                        // Ctrl+click: `[[link]]` opens the note, `#tag` filters the list.
                        let ctrl = m.modifiers.contains(KeyModifiers::CONTROL);
                        let link = ctrl
                            .then(|| line.as_deref().and_then(|l| markdown::link_at(l, cur.col)))
                            .flatten();
                        let tag = ctrl
                            .then(|| line.as_deref().and_then(|l| markdown::tag_at(l, cur.col)))
                            .flatten();
                        if on_box {
                            self.toggle_checkbox();
                        }
                        if let Some(t) = link {
                            let from = self.tabs[self.active].path.clone();
                            self.open_link(&t, &from);
                        } else if let Some(tag) = tag {
                            self.filter_by_tag(&tag);
                        }
                    }
                }
                None => {}
            },
            MouseEventKind::Drag(MouseButton::Left) | MouseEventKind::Up(MouseButton::Left)
                if self.focus == Pane::Editor && !preview && !self.tabs.is_empty() =>
            {
                self.editor_mouse(m);
            }
            _ => {}
        }
    }

    /// Forward a mouse event to the active editor. edtui enters `Visual` on drag (re-entering
    /// it would restart the selection, so it stays until the button is released) and falls
    /// back to `Normal` on the next click. The emacs keymap has no bindings in either mode,
    /// so under emacs keys the editor is put back into `Insert` on click and on release; the
    /// selection itself is mode-independent and, like in a terminal, is copied on release.
    fn editor_mouse(&mut self, m: MouseEvent) {
        let keys = self.keys;
        let Some(t) = self.tabs.get_mut(self.active) else {
            return;
        };
        self.editor_handler.on_event(Event::Mouse(m), &mut t.editor);
        let mut copied = false;
        match m.kind {
            MouseEventKind::Up(MouseButton::Left) => {
                if let Some(sel) = t.editor.selection.clone() {
                    t.editor.execute(CopySelection);
                    t.editor.selection = Some(sel);
                    copied = true;
                }
                if keys == EditorKeys::Emacs {
                    t.editor.mode = EditorMode::Insert;
                }
            }
            MouseEventKind::Down(MouseButton::Left) if keys == EditorKeys::Emacs => {
                t.editor.mode = EditorMode::Insert;
            }
            _ => {}
        }
        if copied {
            self.set_status(StatusKind::Info, "copied to clipboard".into());
        }
    }

    fn overlay_mouse(&mut self, m: MouseEvent, pos: Position, down: bool) {
        match self.overlay.clone() {
            Overlay::Confirm(action) => {
                if down {
                    // Row 0 is `yes`, row 1 is `no`; anything else cancels.
                    let hit = self.hits.overlay_rows.iter().position(|r| r.contains(pos));
                    self.overlay = Overlay::None;
                    if hit == Some(0) {
                        self.run_confirm(action);
                    }
                }
            }
            Overlay::Prompt(_) => {
                if down && !self.hits.overlay.contains(pos) {
                    self.overlay = Overlay::None;
                }
            }
            Overlay::Picker(note) => {
                if !down {
                    return;
                }
                if let Some(i) = self.hits.overlay_rows.iter().position(|r| r.contains(pos)) {
                    if self.picker_sel == i {
                        self.submit_picker(note);
                    } else {
                        self.picker_sel = i;
                    }
                } else if !self.hits.overlay.contains(pos) {
                    self.overlay = Overlay::None;
                }
            }
            Overlay::Backlinks(_) => {
                if !down {
                    return;
                }
                if let Some(i) = self.hits.overlay_rows.iter().position(|r| r.contains(pos)) {
                    self.open_backlink(i);
                } else if !self.hits.overlay.contains(pos) {
                    self.overlay = Overlay::None;
                }
            }
            Overlay::Menu(menu) => match m.kind {
                MouseEventKind::Down(MouseButton::Left) => {
                    if let Some(i) = self.hits.overlay_rows.iter().position(|r| r.contains(pos)) {
                        self.run_menu(menu.items[i].action.clone());
                    } else {
                        self.overlay = Overlay::None;
                    }
                }
                MouseEventKind::Down(MouseButton::Right) => {
                    self.overlay = Overlay::None;
                    self.open_menu(pos);
                }
                MouseEventKind::Moved => {
                    if let Some(i) = self.hits.overlay_rows.iter().position(|r| r.contains(pos)) {
                        let mut menu = menu;
                        menu.sel = i;
                        self.overlay = Overlay::Menu(menu);
                    }
                }
                _ => {}
            },
            Overlay::Browse => self.browser_mouse(m, pos),
            Overlay::None => {}
        }
    }

    /// Resolve a left click against the rects of the last frame, top-most first.
    fn click_target(&self, pos: Position) -> Option<Target> {
        let h = &self.hits;
        let row = |rects: &[Rect]| rects.iter().position(|r| r.contains(pos));
        if let Some(i) = row(&h.popup_rows) {
            return Some(Target::PopupRow(i));
        }
        if let Some(&(i, _)) = h.tabs.iter().find(|(_, r)| r.contains(pos)) {
            return Some(Target::Tab(i));
        }
        let flat: [(Rect, Target); 11] = [
            (h.tab_close, Target::TabClose),
            (h.tab_new, Target::TabNew),
            (h.sidebar_toggle, Target::PanelSwitch),
            (h.status_keys, Target::StatusKeys),
            (h.status_help, Target::StatusHelp),
            (h.status_sort, Target::StatusSort),
            (h.status_filter, Target::StatusFilter),
            (h.status_backlinks, Target::StatusBacklinks),
            (h.settings_link, Target::SettingsLink),
            (h.tree_header, Target::TreeHeader),
            (h.search, Target::Search),
        ];
        if let Some((_, t)) = flat.into_iter().find(|(r, _)| r.contains(pos)) {
            return Some(t);
        }
        if h.tree.contains(pos) {
            return Some(Target::Tree);
        }
        if h.list.contains(pos) {
            return row(&h.list_rows).map(Target::ListRow);
        }
        if h.editor.contains(pos) && !self.tabs.is_empty() {
            return Some(Target::Editor);
        }
        None
    }

    /// Show the sidebar and put the tree cursor on the current filter.
    fn reveal_filter(&mut self) {
        if !self.sidebar {
            self.toggle_sidebar();
        }
        self.focus = Pane::Tree;
        if let Some(i) = self
            .tree_rows
            .iter()
            .position(|r| r.filter().as_ref() == Some(&self.filter))
        {
            self.tree_sel = i;
        }
    }
    pub(super) fn tree_click(&mut self, pos: Position) {
        let Some(i) = self.hits.tree_rows.iter().position(|r| r.contains(pos)) else {
            return;
        };
        let i = self.tree_offset + i;
        let Some(row) = self.tree_rows.get(i) else {
            return;
        };
        let glyph = match row {
            TreeRow::Folder(f) if f.has_children => Some(ui::tree_glyph_x(self.hits.tree, f.depth)),
            TreeRow::Tag(t) if t.has_children => Some(ui::tree_glyph_x(self.hits.tree, t.depth)),
            _ => None,
        };
        if !row.selectable() {
            return;
        }
        self.focus = Pane::Tree;
        self.search_active = false;
        if glyph == Some(pos.x) {
            self.tree_sel = i;
            self.toggle_collapse(i);
        } else {
            self.apply_tree(i, false);
        }
    }
}
