//! Keyboard routing by `App::layer`: overlay → settings → tag popup → global chords → pane.

use super::*;

impl App {
    pub(super) fn handle(&mut self, ev: Event) {
        match ev {
            Event::Key(k) if k.kind != KeyEventKind::Release => self.on_key(k),
            Event::Mouse(m) => self.on_mouse(m),
            Event::Paste(text) => match &self.overlay {
                Overlay::Prompt(_) => {
                    self.query_handler.on_paste_event(text, &mut self.prompt);
                }
                Overlay::Picker(_) => {
                    self.query_handler
                        .on_paste_event(text, &mut self.picker_query);
                    self.refilter_picker();
                }
                Overlay::None if self.focus == Pane::Editor => {
                    if let Some(t) = self.tabs.get_mut(self.active)
                        && !t.preview
                    {
                        self.editor_handler.on_paste_event(text, &mut t.editor);
                        self.after_edit();
                        self.recompute_popup();
                    }
                }
                Overlay::None if self.search_active => {
                    self.query_handler.on_paste_event(text, &mut self.query);
                    self.on_query_changed();
                }
                _ => {}
            },
            _ => {}
        }
    }

    pub(super) fn on_key(&mut self, k: KeyEvent) {
        match self.layer() {
            Layer::Overlay => self.overlay_key(k),
            Layer::Settings => self.settings_key(k),
            Layer::Popup => {
                if !self.popup_key(k) {
                    self.main_key(k);
                }
            }
            Layer::Main => self.main_key(k),
        }
    }

    fn overlay_key(&mut self, k: KeyEvent) {
        let ctrl = k.modifiers.contains(KeyModifiers::CONTROL);
        match self.overlay.clone() {
            Overlay::Confirm(action) => {
                self.overlay = Overlay::None;
                if k.code == KeyCode::Char('y') {
                    self.run_confirm(action);
                }
            }
            Overlay::Prompt(kind) => {
                match k.code {
                    KeyCode::Esc => {
                        // A folder prompt opened from the browser returns to it.
                        self.overlay = if matches!(kind, PromptKind::NewRoot(_)) {
                            Overlay::Browse
                        } else {
                            Overlay::None
                        };
                    }
                    KeyCode::Enter => self.submit_prompt(kind),
                    KeyCode::Tab => {}
                    _ => {
                        self.query_handler.on_event(Event::Key(k), &mut self.prompt);
                        self.prompt_candidates.clear();
                    }
                }
            }
            Overlay::Picker(note) => match (k.code, ctrl) {
                (KeyCode::Esc, _) => self.overlay = Overlay::None,
                (KeyCode::Enter, _) => self.submit_picker(note),
                (KeyCode::Down, _) | (KeyCode::Char('j'), true) | (KeyCode::Char('n'), true) => {
                    self.picker_sel =
                        (self.picker_sel + 1).min(self.picker_rows.len().saturating_sub(1));
                }
                (KeyCode::Up, _) | (KeyCode::Char('k'), true) | (KeyCode::Char('p'), true) => {
                    self.picker_sel = self.picker_sel.saturating_sub(1);
                }
                _ => {
                    self.query_handler
                        .on_event(Event::Key(k), &mut self.picker_query);
                    self.refilter_picker();
                }
            },
            Overlay::Backlinks(_) => match (k.code, ctrl) {
                (KeyCode::Esc, _) => self.overlay = Overlay::None,
                (KeyCode::Enter, _) => self.open_backlink(self.backlink_sel),
                (KeyCode::Down, _) | (KeyCode::Char('j'), _) | (KeyCode::Char('n'), true) => {
                    self.backlink_sel =
                        (self.backlink_sel + 1).min(self.backlink_rows.len().saturating_sub(1));
                }
                (KeyCode::Up, _) | (KeyCode::Char('k'), _) | (KeyCode::Char('p'), true) => {
                    self.backlink_sel = self.backlink_sel.saturating_sub(1);
                }
                _ => {}
            },
            Overlay::Menu(menu) => self.menu_key(k, menu),
            Overlay::Browse => self.browser_key(k),
            Overlay::None => {}
        }
    }

