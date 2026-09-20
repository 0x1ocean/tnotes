//! Settings page: sections on the left, rows on the right; every change applies immediately
//! and is written to `config.toml` when the page closes.

use super::*;

pub const SETTINGS_SECTIONS: [&str; 5] = ["folders", "appearance", "editor", "keys", "about"];

/// Cursor on the settings page: section on the left, row on the right.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SettingsPage {
    pub section: usize,
    pub row: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingId {
    Root(usize),
    AddRoot,
    Compact,
    Dates,
    Counts,
    SidebarWidth,
    TabNumbers,
    EditorKeys,
    Autosave,
    Template,
    Wrap,
    TextWidth,
    Align,
    Cursor,
    Blink,
    /// Read-only line.
    Info,
}

/// How a settings row is shown and what clicking it does.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SettingKind {
    /// Segmented choice; clicking an option selects it directly.
    Choice {
        options: Vec<&'static str>,
        current: usize,
    },
    /// `‹ value ›` stepper.
    Number(String),
    /// Click opens an inline prompt.
    Text(String),
    /// Root folder: path + count, `×` removes.
    Root(String),
    /// A single clickable line (e.g. `+ add folder…`).
    Action,
    Info(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettingRow {
    pub label: String,
    pub id: SettingId,
    pub kind: SettingKind,
}

impl SettingRow {
    pub fn editable(&self) -> bool {
        !matches!(self.kind, SettingKind::Info(_))
    }
}

mod roots;
mod rows;

impl App {
    pub(in crate::app) fn open_settings(&mut self) {
        self.settings = Some(SettingsPage::default());
        self.overlay = Overlay::None;
        self.search_active = false;
        self.tag_popup = None;
    }

    /// `?` / `F1`: the keys section of the settings page.
    pub(in crate::app) fn open_help(&mut self) {
        self.open_settings();
        if let Some(i) = SETTINGS_SECTIONS.iter().position(|s| *s == "keys") {
            self.select_section(i);
        }
    }

    pub(in crate::app) fn close_settings(&mut self) {
        self.settings = None;
        self.overlay = Overlay::None;
        if let Err(e) = config::save(&self.cfg) {
            self.fail("config save", e);
        }
    }

    pub(in crate::app) fn settings_page_mut(&mut self) -> &mut SettingsPage {
        self.settings.get_or_insert_with(SettingsPage::default)
    }

    pub(in crate::app) fn select_section(&mut self, i: usize) {
        let page = self.settings_page_mut();
        page.section = i.min(SETTINGS_SECTIONS.len() - 1);
        page.row = 0;
    }

    pub(in crate::app) fn select_row(&mut self, i: usize) {
        let n = self.settings_rows().len();
        self.settings_page_mut().row = i.min(n.saturating_sub(1));
    }

    pub(in crate::app) fn settings_key(&mut self, k: KeyEvent) {
        let page = self.settings.unwrap_or_default();
        let folders = SETTINGS_SECTIONS[page.section] == "folders";
        let editable_roots = folders && !self.cfg.roots_fixed;
        match k.code {
            KeyCode::Esc | KeyCode::F(2) => self.close_settings(),
            KeyCode::Char('q') if k.modifiers.contains(KeyModifiers::CONTROL) => {
                self.close_settings();
                self.quit = true;
            }
            KeyCode::Tab | KeyCode::Char(']') => {
                self.select_section((page.section + 1) % SETTINGS_SECTIONS.len())
            }
            KeyCode::BackTab | KeyCode::Char('[') => self.select_section(
                (page.section + SETTINGS_SECTIONS.len() - 1) % SETTINGS_SECTIONS.len(),
            ),
            KeyCode::Char(c @ '1'..='5') => self.select_section(c as usize - '1' as usize),
            KeyCode::Char('j') | KeyCode::Down => self.select_row(page.row + 1),
            KeyCode::Char('k') | KeyCode::Up => self.select_row(page.row.saturating_sub(1)),
            KeyCode::Enter | KeyCode::Char(' ') | KeyCode::Char('l') | KeyCode::Right => {
                self.adjust_setting(1)
            }
            KeyCode::Char('h') | KeyCode::Left => self.adjust_setting(-1),
            KeyCode::Char('a') if editable_roots => self.prompt_add_root(),
            KeyCode::Char('d') if editable_roots => {
                if let Some(i) = self.selected_root() {
                    self.confirm_remove_root(i);
                }
            }
            KeyCode::Char('J') if editable_roots => self.move_root(1),
            KeyCode::Char('K') if editable_roots => self.move_root(-1),
            _ => {}
        }
    }

    /// Mouse on the settings page: sections, option buttons, steppers, `×`, rows.
    pub(in crate::app) fn settings_click(&mut self, pos: Position) {
        if let Some(i) = self
            .hits
            .settings_sections
            .iter()
            .position(|r| r.contains(pos))
        {
            self.select_section(i);
            return;
        }
        if self.hits.sidebar_toggle.contains(pos)
            || self.hits.back.contains(pos)
            || self.hits.settings_back.contains(pos)
        {
            self.close_settings();
            return;
        }
        // Controls inside rows: `(row, control, rect)`.
        if let Some(&(row, ctl, _)) = self
            .hits
            .settings_controls
            .iter()
            .find(|(_, _, r)| r.contains(pos))
        {
            self.select_row(row);
            let Some(sr) = self.settings_rows().into_iter().nth(row) else {
                return;
            };
            match sr.kind {
                SettingKind::Choice { .. } => self.set_choice(sr.id, ctl),
                SettingKind::Number(_) => self.step_number(sr.id, if ctl == 0 { -1 } else { 1 }),
                SettingKind::Root(_) => {
                    if let SettingId::Root(i) = sr.id {
                        self.confirm_remove_root(i);
                    }
                }
                _ => self.adjust_setting(1),
            }
            return;
        }
        if let Some(&(i, _)) = self
            .hits
            .settings_rows
            .iter()
            .find(|(_, r)| r.contains(pos))
        {
            self.select_row(i);
            if let Some(sr) = self.settings_rows().into_iter().nth(i)
                && matches!(sr.kind, SettingKind::Text(_) | SettingKind::Action)
            {
                self.adjust_setting(1);
            }
        }
    }
}
