//! Folder browser used to add a root: breadcrumb, type-to-filter, `.md` counts, one
//! explicit "add this folder" row. Keyboard and mouse are equivalent.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use super::*;

/// Recursive `*.md` count is capped so huge trees stay cheap.
const COUNT_CAP: usize = 999;
const COUNT_DEPTH: usize = 4;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DirRow {
    /// Add the directory currently shown.
    AddHere,
    /// Go to the parent.
    Up,
    Dir(PathBuf),
    /// Create a subfolder here and add it.
    NewFolder,
    /// First run only: create the default folder (`~/Documents/notes`).
    CreateDefault,
}

pub struct Browser {
    pub dir: PathBuf,
    pub query: EditorState,
    pub sel: usize,
    pub rows: Vec<DirRow>,
    pub show_hidden: bool,
    /// First-run mode: title changes and a `create default` row is offered.
    pub first_run: bool,
    /// Breadcrumb targets in display order (`/`, `~`, …).
    pub crumbs: Vec<PathBuf>,
    counts: HashMap<PathBuf, usize>,
}

impl Default for Browser {
    fn default() -> Self {
        Browser {
            dir: PathBuf::from("/"),
            query: single_line(""),
            sel: 0,
            rows: Vec::new(),
            show_hidden: false,
            first_run: false,
            crumbs: Vec::new(),
            counts: HashMap::new(),
        }
    }
}

/// Count markdown files under `dir` (skipping dot-dirs), capped.
fn count_md(dir: &Path, depth: usize, budget: &mut usize) -> usize {
    if depth == 0 || *budget == 0 {
        return 0;
    }
    let Ok(rd) = std::fs::read_dir(dir) else {
        return 0;
    };
    let mut n = 0;
    for e in rd.flatten() {
        if *budget == 0 {
            break;
        }
        *budget -= 1;
        let name = e.file_name();
        if name.to_string_lossy().starts_with('.') {
            continue;
        }
        let Ok(ft) = e.file_type() else { continue };
        if ft.is_dir() {
            n += count_md(&e.path(), depth - 1, budget);
        } else if ft.is_file() && e.path().extension().is_some_and(|x| x == "md") {
            n += 1;
        }
        if n >= COUNT_CAP {
            return COUNT_CAP;
        }
    }
    n
}

impl Browser {
    /// Cached `.md` count for a listed directory.
    pub fn count(&mut self, dir: &Path) -> usize {
        if let Some(&n) = self.counts.get(dir) {
            return n;
        }
        let mut budget = 4000;
        let n = count_md(dir, COUNT_DEPTH, &mut budget);
        self.counts.insert(dir.to_path_buf(), n);
        n
    }

    pub fn count_label(n: usize) -> String {
        match n {
            0 => String::new(),
            COUNT_CAP.. => format!("{COUNT_CAP}+ notes"),
            1 => "1 note".into(),
            n => format!("{n} notes"),
        }
    }

    pub fn query_text(&self) -> String {
        self.query.lines.to_string()
    }