    /// Tag-completion popup keys; `true` when the key was consumed.
    fn popup_key(&mut self, k: KeyEvent) -> bool {
        let ctrl = k.modifiers.contains(KeyModifiers::CONTROL);
        let n = self.popup.as_ref().map(|p| p.candidates.len()).unwrap_or(0);
        match (k.code, ctrl) {
            (KeyCode::Esc, _) => {
                self.popup = None;
                true
            }
            (KeyCode::Down, _) | (KeyCode::Char('n'), true) => {
                if let Some(p) = &mut self.popup {
                    p.sel = (p.sel + 1) % n;
                }
                true
            }
            (KeyCode::Up, _) | (KeyCode::Char('p'), true) => {
                if let Some(p) = &mut self.popup {
                    p.sel = (p.sel + n - 1) % n;
                }
                true
            }
            (KeyCode::Tab, _) | (KeyCode::Enter, _) => {
                self.accept_popup(None);
                true
            }
            _ => false,
        }
    }

    /// Global chords, then the focused pane.
    fn main_key(&mut self, k: KeyEvent) {
        let in_editor = self.focus == Pane::Editor;
        if in_editor
            && matches!(k.code, KeyCode::Tab | KeyCode::BackTab)
            && self.indent_list(k.code == KeyCode::Tab)
        {
            return;
        }
        if let Some(action) = keymap::lookup(&k) {
            self.run_action(action);
            return;
        }
        match k.code {
            KeyCode::Char('?') if !in_editor && !self.search_active => self.open_help(),
            KeyCode::Char(',') if !in_editor && !self.search_active => self.open_settings(),
            KeyCode::Char(c @ '1'..='9') if !in_editor && !self.search_active => {
                self.activate_tab(c as usize - '1' as usize);
            }
            _ => match self.focus {
                Pane::Editor => self.editor_key(k),
                Pane::List if self.search_active => self.search_key(k),
                Pane::List => self.list_key(k),
                Pane::Tree => self.tree_key(k),
            },
        }
    }

    /// Execute a global `keymap` action.
    pub(super) fn run_action(&mut self, action: keymap::Action) {
        use keymap::Action as A;
        match action {
            A::Quit => self.quit = true,
            A::NewNote => self.new_note(),
            A::TogglePreview => self.toggle_preview(),
            A::CloseTab => self.close_tab(self.active),
            A::NextTab => self.next_tab(),
            A::PrevTab => self.prev_tab(),
            A::ToggleSidebar => self.toggle_sidebar(),
            A::Help => self.open_help(),
            A::Settings => self.open_settings(),
            A::NextPane => self.cycle_focus(true),
            A::PrevPane => self.cycle_focus(false),
            A::Doc => {}
        }
    }

    pub(super) fn editor_key(&mut self, k: KeyEvent) {
        let Some(tab) = self.tabs.get(self.active) else {
            if k.code == KeyCode::Esc {
                self.focus = Pane::List;
            }
            return;
        };
        if k.code == KeyCode::Enter && k.modifiers == KeyModifiers::ALT {
            self.follow_link_at_cursor();
            return;
        }
        if tab.preview {
            self.preview_key(k);
            return;
        }
        if k.code == KeyCode::Esc {
            let mode = tab.editor.mode;
            match self.keys {
                EditorKeys::Emacs if mode == EditorMode::Search => {
                    let cancel = KeyEvent::new(KeyCode::Char('g'), KeyModifiers::CONTROL);
                    let tab = &mut self.tabs[self.active];
                    self.editor_handler
                        .on_event(Event::Key(cancel), &mut tab.editor);
                }
                EditorKeys::Vim if mode != EditorMode::Normal => {
                    let tab = &mut self.tabs[self.active];
                    self.editor_handler.on_event(Event::Key(k), &mut tab.editor);
                }
                _ => self.focus = Pane::List,
            }
            return;
        }
        let insert = tab.editor.mode == EditorMode::Insert;
        let plain = k.modifiers.is_empty();
        if insert && plain && k.code == KeyCode::Enter && self.continue_list() {
            return;
        }
        if k.code == KeyCode::Char('x') && k.modifiers == KeyModifiers::ALT {
            self.toggle_checkbox();
            return;
        }
        let tab = &mut self.tabs[self.active];
        self.editor_handler.on_event(Event::Key(k), &mut tab.editor);
        self.after_edit();
        self.recompute_popup();
    }

