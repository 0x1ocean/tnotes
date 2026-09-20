//! Headless behaviour tests: drive `App` through `on_key`/`on_external` with a temp vault.
use std::fs;
use std::path::PathBuf;
use std::sync::mpsc::{Sender, channel};
use std::time::SystemTime;

use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::*;
use crate::config::{Appearance, Config, Editor};
use crate::store::Store;
use crate::ui::theme::Palette;

/// Temp vault + app. The `Sender` keeps the watch channel open; drop order is irrelevant.
pub(super) struct Fixture {
    pub app: App,
    pub dir: PathBuf,
    _tx: Sender<PathBuf>,
}

pub(super) fn config(dir: &PathBuf) -> Config {
    Config {
        roots: vec![dir.clone()],
        roots_fixed: false,
        first_run: false,
        path: None,
        appearance: Appearance::default(),
        editor: Editor::default(),
        theme: Palette::default(),
    }
}

pub(super) fn fixture(notes: &[(&str, &str)]) -> Fixture {
    let dir = std::env::temp_dir().join(format!(
        "tnotes-app-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&dir).unwrap();
    for (name, text) in notes {
        fs::write(dir.join(name), text).unwrap();
    }
    let store = Store::load(&[dir.clone()]).unwrap();
    let (tx, rx) = channel();
    let app = App::new(config(&dir), store, rx, Vec::new());
    Fixture { app, dir, _tx: tx }
}

pub(super) fn key(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
}
pub(super) fn ctrl(c: char) -> KeyEvent {
    KeyEvent::new(KeyCode::Char(c), KeyModifiers::CONTROL)
}
pub(super) fn alt(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::ALT)
}
pub(super) fn type_str(app: &mut App, s: &str) {
    for c in s.chars() {
        app.on_key(key(KeyCode::Char(c)));
    }
}

fn note_path(app: &App, name: &str) -> PathBuf {
    app.store
        .notes
        .iter()
        .map(|n| n.path.clone())
        .find(|p| p.file_name().is_some_and(|f| f == name))
        .unwrap_or_else(|| panic!("no note {name}"))
}

/// Visible-list index of the note called `name`.
fn list_index(app: &App, name: &str) -> usize {
    let p = note_path(app, name);
    app.visible
        .iter()
        .position(|&i| app.store.notes[i].path == p)
        .unwrap()
}

fn open_pinned(app: &mut App, name: &str) {
    app.focus = Pane::List;
    let i = list_index(app, name);
    app.select_list(i);
    app.on_key(key(KeyCode::Enter));
}

fn line(app: &App, row: usize) -> String {
    app.tabs[app.active]
        .editor
        .lines
        .get(RowIndex::new(row))
        .map(|l| l.iter().collect())
        .unwrap_or_default()
}

#[test]
fn enter_pins_the_preview_tab() {
    let mut f = fixture(&[("a.md", "# A\n"), ("b.md", "# B\n")]);
    let app = &mut f.app;
    app.focus = Pane::List;
    app.select_list(0);
    assert_eq!(app.tabs.len(), 1);
    assert!(!app.tabs[0].pinned);
    app.on_key(key(KeyCode::Enter));
    assert!(app.tabs[0].pinned);
    assert_eq!(app.focus, Pane::Editor);
}

#[test]
fn selecting_another_note_replaces_the_preview_tab() {
    let mut f = fixture(&[("a.md", "# A\n"), ("b.md", "# B\n")]);
    let app = &mut f.app;
    app.focus = Pane::List;
    app.select_list(0);
    app.select_list(1);
    assert_eq!(app.tabs.len(), 1);
    assert_eq!(app.tabs[0].path, app.store.notes[app.visible[1]].path);
}

#[test]
fn editing_pins_and_autosave_renames_by_title() {
    let mut f = fixture(&[("untitled.md", "# \n")]);
    let app = &mut f.app;
    open_pinned(app, "untitled.md");
    app.tabs[0].editor.cursor = Index2::new(0, 2);
    type_str(app, "Groceries");
    assert!(app.tabs[0].dirty && app.tabs[0].pinned);
    app.save_tab(0);
    assert_eq!(app.tabs[0].path, f.dir.join("groceries.md"));
    assert!(!f.dir.join("untitled.md").exists());
}

#[test]
fn trash_after_title_change_uses_the_renamed_path() {
    let mut f = fixture(&[("untitled.md", "# \n")]);
    let app = &mut f.app;
    open_pinned(app, "untitled.md");
    app.tabs[0].editor.cursor = Index2::new(0, 2);
    type_str(app, "Groceries");
    app.focus = Pane::List;
    app.on_key(key(KeyCode::Char('d')));
    assert!(app.store.notes.is_empty());
    assert_eq!(app.store.trash.len(), 1);
    assert!(f.dir.join(".Trash/groceries.md").exists());
    assert!(app.tabs.is_empty());
}

#[test]
fn close_tab_keeps_the_active_note() {
    let mut f = fixture(&[("a.md", "# A\n"), ("b.md", "# B\n"), ("c.md", "# C\n")]);
    let app = &mut f.app;
    app.focus = Pane::List;
    for i in 0..3 {
        app.select_list(i);
        app.pin_active();
    }
    assert_eq!(app.tabs.len(), 3);
    assert_eq!(app.active, 2);
    let p = app.tabs[2].path.clone();
    app.close_tab(0);
    assert_eq!(app.tabs.len(), 2);
    assert_eq!(app.tabs[app.active].path, p);
}

#[test]
fn session_roundtrip_keeps_tab_order_and_pinning() {
    let mut f = fixture(&[("a.md", "# A\n"), ("b.md", "# B\n")]);
    let (a, b) = (note_path(&f.app, "a.md"), note_path(&f.app, "b.md"));
    let app = &mut f.app;
    app.push_tab(a.clone(), false);
    app.push_tab(b.clone(), true);
    app.active = 1;
    app.sort = SortMode::Title;
    app.collapsed.insert(TAGS_KEY.into());
    let s = app.session_snapshot();

    let store = Store::load(&[f.dir.clone()]).unwrap();
    let (_tx, rx) = channel();
    let mut app = App::new(config(&f.dir), store, rx, Vec::new());
    app.apply_session(s);
    assert_eq!(app.tabs.len(), 2);
    assert!(app.tabs[0].path == a && !app.tabs[0].pinned);
    assert!(app.tabs[1].path == b && app.tabs[1].pinned);
    assert_eq!(app.active, 1);
    assert_eq!(app.sort, SortMode::Title);
    assert!(app.collapsed.contains(TAGS_KEY));
}

#[test]
fn enter_continues_and_ends_lists() {
    let mut f = fixture(&[("t.md", "# T\n\n- item")]);
    let app = &mut f.app;
    open_pinned(app, "t.md");
    app.tabs[0].editor.cursor = Index2::new(2, 6);
    app.on_key(key(KeyCode::Enter));
    assert_eq!(line(app, 3), "- ");
    app.on_key(key(KeyCode::Enter));
    assert_eq!(line(app, 3), "");
    assert_eq!(app.tabs[0].editor.lines.len(), 4);
}

#[test]
fn alt_x_toggles_the_checkbox() {
    let mut f = fixture(&[("t.md", "- [ ] task")]);
    let app = &mut f.app;
    open_pinned(app, "t.md");
    app.tabs[0].editor.cursor = Index2::new(0, 0);
    app.on_key(alt(KeyCode::Char('x')));
    assert!(line(app, 0).starts_with("- [x] "));
    app.on_key(alt(KeyCode::Char('x')));
    assert!(line(app, 0).starts_with("- [ ] "));
}

#[test]
fn ctrl_t_from_editor_creates_a_note_in_the_current_folder() {
    let mut f = fixture(&[("a.md", "# A\n")]);
    let app = &mut f.app;
    open_pinned(app, "a.md");
    app.on_key(ctrl('t'));
    assert_eq!(app.store.notes.len(), 2);
    let tab = &app.tabs[app.active];
    assert_eq!(tab.editor.lines.to_string(), Editor::default().template);
    assert!(tab.pinned);
    assert_eq!(app.focus, Pane::Editor);
    assert_eq!(tab.editor.cursor, Index2::new(0, 2));
}

#[test]
fn external_delete_of_a_dirty_tab_recreates_the_file() {
    let mut f = fixture(&[("a.md", "# A\n")]);
    let app = &mut f.app;
    open_pinned(app, "a.md");
    let path = app.tabs[0].path.clone();
    app.tabs[0].editor.cursor = Index2::new(0, 0);
    type_str(app, "x");
    assert!(app.tabs[0].dirty);
    fs::remove_file(&path).unwrap();
    app.on_external(path.clone());
    assert!(path.exists());
    assert!(fs::read_to_string(&path).unwrap().starts_with('x'));
    assert!(!app.tabs[0].dirty);
}

#[test]
fn confirm_overlay_swallows_pane_keys() {
    let mut f = fixture(&[("a.md", "# A\n"), ("b.md", "# B\n")]);
    let app = &mut f.app;
    app.focus = Pane::List;
    app.overlay = Overlay::Confirm(ConfirmAction::Purge(PathBuf::from("/nope")));
    app.on_key(key(KeyCode::Char('j')));
    assert_eq!(app.overlay, Overlay::None);
    assert_eq!(app.list_sel, 0);
    app.on_key(key(KeyCode::Char('j')));
    assert_eq!(app.list_sel, 1);
}

#[test]
fn alt_period_switches_tabs_from_the_editor() {
    let mut f = fixture(&[("a.md", "# A\n"), ("b.md", "# B\n")]);
    let app = &mut f.app;
    app.focus = Pane::List;
    app.select_list(0);
    app.pin_active();
    app.select_list(1);
    app.pin_active();
    app.activate_tab(0);
    app.focus = Pane::Editor;
    app.on_key(alt(KeyCode::Char('.')));
    assert_eq!(app.active, 1);
    app.on_key(alt(KeyCode::Char(',')));
    assert_eq!(app.active, 0);
}
