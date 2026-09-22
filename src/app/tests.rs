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

pub(super) fn config(dir: &Path) -> Config {
    Config {
        roots: vec![dir.to_path_buf()],
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
    let store = Store::load(std::slice::from_ref(&dir)).unwrap();
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

#[test]
fn link_popup_completes_titles() {
    let mut f = fixture(&[("a.md", "# A\n\n"), ("plan.md", "# Weekly plan\n")]);
    let app = &mut f.app;
    open_pinned(app, "a.md");
    app.tabs[0].editor.cursor = Index2::new(1, 0);
    type_str(app, "[[wee");
    assert_eq!(
        app.popup.as_ref().map(|p| p.candidates.clone()),
        Some(vec!["Weekly plan".to_string()])
    );
    app.on_key(key(KeyCode::Tab));
    assert_eq!(line(app, 1), "[[Weekly plan]]");
    assert!(app.popup.is_none());
    assert_eq!(app.tabs[0].editor.cursor, Index2::new(1, 15));
    // Moving back into the finished link must not reopen the popup.
    for _ in 0..10 {
        app.on_key(key(KeyCode::Left));
    }
    assert_eq!(app.tabs[0].editor.cursor, Index2::new(1, 5));
    assert!(app.popup.is_none());
}

#[test]
fn alt_enter_follows_a_link_and_creates_a_missing_one() {
    let mut f = fixture(&[("a.md", "# A\n\n[[B]]\n"), ("b.md", "# B\n")]);
    let b = note_path(&f.app, "b.md");
    let app = &mut f.app;
    open_pinned(app, "a.md");
    app.tabs[0].editor.cursor = Index2::new(2, 2);
    app.on_key(alt(KeyCode::Enter));
    assert_eq!(app.tabs[app.active].path, b);

    let mut f = fixture(&[("a.md", "# A\n\n[[Zed]]\n")]);
    let app = &mut f.app;
    open_pinned(app, "a.md");
    app.tabs[0].editor.cursor = Index2::new(2, 3);
    app.on_key(alt(KeyCode::Enter));
    let zed = f.dir.join("zed.md");
    assert_eq!(app.store.notes.len(), 2);
    assert_eq!(fs::read_to_string(&zed).unwrap(), "# Zed\n");
    assert_eq!(app.tabs[app.active].path, zed);
    assert_eq!(app.tabs[app.active].editor.cursor, Index2::new(0, 5));
    assert_eq!(app.focus, Pane::Editor);
}

#[test]
fn title_change_rewrites_links_in_other_notes() {
    let mut f = fixture(&[
        ("b.md", "# B\n"),
        ("a.md", "# A\n\n[[B]]\n"),
        ("c.md", "# C\n\n[[b]]\n"),
    ]);
    let (a, c) = (note_path(&f.app, "a.md"), note_path(&f.app, "c.md"));
    let app = &mut f.app;
    open_pinned(app, "c.md");
    app.tabs[app.active].editor.cursor = Index2::new(0, 3);
    type_str(app, "x");
    open_pinned(app, "b.md");
    app.tabs[app.active].editor.cursor = Index2::new(0, 3);
    type_str(app, "ee");
    app.save_tab(app.active);
    assert_eq!(app.tabs[app.active].path, f.dir.join("bee.md"));
    assert_eq!(fs::read_to_string(&a).unwrap(), "# A\n\n[[Bee]]\n");
    let ct = app.tabs.iter().find(|t| t.path == c).unwrap();
    assert!(ct.dirty);
    assert_eq!(ct.editor.lines.to_string(), "# Cx\n\n[[Bee]]\n");
    assert_eq!(fs::read_to_string(&c).unwrap(), "# C\n\n[[b]]\n");
    let b_idx = app.store.resolve_link("Bee").unwrap();
    assert_eq!(app.store.backlinks(b_idx).len(), 1);
    assert!(
        app.status
            .as_ref()
            .unwrap()
            .text
            .contains("updated links in 2")
    );
}

#[test]
fn backlinks_overlay_opens_the_source_note() {
    let mut f = fixture(&[("a.md", "# A\n\n[[B]]\n"), ("b.md", "# B\n")]);
    let (a, b) = (note_path(&f.app, "a.md"), note_path(&f.app, "b.md"));
    let app = &mut f.app;
    open_pinned(app, "b.md");
    app.tabs[app.active].editor.cursor = Index2::new(0, 1);
    app.on_key(alt(KeyCode::Enter));
    assert_eq!(app.overlay, Overlay::Backlinks(b.clone()));
    assert_eq!(app.backlink_rows, vec![a.clone()]);
    app.on_key(key(KeyCode::Enter));
    assert_eq!(app.overlay, Overlay::None);
    assert_eq!(app.tabs[app.active].path, a);
    assert_eq!(app.focus, Pane::Editor);

    app.on_key(alt(KeyCode::Enter));
    assert_eq!(app.overlay, Overlay::None);
    assert_eq!(app.status.as_ref().unwrap().text, "no backlinks");
}

/// Render once so the editor knows its screen area, then drag-select `cols` chars on
/// editor line `row` (emacs keys).
fn drag_select(app: &mut App, row: u16, cols: u16) {
    use ratatui::backend::TestBackend;
    use ratatui::crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
    let mut term = ratatui::Terminal::new(TestBackend::new(100, 20)).unwrap();
    term.draw(|fr| app.hits = crate::ui::draw(fr, app)).unwrap();
    let ev = |kind, col, row| MouseEvent {
        kind,
        column: col,
        row,
        modifiers: KeyModifiers::NONE,
    };
    let (x, y) = (app.hits.editor.x, app.hits.editor.y + row);
    app.on_mouse(ev(MouseEventKind::Down(MouseButton::Left), x, y));
    // Several drag events: each one must extend the selection, not restart it.
    app.on_mouse(ev(MouseEventKind::Drag(MouseButton::Left), x + 1, y));
    app.on_mouse(ev(MouseEventKind::Drag(MouseButton::Left), x + cols - 1, y));
    app.on_mouse(ev(MouseEventKind::Up(MouseButton::Left), x + cols - 1, y));
    let ed = &app.tabs[app.active].editor;
    let sel = ed.selection.as_ref().expect("selection");
    assert_eq!((sel.start().col, sel.end().col), (0, cols as usize - 1));
    assert_eq!(ed.mode, EditorMode::Insert);
}

#[test]
fn menu_cut_removes_the_mouse_selection() {
    let mut f = fixture(&[("a.md", "# A\n\nline one here\n")]);
    let app = &mut f.app;
    app.sidebar = false;
    open_pinned(app, "a.md");
    drag_select(app, 2, 8);
    app.open_menu(Position::new(app.hits.editor.x + 5, app.hits.editor.y + 2));
    app.on_key(key(KeyCode::Char('x')));
    assert_eq!(line(app, 2), " here");
    assert!(app.tabs[0].dirty);
    // The editor is still usable afterwards.
    type_str(app, "Z");
    assert_eq!(line(app, 2), "Z here");
}

#[test]
fn typing_replaces_and_backspace_deletes_the_mouse_selection() {
    let mut f = fixture(&[("a.md", "# A\n\nline one here\n")]);
    let app = &mut f.app;
    app.sidebar = false;
    open_pinned(app, "a.md");
    drag_select(app, 2, 4);
    app.on_key(key(KeyCode::Backspace));
    assert_eq!(line(app, 2,), " one here");
    drag_select(app, 2, 4);
    type_str(app, "X");
    assert_eq!(line(app, 2), "X here");
    drag_select(app, 2, 1);
    app.on_key(key(KeyCode::Right));
    assert!(app.tabs[0].editor.selection.is_none());
    assert_eq!(line(app, 2), "X here");
}

#[test]
fn external_rename_retargets_a_clean_tab() {
    let mut f = fixture(&[("a.md", "# A\n")]);
    let (a, b) = (f.dir.join("a.md"), f.dir.join("b.md"));
    let app = &mut f.app;
    open_pinned(app, "a.md");
    fs::rename(&a, &b).unwrap();
    fs::write(&b, "# B\n").unwrap();
    app.on_external_batch(vec![a.clone(), b.clone()]);
    assert_eq!(app.tabs.len(), 1);
    assert_eq!(app.tabs[0].path, b);
    assert_eq!(line(app, 0), "# B");
    assert!(!app.tabs[0].dirty);

    // A file that merely appeared in an earlier batch is not a rename target.
    fs::write(f.dir.join("c.md"), "# C\n").unwrap();
    app.on_external_batch(vec![f.dir.join("c.md")]);
    fs::remove_file(&b).unwrap();
    app.on_external_batch(vec![b.clone()]);
    assert!(app.tabs.is_empty());
}

#[test]
fn external_rename_of_a_dirty_tab_keeps_the_edits_as_a_conflict_copy() {
    let mut f = fixture(&[("a.md", "# A\n")]);
    let (a, b) = (f.dir.join("a.md"), f.dir.join("b.md"));
    let app = &mut f.app;
    open_pinned(app, "a.md");
    app.tabs[0].editor.cursor = Index2::new(0, 3);
    type_str(app, "x");
    fs::rename(&a, &b).unwrap();
    fs::write(&b, "# B\n").unwrap();
    app.on_external_batch(vec![a.clone(), b.clone()]);
    assert_eq!(app.tabs[0].path, b);
    assert_eq!(line(app, 0), "# B");
    assert!(!app.tabs[0].dirty);
    let copies: Vec<_> = fs::read_dir(&f.dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| n.contains("conflict"))
        .collect();
    assert_eq!(copies.len(), 1);
    assert_eq!(
        fs::read_to_string(f.dir.join(&copies[0])).unwrap(),
        "# Ax\n"
    );
    assert!(app.status.as_ref().unwrap().text.contains("conflict"));
}

#[test]
fn highlight_setting_switches_live() {
    use crate::app::settings::SettingId;
    use crate::ui::theme::{code, dim};
    let mut f = fixture(&[("a.md", "# A\n\n`c`\n")]);
    let app = &mut f.app;
    open_pinned(app, "a.md");
    let fg_at = |app: &App, row, col| {
        let at = Index2::new(row, col);
        app.tabs[0]
            .editor
            .highlights
            .iter()
            .find(|h| h.start <= at && at <= h.end)
            .unwrap_or_else(|| panic!("no highlight at {row}:{col}"))
            .style
            .fg
    };
    assert_eq!(fg_at(app, 2, 1), Some(code()));
    app.set_choice(SettingId::Highlight, 1);
    assert_eq!(fg_at(app, 2, 1), Some(dim()));
    app.set_choice(SettingId::Highlight, 0);
    assert_eq!(fg_at(app, 2, 1), Some(code()));
}

#[test]
fn dragging_the_separator_resizes_the_sidebar() {
    use ratatui::backend::TestBackend;
    use ratatui::crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
    let mut f = fixture(&[("a.md", "# A\n")]);
    let app = &mut f.app;
    let mut term = ratatui::Terminal::new(TestBackend::new(100, 20)).unwrap();
    term.draw(|fr| app.hits = crate::ui::draw(fr, app)).unwrap();
    let ev = |kind, col| MouseEvent {
        kind,
        column: col,
        row: 5,
        modifiers: KeyModifiers::NONE,
    };
    let sep = app.hits.separator.x + 1;
    assert_eq!(sep, 32);
    app.on_mouse(ev(MouseEventKind::Down(MouseButton::Left), sep));
    app.on_mouse(ev(MouseEventKind::Drag(MouseButton::Left), sep + 6));
    assert_eq!(app.cfg.appearance.sidebar_width, 38);
    // Redraw moves the separator; the next drag is still relative to the sidebar's left edge.
    term.draw(|fr| app.hits = crate::ui::draw(fr, app)).unwrap();
    assert_eq!(app.hits.separator.x + 1, 38);
    app.on_mouse(ev(MouseEventKind::Drag(MouseButton::Left), 90));
    assert_eq!(app.cfg.appearance.sidebar_width, 48);
    app.on_mouse(ev(MouseEventKind::Drag(MouseButton::Left), 3));
    assert_eq!(app.cfg.appearance.sidebar_width, 24);
    app.on_mouse(ev(MouseEventKind::Up(MouseButton::Left), 3));
    assert!(app.sidebar_drag.is_none());
    // Clicks off the separator do not start a drag.
    app.on_mouse(ev(MouseEventKind::Down(MouseButton::Left), 60));
    assert!(app.sidebar_drag.is_none());
}
