//! Rows of each settings section and how changing them applies to `cfg`.

use super::*;
use crate::app::keymap::Scope;

impl App {
    /// Rows of the current section, rebuilt from `cfg` on every call.
    pub fn settings_rows(&self) -> Vec<SettingRow> {
        let row = |label: &str, id: SettingId, kind: SettingKind| SettingRow {
            label: label.to_string(),
            id,
            kind,
        };
        let info =
            |label: &str, value: String| row(label, SettingId::Info, SettingKind::Info(value));
        let choice = |label: &str, id: SettingId, options: &[&'static str], current: usize| {
            row(
                label,
                id,
                SettingKind::Choice {
                    options: options.to_vec(),
                    current,
                },
            )
        };
        let page = self.settings.unwrap_or_default();
        let a = &self.cfg.appearance;
        let e = &self.cfg.editor;
        match SETTINGS_SECTIONS[page.section] {
            "folders" => {
                let mut rows: Vec<SettingRow> = self
                    .cfg
                    .roots
                    .iter()
                    .enumerate()
                    .map(|(i, r)| {
                        let n = self.store.notes.iter().filter(|n| n.root == i).count();
                        let label = if self.cfg.roots_fixed {
                            format!("{}  (fixed by --dir)", config::contract_tilde(r))
                        } else {
                            config::contract_tilde(r)
                        };
                        row(
                            &label,
                            SettingId::Root(i),
                            SettingKind::Root(if n == 1 {
                                "1 note".to_string()
                            } else {
                                format!("{n} notes")
                            }),
                        )
                    })
                    .collect();
                if !self.cfg.roots_fixed {
                    rows.push(row(
                        "+ add folder…",
                        SettingId::AddRoot,
                        SettingKind::Action,
                    ));
                }
                rows
            }
            "appearance" => vec![
                choice(
                    "list items",
                    SettingId::Compact,
                    &["title + preview", "compact"],
                    a.compact as usize,
                ),
                choice(
                    "dates",
                    SettingId::Dates,
                    &["relative", "absolute"],
                    (a.dates == Dates::Absolute) as usize,
                ),
                choice(
                    "counts in tree",
                    SettingId::Counts,
                    &["on", "off"],
                    (!a.counts) as usize,
                ),
                row(
                    "sidebar width",
                    SettingId::SidebarWidth,
                    SettingKind::Number(a.sidebar_width.to_string()),
                ),
                choice(
                    "tab numbers",
                    SettingId::TabNumbers,
                    &["always", "when 2+ tabs"],
                    (a.tab_numbers == TabNumbers::Multi) as usize,
                ),
                info("colors", "edit [theme] in config.toml".into()),
            ],
            "editor" => vec![
                choice(
                    "editor keybindings",
                    SettingId::EditorKeys,
                    &["emacs", "vim"],
                    (e.keys == EditorKeys::Vim) as usize,
                ),
                row(
                    "autosave after",
                    SettingId::Autosave,
                    SettingKind::Number(format!("{} ms", e.autosave_ms)),
                ),
                row(
                    "new note template",
                    SettingId::Template,
                    SettingKind::Text(format!("{:?}", e.template)),
                ),
                choice(
                    "wrap long lines",
                    SettingId::Wrap,
                    &["on", "off"],
                    (!e.wrap) as usize,
                ),
                row(
                    "text width",
                    SettingId::TextWidth,
                    SettingKind::Number(if e.width == 0 {
                        "full".to_string()
                    } else {
                        e.width.to_string()
                    }),
                ),
                choice(
                    "text column",
                    SettingId::Align,
                    &["left", "center"],
                    (e.align == config::Align::Center) as usize,
                ),
                choice(
                    "cursor",
                    SettingId::Cursor,
                    &["drawn block", "bar", "underline", "block"],
                    match e.cursor {
                        config::CursorShape::Drawn => 0,
                        config::CursorShape::Bar => 1,
                        config::CursorShape::Underline => 2,
                        config::CursorShape::Block => 3,
                    },
                ),
                choice(
                    "cursor blink",
                    SettingId::Blink,
                    &["on", "off"],
                    (!e.blink) as usize,
                ),
            ],
            "keys" => {
                let header = |name: &str| info(name, String::new());
                let mut rows = Vec::new();
                for (scope, name) in [
                    (Scope::Global, "global"),
                    (Scope::List, "list"),
                    (Scope::Tree, "tree"),
                    (Scope::Editor, "editor"),
                ] {
                    rows.push(header(name));
                    rows.extend(keymap::in_scope(scope).map(|b| info(b.label, b.keys_label())));
                }
                rows.push(header(match self.keys {
                    EditorKeys::Emacs => "editor · emacs",
                    EditorKeys::Vim => "editor · vim",
                }));
                let scheme: &[(&str, &str)] = match self.keys {
                    EditorKeys::Emacs => &[
                        ("find in note", "ctrl+s"),
                        ("undo / redo", "ctrl+u / ctrl+r"),
                        ("word back / forward", "alt+b / alt+f"),
                    ],
                    EditorKeys::Vim => &[
                        ("insert", "i a I A o O"),
                        ("normal · then back to list", "esc"),
                        ("undo / redo / repeat", "u · ctrl+r · ."),
                        ("search", "/ n N"),
                    ],
                };
                rows.extend(scheme.iter().map(|(l, k)| info(l, k.to_string())));
                rows.push(header("mouse"));
                rows.push(info("click / double-click", "select · pin".into()));
                rows.push(info("right-click", "context menu".into()));
                rows.push(info(
                    "wheel / drag",
                    "scroll · select text (copied on release)".into(),
                ));
                rows.push(info("shift+drag", "the terminal's own selection".into()));
                rows
            }
            _ => vec![
                info("version", env!("CARGO_PKG_VERSION").into()),
                info(
                    "config",
                    config::config_path()
                        .map(|p| config::contract_tilde(&p))
                        .unwrap_or_default(),
                ),
                info(
                    "session",
                    session::path()
                        .map(|p| config::contract_tilde(&p))
                        .unwrap_or_default(),
                ),
                info(
                    "notes",
                    format!(
                        "{} in {} folders · {} in trash",
                        self.store.notes.len(),
                        self.store.roots.len() + self.store.folders.len(),
                        self.store.trash.len()
                    ),
                ),
            ],
        }
    }