    /// `Enter` inside a list item: continue the list (or leave it when the item is empty).
    /// Returns `false` when the line is not a list item so the key is forwarded as usual.
    fn continue_list(&mut self) -> bool {
        let Some(tab) = self.tabs.get_mut(self.active) else {
            return false;
        };
        let row = tab.editor.cursor.row;
        let col = tab.editor.cursor.col;
        let Some(line) = tab.editor.lines.get(RowIndex::new(row)).cloned() else {
            return false;
        };
        let Some(item) = markdown::list_line(&line) else {
            return false;
        };
        if col < item.text_start {
            return false;
        }
        if line.len() == item.text_start {
            // Empty item: drop the marker and stay on the (now blank) line.
            if let Some(l) = tab.editor.lines.get_mut(RowIndex::new(row)) {
                l.clear();
            }
            tab.editor.cursor.col = 0;
        } else {
            let marker = markdown::next_marker(&item);
            self.editor_handler
                .on_event(Event::Key(KeyEvent::from(KeyCode::Enter)), &mut tab.editor);
            for c in marker.chars() {
                tab.editor.execute(InsertChar(c));
            }
        }
        self.after_edit();
        true
    }

    /// `Tab` / `Shift+Tab` on a list item: nest / un-nest by two spaces.
    /// Returns `false` when the cursor line is not a list item.
    fn indent_list(&mut self, deeper: bool) -> bool {
        let Some(tab) = self.tabs.get_mut(self.active) else {
            return false;
        };
        if tab.preview || tab.editor.mode != EditorMode::Insert {
            return false;
        }
        let row = tab.editor.cursor.row;
        let Some(line) = tab.editor.lines.get_mut(RowIndex::new(row)) else {
            return false;
        };
        if markdown::list_line(line).is_none() {
            return false;
        }
        if deeper {
            line.insert(0, ' ');
            line.insert(0, ' ');
            tab.editor.cursor.col += 2;
        } else {
            let strip = line.iter().take(2).take_while(|c| **c == ' ').count();
            if strip == 0 {
                return true;
            }
            line.drain(..strip);
            tab.editor.cursor.col = tab.editor.cursor.col.saturating_sub(strip);
        }
        self.after_edit();
        true
    }

    /// Toggle `[ ]` ↔ `[x]` on the cursor line.
    pub(super) fn toggle_checkbox(&mut self) {
        let Some(tab) = self.tabs.get_mut(self.active) else {
            return;
        };
        let row = tab.editor.cursor.row;
        let Some(line) = tab.editor.lines.get_mut(RowIndex::new(row)) else {
            return;
        };
        let Some(item) = markdown::list_line(line) else {
            return;
        };
        let Some(b) = item.checkbox else { return };
        line[b + 1] = if item.done { ' ' } else { 'x' };
        self.after_edit();
    }

    pub(super) fn preview_key(&mut self, k: KeyEvent) {
        let g = std::mem::take(&mut self.pending_g);
        match k.code {
            KeyCode::Char('j') | KeyCode::Down => self.scroll_preview(1),
            KeyCode::Char('k') | KeyCode::Up => self.scroll_preview(-1),
            KeyCode::PageDown => self.scroll_preview(10),
            KeyCode::PageUp => self.scroll_preview(-10),
            KeyCode::Char('g') if g => self.scroll_preview(i32::MIN / 2),
            KeyCode::Char('g') => self.pending_g = true,
            KeyCode::Char('G') => self.scroll_preview(i32::MAX / 2),
            KeyCode::Esc => self.focus = Pane::List,
            _ => {}
        }
    }

