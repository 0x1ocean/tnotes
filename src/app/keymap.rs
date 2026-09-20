//! Key bindings: one table drives global routing, the help overlay and the settings `keys` page.

use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Where a binding applies. `Global` bindings are routed by `App::on_key`; the others are
/// documentation for keys handled inside their pane.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    Global,
    List,
    Tree,
    Editor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Quit,
    NewNote,
    TogglePreview,
    CloseTab,
    NextTab,
    PrevTab,
    ToggleSidebar,
    Help,
    Settings,
    NextPane,
    PrevPane,
    /// Documentation-only entries (routed by the owning pane).
    Doc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Chord {
    pub code: KeyCode,
    pub mods: KeyModifiers,
}

const fn ctrl(c: char) -> Chord {
    Chord {
        code: KeyCode::Char(c),
        mods: KeyModifiers::CONTROL,
    }
}

const fn alt(code: KeyCode) -> Chord {
    Chord {
        code,
        mods: KeyModifiers::ALT,
    }
}

const fn plain(code: KeyCode) -> Chord {
    Chord {
        code,
        mods: KeyModifiers::NONE,
    }
}

pub struct Binding {
    pub scope: Scope,
    pub action: Action,
    pub label: &'static str,
    /// Chords that trigger `action`; only meaningful for `Scope::Global`.
    pub chords: &'static [Chord],
    /// Text shown in help/settings; `None` → derived from `chords`.
    pub keys: Option<&'static str>,
}

const fn bind(
    scope: Scope,
    action: Action,
    label: &'static str,
    chords: &'static [Chord],
) -> Binding {
    Binding {
        scope,
        action,
        label,
        chords,
        keys: None,
    }
}

const fn doc(scope: Scope, keys: &'static str, label: &'static str) -> Binding {
    Binding {
        scope,
        action: Action::Doc,
        label,
        chords: &[],
        keys: Some(keys),
    }
}

use Scope::*;

pub const BINDINGS: &[Binding] = &[
    bind(Global, Action::NewNote, "new note", &[ctrl('t')]),
    bind(
        Global,
        Action::TogglePreview,
        "preview / edit",
        &[ctrl('l')],
    ),
    bind(Global, Action::CloseTab, "close tab", &[ctrl('w')]),
    bind(
        Global,
        Action::NextTab,
        "next tab",
        &[
            alt(KeyCode::Char('.')),
            ctrl('x'),
            Chord {
                code: KeyCode::PageDown,
                mods: KeyModifiers::CONTROL,
            },
            alt(KeyCode::Right),
        ],
    ),
    bind(
        Global,
        Action::PrevTab,
        "previous tab",
        &[
            alt(KeyCode::Char(',')),
            Chord {
                code: KeyCode::PageUp,
                mods: KeyModifiers::CONTROL,
            },
            alt(KeyCode::Left),
        ],
    ),
    doc(Global, "1-9", "jump to tab"),
    bind(
        Global,
        Action::ToggleSidebar,
        "sidebar / panel",
        &[ctrl('b')],
    ),
    bind(
        Global,
        Action::NextPane,
        "next pane",
        &[plain(KeyCode::Tab)],
    ),
    bind(
        Global,
        Action::PrevPane,
        "previous pane",
        &[plain(KeyCode::BackTab)],
    ),
    bind(
        Global,
        Action::Settings,
        "settings",
        &[plain(KeyCode::F(2))],
    ),
    bind(Global, Action::Help, "help", &[plain(KeyCode::F(1))]),
    bind(Global, Action::Quit, "quit", &[ctrl('q')]),
    doc(List, "j/k ↑/↓", "move"),
    doc(List, "enter / l", "edit (pins the tab)"),
    doc(List, "/", "search"),
    doc(List, "d", "trash (in trash: delete)"),
    doc(List, "u", "undo trash"),
    doc(List, "r", "restore (trash)"),
    doc(List, "m", "move to folder"),
    doc(List, "s", "sort"),
    doc(List, "gg / G", "top / bottom"),
    doc(List, "? ,", "help / settings"),
    doc(Tree, "enter", "filter"),
    doc(Tree, "space", "fold"),
    doc(Tree, "N / R / D", "new / rename / delete folder"),
    doc(Tree, "h / l", "switch pane"),
    doc(Editor, "esc", "back to list"),
    doc(Editor, "#… tab", "complete tag"),
    doc(Editor, "enter", "continue list · empty item ends it"),
    doc(Editor, "alt+x", "toggle task (or click the box)"),
    doc(Editor, "tab / shift+tab", "nest / un-nest list item"),
    doc(Editor, "ctrl+click", "#tag → filter by it"),
];