    /// One-line hint for the current section.
    pub fn settings_hint(&self) -> &'static str {
        let page = self.settings.unwrap_or_default();
        match SETTINGS_SECTIONS[page.section] {
            "folders" if self.cfg.roots_fixed => "folders are fixed by --dir for this session",
            "folders" => "click × to remove · J/K reorder · a add · d remove",
            "appearance" | "editor" => "click a value to set it · h/l or enter to cycle",
            "keys" => "bindings are fixed for now — remapping comes later",
            _ => "",
        }
    }

    /// Set a choice row to option `opt`.
    pub(in crate::app) fn set_choice(&mut self, id: SettingId, opt: usize) {
        let a = &mut self.cfg.appearance;
        let e = &mut self.cfg.editor;
        match id {
            SettingId::Compact => a.compact = opt == 1,
            SettingId::Dates => {
                a.dates = if opt == 1 {
                    Dates::Absolute
                } else {
                    Dates::Relative
                }
            }
            SettingId::Counts => a.counts = opt == 0,
            SettingId::TabNumbers => {
                a.tab_numbers = if opt == 1 {
                    TabNumbers::Multi
                } else {
                    TabNumbers::Always
                }
            }
            SettingId::EditorKeys => self.apply_editor_keys(if opt == 1 {
                EditorKeys::Vim
            } else {
                EditorKeys::Emacs
            }),
            SettingId::Wrap => e.wrap = opt == 0,
            SettingId::Cursor => {
                e.cursor = match opt {
                    1 => config::CursorShape::Bar,
                    2 => config::CursorShape::Underline,
                    3 => config::CursorShape::Block,
                    _ => config::CursorShape::Drawn,
                }
            }
            SettingId::Blink => e.blink = opt == 0,
            SettingId::Align => {
                e.align = if opt == 1 {
                    config::Align::Center
                } else {
                    config::Align::Left
                }
            }
            _ => {}
        }
    }

    /// Step a number row by `delta`.
    pub(in crate::app) fn step_number(&mut self, id: SettingId, delta: i32) {
        let a = &mut self.cfg.appearance;
        let e = &mut self.cfg.editor;
        match id {
            SettingId::SidebarWidth => {
                let w = (a.sidebar_width as i32 + delta * 2).clamp(
                    *config::SIDEBAR_WIDTH_RANGE.start() as i32,
                    *config::SIDEBAR_WIDTH_RANGE.end() as i32,
                );
                a.sidebar_width = w as u16;
            }
            SettingId::Autosave => {
                let ms = (e.autosave_ms as i64 + delta as i64 * 100).clamp(
                    *config::AUTOSAVE_RANGE.start() as i64,
                    *config::AUTOSAVE_RANGE.end() as i64,
                );
                e.autosave_ms = ms as u64;
            }
            SettingId::TextWidth => {
                // 0 (full) sits above the top of the range: … 150 → 160 → full.
                let (lo, hi) = (
                    *config::TEXT_WIDTH_RANGE.start() as i32,
                    *config::TEXT_WIDTH_RANGE.end() as i32,
                );
                let cur = if e.width == 0 {
                    hi + 10
                } else {
                    e.width as i32
                };
                let next = (cur + delta * 10).clamp(lo, hi + 10);
                e.width = if next > hi { 0 } else { next as u16 };
            }
            _ => {}
        }
    }

    /// Activate a row: open its prompt, run its action, or cycle its value by `delta`.
    pub(in crate::app) fn adjust_setting(&mut self, delta: i32) {
        let page = self.settings.unwrap_or_default();
        let Some(row) = self.settings_rows().into_iter().nth(page.row) else {
            return;
        };
        match row.kind {
            SettingKind::Choice { options, current } => {
                let n = options.len() as i32;
                let next = (current as i32 + delta).rem_euclid(n) as usize;
                self.set_choice(row.id, next);
            }
            SettingKind::Number(_) => self.step_number(row.id, delta),
            SettingKind::Text(_) => {
                self.prompt = single_line(&self.cfg.editor.template.replace('\n', "\\n"));
                self.overlay = Overlay::Prompt(PromptKind::Template);
            }
            SettingKind::Action => self.prompt_add_root(),
            SettingKind::Root(_) | SettingKind::Info(_) => {}
        }
    }

    /// Toggle emacs ↔ vim (status-bar click) and write the config.
    pub(in crate::app) fn toggle_editor_keys(&mut self) {
        let keys = match self.cfg.editor.keys {
            EditorKeys::Emacs => EditorKeys::Vim,
            EditorKeys::Vim => EditorKeys::Emacs,
        };
        self.apply_editor_keys(keys);
        if let Err(e) = config::save(&self.cfg) {
            self.fail("config save", e);
        }
        self.set_status(
            StatusKind::Info,
            format!(
                "editor keys: {}",
                match keys {
                    EditorKeys::Emacs => "emacs",
                    EditorKeys::Vim => "vim",
                }
            ),
        );
    }

    /// Switch the running editor handler; existing tabs get the scheme's resting mode.
    pub(in crate::app) fn apply_editor_keys(&mut self, keys: EditorKeys) {
        self.cfg.editor.keys = keys;
        self.keys = keys;
        self.editor_handler = match keys {
            EditorKeys::Emacs => EditorEventHandler::emacs_mode(),
            EditorKeys::Vim => EditorEventHandler::vim_mode(),
        };
        for t in &mut self.tabs {
            t.editor.mode = match keys {
                EditorKeys::Emacs => EditorMode::Insert,
                EditorKeys::Vim => EditorMode::Normal,
            };
            t.editor.selection = None;
        }
    }

    pub(in crate::app) fn set_template(&mut self, text: &str) {
        let text = text.replace("\\n", "\n");
        self.cfg.editor.template = if text.is_empty() {
            "# ".to_string()
        } else {
            text
        };
    }
}
