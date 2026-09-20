use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};

use anyhow::Result;
use edtui::actions::{CopyLine, CopySelection, DeleteLine, DeleteSelection, InsertChar, Paste};
use edtui::{EditorEventHandler, EditorMode, EditorState, Index2, Lines, RowIndex};
use notify::RecommendedWatcher;
use nucleo_matcher::pattern::{CaseMatching, Normalization, Pattern};
use nucleo_matcher::{Matcher, Utf32Str};
use ratatui::DefaultTerminal;
use ratatui::crossterm::cursor::SetCursorStyle;
use ratatui::crossterm::event::{
    self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseButton, MouseEvent,
    MouseEventKind,
};
use ratatui::crossterm::execute;
use ratatui::layout::Position;

use crate::config::{self, Config, Dates, EditorKeys, TabNumbers};
use crate::index::{
    self, Filter, FolderRow, TagRow, build_folder_tree, build_tag_tree, dir_key, tag_key,
};
use crate::note::{self, Note};
use crate::session::{self, Session};
use crate::store::{Reload, SaveOutcome, Store};
use crate::ui::{self, Hits};
use crate::watch;

mod browser;
mod folders;
pub use browser::{Browser, DirRow};
pub use menu::Menu;
pub use navigate::TreeRow;
pub use settings::{SETTINGS_SECTIONS, SettingKind, SettingsPage};
pub mod keymap;
mod keys;
mod markdown;
mod menu;
mod mouse;
mod navigate;
mod notes;
mod persist;
mod picker;
mod settings;
mod sync;
mod tabs;
mod tags;

#[cfg(test)]
mod tests;

const STATUS_TTL: Duration = Duration::from_secs(3);
const UNDO_TRASH_WINDOW: Duration = Duration::from_secs(5);
const WHEEL_STEP: usize = 3;
const DOUBLE_CLICK: Duration = Duration::from_millis(400);
const TAG_POPUP_MAX: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pane {
    Tree,
    List,
    Editor,
}

/// Which layer receives input, top-most first. Both `on_key` and `on_mouse` dispatch on it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layer {
    Overlay,
    Settings,
    TagPopup,
    Main,
}