    pub(super) fn search_key(&mut self, k: KeyEvent) {
        let ctrl = k.modifiers.contains(KeyModifiers::CONTROL);
        match (k.code, ctrl) {
            (KeyCode::Esc, _) => {
                self.query = single_line("");
                self.search_active = false;
                self.on_query_changed();
            }
            (KeyCode::Enter, _) => self.search_active = false,
            (KeyCode::Down, _) | (KeyCode::Char('j'), true) => self.move_list(1),
            (KeyCode::Up, _) | (KeyCode::Char('k'), true) => self.move_list(-1),
            _ => {
                self.query_handler.on_event(Event::Key(k), &mut self.query);
                self.on_query_changed();
            }
        }
    }

    pub(super) fn on_query_changed(&mut self) {
        self.list_sel = 0;
        self.refresh();
        self.list_sel = 0;
    }

    pub(super) fn list_key(&mut self, k: KeyEvent) {
        let g = std::mem::take(&mut self.pending_g);
        match k.code {
            KeyCode::Char('j') | KeyCode::Down => self.move_list(1),
            KeyCode::Char('k') | KeyCode::Up => self.move_list(-1),
            KeyCode::PageDown => self.move_list(10),
            KeyCode::PageUp => self.move_list(-10),
            KeyCode::Char('g') if g => self.select_list(0),
            KeyCode::Char('g') => self.pending_g = true,
            KeyCode::Char('G') => self.select_list(usize::MAX),
            KeyCode::Enter | KeyCode::Char('l') | KeyCode::Right => {
                if !self.visible.is_empty() {
                    self.select_list(self.list_sel);
                    self.pin_active();
                    self.focus = Pane::Editor;
                }
            }
            KeyCode::Char('h') | KeyCode::Left => self.focus = Pane::Tree,
            KeyCode::Char('/') => self.search_active = true,
            KeyCode::Char('d') => self.trash_selected(),
            KeyCode::Char('u') => self.undo_trash(),
            KeyCode::Char('r') => self.restore_selected(),
            KeyCode::Char('m') => self.open_picker(),
            KeyCode::Char('s') => self.cycle_sort(),
            KeyCode::Char('q') => self.quit = true,
            _ => {}
        }
    }

    pub(super) fn tree_key(&mut self, k: KeyEvent) {
        match k.code {
            KeyCode::Char('j') | KeyCode::Down => self.move_tree(1),
            KeyCode::Char('k') | KeyCode::Up => self.move_tree(-1),
            KeyCode::Enter | KeyCode::Char('l') | KeyCode::Right => {
                self.apply_tree(self.tree_sel, true)
            }
            KeyCode::Char(' ') => self.toggle_collapse(self.tree_sel),
            KeyCode::Char('N') => {
                if let Some(d) = self.tree_folder() {
                    self.prompt_new_folder(d);
                }
            }
            KeyCode::Char('R') => {
                if let Some(d) = self.tree_folder() {
                    self.prompt_rename_folder(d);
                }
            }
            KeyCode::Char('D') => {
                if let Some(d) = self.tree_folder() {
                    self.confirm_delete_folder(d);
                }
            }
            KeyCode::Char('h') | KeyCode::Left => self.focus = Pane::Editor,
            KeyCode::Esc => self.focus = Pane::List,
            KeyCode::Char('/') => {
                self.search_active = true;
                self.focus = Pane::List;
            }
            KeyCode::Char('u') => self.undo_trash(),
            KeyCode::Char('q') => self.quit = true,
            _ => {}
        }
    }
}
