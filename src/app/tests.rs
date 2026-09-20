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