pub struct Tab {
    pub path: PathBuf,
    pub editor: EditorState,
    pub pinned: bool,
    pub dirty: bool,
    pub last_edit: Instant,
    pub preview: bool,
    pub preview_scroll: u16,
    /// Largest useful `preview_scroll` for the last rendered width/height.
    pub preview_max: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfirmAction {
    Purge(PathBuf),
    DeleteFolder(PathBuf),
    RemoveRoot(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PromptKind {
    NewFolder(PathBuf),
    RenameFolder(PathBuf),
    /// Create `parent/<name>` from the folder browser and add it as a root.
    NewRoot(PathBuf),
    /// New-note template (settings page).
    Template,
}

/// Collapse key of the `tags` section in the tree.
pub const TAGS_KEY: &str = "tags";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Overlay {
    None,
    Confirm(ConfirmAction),
    Prompt(PromptKind),
    /// Move-note picker for the note at this path.
    Picker(PathBuf),
    Menu(Menu),
    /// Folder browser (add root).
    Browse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SortMode {
    #[default]
    Modified,
    Title,
    Created,
}

impl SortMode {
    fn name(self) -> &'static str {
        match self {
            SortMode::Modified => "modified",
            SortMode::Title => "title",
            SortMode::Created => "created",
        }
    }

    fn parse(s: &str) -> SortMode {
        match s {
            "title" => SortMode::Title,
            "created" => SortMode::Created,
            _ => SortMode::Modified,
        }
    }
}

pub struct TagPopup {
    pub candidates: Vec<String>,
    pub sel: usize,
    /// Chars typed after `#` (the part already present in the buffer).
    pub typed: usize,
}

/// Responsive layout chosen by the last draw.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Layout {
    #[default]
    Wide,
    Narrow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusKind {
    Conflict,
    Failed,
    Reloaded,
    Info,
}

pub struct Status {
    pub kind: StatusKind,
    pub text: String,
    pub at: Instant,
    pub ttl: Duration,
}

pub struct App {
    pub store: Store,
    pub cfg: Config,
    matcher: Matcher,
    pub filter: Filter,
    pub collapsed: HashSet<String>,
    pub tree_rows: Vec<TreeRow>,
    pub tree_sel: usize,
    pub tree_offset: usize,
    pub visible: Vec<usize>,
    pub list_sel: usize,
    pub list_offset: usize,
    pub query: EditorState,
    query_handler: EditorEventHandler,
    pub search_active: bool,
    pub tabs: Vec<Tab>,
    pub active: usize,
    editor_handler: EditorEventHandler,
    /// Key scheme the running editor handler uses (config edits apply on next launch).
    pub keys: EditorKeys,
    pub sort: SortMode,
    pub tag_popup: Option<TagPopup>,
    pub prompt: EditorState,
    pub prompt_candidates: Vec<String>,
    pub picker_query: EditorState,
    pub picker_sel: usize,
    pub picker_rows: Vec<PathBuf>,
    pub browser: browser::Browser,
    /// Folder given on the command line: applied as the filter after the session restore.
    pub start_filter: Option<PathBuf>,
    /// `(trash path, original path, when)` of the last trashed note, for `u`.
    last_trash: Option<(PathBuf, PathBuf, Instant)>,
    pub focus: Pane,
    /// Navigator column shown (`Ctrl+b` / ▤ toggles).
    pub sidebar: bool,
    /// Settings page replaces the main view while `Some`.
    pub settings: Option<SettingsPage>,
    /// Narrow layout: tree hidden behind its header row so the list gets the space.
    pub tree_folded: bool,
    /// Last terminal cursor shape we emitted (`None` = default / editor-drawn).
    cursor_shape: Option<SetCursorStyle>,
    pub overlay: Overlay,
    pub layout: Layout,
    pub status: Option<Status>,
    pub hits: Hits,
    watch_rx: Receiver<PathBuf>,
    watchers: Vec<RecommendedWatcher>,
    pending_g: bool,
    /// Last list-row click `(row, when)` for double-click detection.
    last_click: Option<(usize, Instant)>,
    session_ready: bool,
    session_warned: bool,
    quit: bool,
}

fn single_line(text: &str) -> EditorState {
    let mut s = EditorState::new(Lines::from(text));
    s.set_single_line(true);
    s.mode = EditorMode::Insert;
    s.cursor = Index2::new(0, text.chars().count());
    s
}

fn file_name(p: &Path) -> String {
    p.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default()
}

fn base64(bytes: &[u8]) -> String {
    const T: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let n = (chunk[0] as u32) << 16
            | (chunk.get(1).copied().unwrap_or(0) as u32) << 8
            | chunk.get(2).copied().unwrap_or(0) as u32;
        out.push(T[(n >> 18) as usize & 63] as char);
        out.push(T[(n >> 12) as usize & 63] as char);
        out.push(if chunk.len() > 1 {
            T[(n >> 6) as usize & 63] as char
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            T[n as usize & 63] as char
        } else {
            '='
        });
    }
    out
}

impl App {
    pub fn new(
        cfg: Config,
        store: Store,
        watch_rx: Receiver<PathBuf>,
        watchers: Vec<RecommendedWatcher>,
    ) -> Self {
        let keys = cfg.editor.keys;
        let editor_handler = match keys {
            EditorKeys::Emacs => EditorEventHandler::emacs_mode(),
            EditorKeys::Vim => EditorEventHandler::vim_mode(),
        };
        let mut app = App {
            store,
            cfg,
            matcher: Matcher::new(nucleo_matcher::Config::DEFAULT),
            filter: Filter::All,
            collapsed: HashSet::new(),
            tree_rows: Vec::new(),
            tree_sel: 0,
            tree_offset: 0,
            visible: Vec::new(),
            list_sel: 0,
            list_offset: 0,
            query: single_line(""),
            query_handler: EditorEventHandler::emacs_mode(),
            search_active: false,
            tabs: Vec::new(),
            active: 0,
            editor_handler,
            keys,
            sort: SortMode::default(),
            tag_popup: None,
            prompt: single_line(""),
            prompt_candidates: Vec::new(),
            picker_query: single_line(""),
            picker_sel: 0,
            picker_rows: Vec::new(),
            browser: browser::Browser::default(),
            start_filter: None,
            last_trash: None,
            focus: Pane::List,
            sidebar: true,
            settings: None,
            tree_folded: false,
            cursor_shape: None,
            overlay: Overlay::None,
            layout: Layout::default(),
            status: None,
            hits: Hits::default(),
            watch_rx,
            watchers,
            pending_g: false,
            last_click: None,
            session_ready: false,
            session_warned: false,
            quit: false,
        };
        app.refresh();
        app
    }

    pub fn layer(&self) -> Layer {
        if self.overlay != Overlay::None {
            Layer::Overlay
        } else if self.settings.is_some() {
            Layer::Settings
        } else if self.tag_popup.is_some() && self.focus == Pane::Editor {
            Layer::TagPopup
        } else {
            Layer::Main
        }
    }

    pub fn run(&mut self, term: &mut DefaultTerminal) -> Result<()> {
        self.restore_session();
        while !self.quit {
            term.draw(|f| self.hits = ui::draw(f, self))?;
            self.apply_cursor_shape();
            if event::poll(Duration::from_millis(100))? {
                self.handle(event::read()?);
            }
            let mut changed: HashSet<PathBuf> = HashSet::new();
            while let Ok(p) = self.watch_rx.try_recv() {
                changed.insert(p);
            }
            for p in changed {
                self.on_external(p);
            }
            for i in 0..self.tabs.len() {
                if self.tabs[i].dirty
                    && self.tabs[i].last_edit.elapsed()
                        >= Duration::from_millis(self.cfg.editor.autosave_ms)
                {
                    self.save_tab(i);
                }
            }
            if self.status.as_ref().is_some_and(|s| s.at.elapsed() > s.ttl) {
                self.status = None;
            }
        }
        self.save_all_tabs();
        if self.cursor_shape.is_some() {
            let _ = execute!(std::io::stdout(), SetCursorStyle::DefaultUserShape);
        }
        self.persist_session();
        Ok(())
    }

    /// Terminal cursor shape for the current state: `None` = the editor paints its own.
    fn desired_cursor_shape(&self) -> Option<SetCursorStyle> {
        use config::CursorShape as C;
        let e = &self.cfg.editor;
        if e.cursor == C::Drawn || self.settings.is_some() || self.overlay != Overlay::None {
            return None;
        }
        let tab = self.active_tab()?;
        if self.focus != Pane::Editor || tab.preview {
            return None;
        }
        // Vim: the shape follows the mode; emacs: the configured shape.
        let shape = match (self.keys, tab.editor.mode) {
            (EditorKeys::Vim, EditorMode::Insert | EditorMode::Search) => C::Bar,
            (EditorKeys::Vim, EditorMode::Normal) => C::Block,
            (EditorKeys::Vim, EditorMode::Visual) => C::Underline,
            (EditorKeys::Emacs, _) => e.cursor,
        };
        Some(match (shape, e.blink) {
            (C::Bar, true) => SetCursorStyle::BlinkingBar,
            (C::Bar, false) => SetCursorStyle::SteadyBar,
            (C::Underline, true) => SetCursorStyle::BlinkingUnderScore,
            (C::Underline, false) => SetCursorStyle::SteadyUnderScore,
            (_, true) => SetCursorStyle::BlinkingBlock,
            (_, false) => SetCursorStyle::SteadyBlock,
        })
    }

    /// Emit DECSCUSR only when the shape changes.
    fn apply_cursor_shape(&mut self) {
        let want = self.desired_cursor_shape();
        if want == self.cursor_shape {
            return;
        }
        let style = want.unwrap_or(SetCursorStyle::DefaultUserShape);
        let _ = execute!(std::io::stdout(), style);
        self.cursor_shape = want;
    }

    pub fn active_notes(&self) -> &[Note] {
        if self.filter == Filter::Trash {
            &self.store.trash
        } else {
            &self.store.notes
        }
    }

    pub fn active_tab(&self) -> Option<&Tab> {
        self.tabs.get(self.active)
    }

    pub fn active_tab_mut(&mut self) -> Option<&mut Tab> {
        self.tabs.get_mut(self.active)
    }

    /// `(note, in_trash)` for a path known to the store.
    pub fn note_by_path(&self, path: &Path) -> Option<(&Note, bool)> {
        self.store
            .notes
            .iter()
            .find(|n| n.path == path)
            .map(|n| (n, false))
            .or_else(|| {
                self.store
                    .trash
                    .iter()
                    .find(|n| n.path == path)
                    .map(|n| (n, true))
            })
    }

    pub fn tab_note(&self, i: usize) -> Option<&Note> {
        self.tabs
            .get(i)
            .and_then(|t| self.note_by_path(&t.path))
            .map(|(n, _)| n)
    }

    pub fn current_note(&self) -> Option<&Note> {
        self.tab_note(self.active)
    }

    pub fn tab_in_trash(&self, i: usize) -> bool {
        self.tabs
            .get(i)
            .and_then(|t| self.note_by_path(&t.path))
            .is_some_and(|(_, t)| t)
    }

    pub fn query_text(&self) -> String {
        self.query.lines.to_string()
    }

    pub fn status_text(&self) -> Option<(StatusKind, &str)> {
        self.status.as_ref().map(|s| (s.kind, s.text.as_str()))
    }

    /// Human label of the current filter for the status bar.
    pub fn filter_label(&self) -> String {
        match &self.filter {
            Filter::All => "All notes".to_string(),
            Filter::Trash => "trash".to_string(),
            Filter::Tag(t) => format!("#{t}"),
            Filter::Folder(d) => self.store.folder_label(d),
        }
    }

    fn set_status(&mut self, kind: StatusKind, text: String) {
        self.set_status_ttl(kind, text, STATUS_TTL);
    }

    fn set_status_ttl(&mut self, kind: StatusKind, text: String, ttl: Duration) {
        self.status = Some(Status {
            kind,
            text,
            at: Instant::now(),
            ttl,
        });
    }

    fn fail(&mut self, what: &str, e: impl std::fmt::Display) {
        self.set_status(StatusKind::Failed, format!("{what} failed: {e}"));
    }

    fn new_editor(&self, text: &str) -> EditorState {
        let mut editor = EditorState::new(Lines::from(text));
        if self.keys == EditorKeys::Emacs {
            editor.mode = EditorMode::Insert;
        }
        markdown::refresh(&mut editor);
        editor
    }

    fn folder_exists(&self, dir: &Path) -> bool {
        self.store.roots.iter().any(|r| r.dir == dir) || self.store.folders.iter().any(|f| f == dir)
    }

    /// Rebuild tree rows and the visible list after any store/filter/query change.
    fn refresh(&mut self) {
        match &self.filter {
            Filter::Tag(t) => {
                let exists = self
                    .store
                    .notes
                    .iter()
                    .any(|n| n.tags.iter().any(|tag| index::is_under(tag, t)));
                if !exists {
                    self.filter = Filter::All;
                }
            }
            Filter::Folder(d) if !self.folder_exists(d) => {
                self.filter = Filter::All;
            }
            _ => {}
        }
        let prev = self.tree_rows.get(self.tree_sel).cloned();
        let mut rows = vec![TreeRow::All];
        rows.extend(
            build_folder_tree(&self.store, &self.collapsed)
                .into_iter()
                .map(TreeRow::Folder),
        );
        // Trash only shows once it has something (or while it is the active filter).
        if !self.store.trash.is_empty() || self.filter == Filter::Trash {
            rows.push(TreeRow::Trash);
        }
        let tags = build_tag_tree(&self.store.notes, &self.collapsed);
        if !tags.rows.is_empty() {
            rows.push(TreeRow::Header(""));
            rows.push(TreeRow::Header("tags"));
            if !self.collapsed.contains(TAGS_KEY) {
                rows.extend(tags.rows.into_iter().map(TreeRow::Tag));
            }
        }
        self.tree_sel = prev
            .and_then(|p| rows.iter().position(|r| r.same_as(&p)))
            .unwrap_or(0)
            .min(rows.len() - 1);
        self.tree_rows = rows;

        let query = self.query_text();
        let notes = if self.filter == Filter::Trash {
            &self.store.trash
        } else {
            &self.store.notes
        };
        let mut visible = index::visible(notes, &self.filter, &query, &mut self.matcher);
        if query.trim().is_empty() {
            match self.sort {
                SortMode::Modified => {}
                SortMode::Title => visible.sort_by(|&a, &b| {
                    notes[a]
                        .title
                        .to_lowercase()
                        .cmp(&notes[b].title.to_lowercase())
                        .then_with(|| notes[b].modified.cmp(&notes[a].modified))
                }),
                SortMode::Created => visible.sort_by(|&a, &b| {
                    notes[b]
                        .created
                        .cmp(&notes[a].created)
                        .then_with(|| notes[b].modified.cmp(&notes[a].modified))
                }),
            }
        }
        self.visible = visible;
        let cur = self
            .active_tab()
            .and_then(|t| self.active_notes().iter().position(|n| n.path == t.path));
        self.list_sel = cur
            .and_then(|c| self.visible.iter().position(|&i| i == c))
            .unwrap_or(self.list_sel)
            .min(self.visible.len().saturating_sub(1));
    }
}