    /// Rebuild rows for `dir`, filtered by the query (fuzzy via `matcher`).
    pub fn refresh(&mut self, matcher: &mut Matcher) {
        let mut dirs: Vec<PathBuf> = std::fs::read_dir(&self.dir)
            .map(|rd| {
                rd.flatten()
                    .filter(|e| e.file_type().is_ok_and(|t| t.is_dir()))
                    .map(|e| e.path())
                    .filter(|p| {
                        self.show_hidden
                            || !p
                                .file_name()
                                .is_some_and(|n| n.to_string_lossy().starts_with('.'))
                    })
                    .collect()
            })
            .unwrap_or_default();
        let query = self.query_text();
        let query = query.trim().to_string();
        if query.is_empty() {
            dirs.sort_by_key(|p| p.file_name().map(|n| n.to_string_lossy().to_lowercase()));
        } else {
            let pattern = Pattern::parse(&query, CaseMatching::Ignore, Normalization::Smart);
            let mut buf = Vec::new();
            let mut scored: Vec<(u32, PathBuf)> = dirs
                .into_iter()
                .filter_map(|p| {
                    let name = p.file_name()?.to_string_lossy().into_owned();
                    pattern
                        .score(Utf32Str::new(&name, &mut buf), matcher)
                        .map(|s| (s, p))
                })
                .collect();
            scored.sort_by_key(|(s, _)| std::cmp::Reverse(*s));
            dirs = scored.into_iter().map(|(_, p)| p).collect();
        }
        let mut rows = Vec::new();
        if self.first_run {
            rows.push(DirRow::CreateDefault);
        }
        rows.push(DirRow::AddHere);
        if self.dir.parent().is_some() {
            rows.push(DirRow::Up);
        }
        rows.extend(dirs.into_iter().map(DirRow::Dir));
        rows.push(DirRow::NewFolder);
        self.rows = rows;
        self.sel = self.sel.min(self.rows.len() - 1);

        // Breadcrumb: `/`, then each component; the home prefix collapses to `~`.
        self.crumbs.clear();
        let mut acc = PathBuf::new();
        for c in self.dir.components() {
            acc.push(c);
            self.crumbs.push(acc.clone());
        }
    }

    pub fn crumb_label(p: &Path) -> String {
        if let Some(home) = dirs::home_dir()
            && p == home
        {
            return "~".into();
        }
        match p.file_name() {
            Some(n) => n.to_string_lossy().into_owned(),
            None => "/".into(),
        }
    }

    fn go(&mut self, dir: PathBuf, matcher: &mut Matcher) {
        self.dir = dir;
        self.query = single_line("");
        self.sel = 0;
        self.refresh(matcher);
        // Land on the first real directory when there is one.
        if let Some(i) = self.rows.iter().position(|r| matches!(r, DirRow::Dir(_))) {
            self.sel = i;
        }
    }
}

impl App {
    /// First launch: ask where the notes live.
    pub(super) fn open_first_run(&mut self) {
        self.browser.first_run = true;
        self.open_browser();
    }

    /// Open the browser at the parent of the last root (or `~`).
    pub(super) fn open_browser(&mut self) {
        if self.cfg.roots_fixed {
            return;
        }
        let start = self
            .cfg
            .roots
            .last()
            .and_then(|r| r.parent().map(Path::to_path_buf))
            .or_else(dirs::home_dir)
            .unwrap_or_else(|| PathBuf::from("/"));
        self.browser.show_hidden = false;
        self.browser.go(start, &mut self.matcher);
        self.overlay = Overlay::Browse;
    }

    fn browser_go(&mut self, dir: PathBuf) {
        if dir.is_dir() {
            self.browser.go(dir, &mut self.matcher);
        } else {
            self.set_status(
                StatusKind::Failed,
                format!("not a folder: {}", dir.display()),
            );
        }
    }

    fn browser_up(&mut self) {
        if let Some(p) = self.browser.dir.parent().map(Path::to_path_buf) {
            let from = self.browser.dir.clone();
            self.browser_go(p);
            // Keep the cursor on the folder we came from.
            if let Some(i) = self
                .browser
                .rows
                .iter()
                .position(|r| matches!(r, DirRow::Dir(d) if *d == from))
            {
                self.browser.sel = i;
            }
        }
    }

    fn browser_run(&mut self, i: usize) {
        let Some(row) = self.browser.rows.get(i).cloned() else {
            return;
        };
        match row {
            DirRow::AddHere => {
                let dir = self.browser.dir.clone();
                self.overlay = Overlay::None;
                self.add_root(&dir.display().to_string());
            }
            DirRow::Up => self.browser_up(),
            DirRow::Dir(d) => self.browser_go(d),
            DirRow::CreateDefault => {
                let dir = config::default_notes_dir();
                self.overlay = Overlay::None;
                self.add_root(&dir.display().to_string());
            }
            DirRow::NewFolder => {
                self.prompt = single_line("");
                self.overlay = Overlay::Prompt(PromptKind::NewRoot(self.browser.dir.clone()));
            }
        }
    }