/// Global action for a key event, if any.
pub fn lookup(k: &KeyEvent) -> Option<Action> {
    let mods = k.modifiers & (KeyModifiers::CONTROL | KeyModifiers::ALT);
    BINDINGS
        .iter()
        .filter(|b| b.scope == Global)
        .find(|b| b.chords.iter().any(|c| c.code == k.code && c.mods == mods))
        .map(|b| b.action)
}

pub fn chord_label(c: &Chord) -> String {
    let key = match c.code {
        KeyCode::Char(' ') => "space".to_string(),
        KeyCode::Char(ch) => ch.to_string(),
        KeyCode::Tab => "tab".to_string(),
        KeyCode::BackTab => "shift+tab".to_string(),
        KeyCode::F(n) => format!("F{n}"),
        KeyCode::Left => "←".to_string(),
        KeyCode::Right => "→".to_string(),
        KeyCode::Up => "↑".to_string(),
        KeyCode::Down => "↓".to_string(),
        KeyCode::PageUp => "pgup".to_string(),
        KeyCode::PageDown => "pgdn".to_string(),
        KeyCode::Enter => "enter".to_string(),
        KeyCode::Esc => "esc".to_string(),
        other => format!("{other:?}").to_lowercase(),
    };
    let mut out = String::new();
    if c.mods.contains(KeyModifiers::CONTROL) {
        out.push_str("ctrl+");
    }
    if c.mods.contains(KeyModifiers::ALT) {
        out.push_str("alt+");
    }
    out.push_str(&key);
    out
}

impl Binding {
    /// `keys` text for display: explicit or all chords joined by ` · `.
    pub fn keys_label(&self) -> String {
        match self.keys {
            Some(k) => k.to_string(),
            None => self
                .chords
                .iter()
                .map(chord_label)
                .collect::<Vec<_>>()
                .join(" · "),
        }
    }
}

/// Bindings of one scope, in table order.
pub fn in_scope(scope: Scope) -> impl Iterator<Item = &'static Binding> {
    BINDINGS.iter().filter(move |b| b.scope == scope)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_matches_global_chords_only() {
        let ev = |code, mods| KeyEvent::new(code, mods);
        assert_eq!(
            lookup(&ev(KeyCode::Char('t'), KeyModifiers::CONTROL)),
            Some(Action::NewNote)
        );
        assert_eq!(
            lookup(&ev(KeyCode::Char('.'), KeyModifiers::ALT)),
            Some(Action::NextTab)
        );
        assert_eq!(lookup(&ev(KeyCode::Char('t'), KeyModifiers::NONE)), None);
        assert_eq!(
            lookup(&ev(KeyCode::PageDown, KeyModifiers::CONTROL)),
            Some(Action::NextTab)
        );
        // Shift is not part of a chord: `F2` with Shift still opens settings.
        assert_eq!(
            lookup(&ev(KeyCode::F(2), KeyModifiers::SHIFT)),
            Some(Action::Settings)
        );
    }

    #[test]
    fn every_global_binding_has_a_chord() {
        assert!(
            in_scope(Scope::Global)
                .filter(|b| b.action != Action::Doc)
                .all(|b| !b.chords.is_empty())
        );
    }
}