    pub(super) fn browser_key(&mut self, k: KeyEvent) {
        let ctrl = k.modifiers.contains(KeyModifiers::CONTROL);
        let alt = k.modifiers.contains(KeyModifiers::ALT);
        let n = self.browser.rows.len().max(1);
        let empty = self.browser.query_text().is_empty();
        match k.code {
            KeyCode::Esc => self.close_browser(),
            KeyCode::Enter => {
                let q = self.browser.query_text();
                let q = q.trim();
                if q.starts_with('/') || q.starts_with('~') {
                    // A typed path jumps straight there.
                    self.browser_go(config::expand_tilde(q));
                } else {
                    self.browser_run(self.browser.sel);
                }
            }
            KeyCode::Tab => {
                if let Some(i) = self.browser.rows.iter().position(|r| *r == DirRow::AddHere) {
                    self.browser_run(i);
                }
            }
            KeyCode::Right => {
                if let Some(DirRow::Dir(d)) = self.browser.rows.get(self.browser.sel).cloned() {
                    self.browser_go(d);
                }
            }
            KeyCode::Left => self.browser_up(),
            KeyCode::Backspace if empty => self.browser_up(),
            KeyCode::Down => self.browser.sel = (self.browser.sel + 1) % n,
            KeyCode::Up => self.browser.sel = (self.browser.sel + n - 1) % n,
            KeyCode::Char('n' | 'j') if ctrl => self.browser.sel = (self.browser.sel + 1) % n,
            KeyCode::Char('p' | 'k') if ctrl => self.browser.sel = (self.browser.sel + n - 1) % n,
            KeyCode::PageDown => self.browser.sel = (self.browser.sel + 10).min(n - 1),
            KeyCode::PageUp => self.browser.sel = self.browser.sel.saturating_sub(10),
            KeyCode::Char('.') if alt => {
                self.browser.show_hidden = !self.browser.show_hidden;
                self.browser.refresh(&mut self.matcher);
            }
            KeyCode::Char('~') if empty => {
                if let Some(h) = dirs::home_dir() {
                    self.browser_go(h);
                }
            }
            _ => {
                self.query_handler
                    .on_event(Event::Key(k), &mut self.browser.query);
                self.browser.refresh(&mut self.matcher);
                // Typing narrows the list: jump to the first match.
                if let Some(i) = self
                    .browser
                    .rows
                    .iter()
                    .position(|r| matches!(r, DirRow::Dir(_)))
                {
                    self.browser.sel = i;
                }
            }
        }
    }

    pub(super) fn browser_mouse(&mut self, m: MouseEvent, pos: Position) {
        match m.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                if let Some(i) = self.hits.crumbs.iter().position(|r| r.contains(pos)) {
                    if let Some(p) = self.browser.crumbs.get(i).cloned() {
                        self.browser_go(p);
                    }
                } else if let Some(i) = self.hits.overlay_row(pos) {
                    self.browser_run(i);
                } else if !self.hits.overlay.contains(pos) {
                    self.close_browser();
                }
            }
            MouseEventKind::ScrollDown => {
                let n = self.browser.rows.len().max(1);
                self.browser.sel = (self.browser.sel + 1).min(n - 1);
            }
            MouseEventKind::ScrollUp => self.browser.sel = self.browser.sel.saturating_sub(1),
            _ => {}
        }
    }

    /// Leaving the browser on first run without choosing falls back to the default folder.
    fn close_browser(&mut self) {
        self.overlay = Overlay::None;
        if self.cfg.roots.is_empty() {
            let dir = config::default_notes_dir();
            self.add_root(&dir.display().to_string());
        }
        self.browser.first_run = false;
    }

    /// Prompt result for `NewFolder` inside the browser: create and add.
    pub(super) fn add_new_root(&mut self, parent: &Path, name: &str) {
        let name = name.trim();
        if name.is_empty() || name.contains('/') {
            self.overlay = Overlay::Browse;
            return;
        }
        let dir = parent.join(name);
        self.overlay = Overlay::None;
        self.add_root(&dir.display().to_string());
    }
}
