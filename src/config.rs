use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use ratatui::style::Color;
use serde::{Deserialize, Serialize};

pub use crate::ui::theme::Palette;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EditorKeys {
    #[default]
    Emacs,
    Vim,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Dates {
    #[default]
    Relative,
    Absolute,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TabNumbers {
    #[default]
    Always,
    /// Only when two or more tabs are open.
    Multi,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Appearance {
    /// One-line list items instead of title + preview.
    pub compact: bool,
    pub dates: Dates,
    /// Note counts on the right of tree rows.
    pub counts: bool,
    pub sidebar_width: u16,
    pub tab_numbers: TabNumbers,
}

impl Default for Appearance {
    fn default() -> Self {
        Appearance {
            compact: false,
            dates: Dates::Relative,
            counts: true,
            sidebar_width: 32,
            tab_numbers: TabNumbers::Always,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Editor {
    pub keys: EditorKeys,
    /// Idle time before a dirty tab is written, in milliseconds.
    pub autosave_ms: u64,
    /// Initial text of a new note; the cursor lands at the end of its first line.
    pub template: String,
    pub wrap: bool,
    /// Maximum text column width in cells; `0` = full editor width.
    pub width: u16,
    /// Where the text column sits when narrower than the editor.
    pub align: Align,
    pub cursor: CursorShape,
    pub blink: bool,
}

impl Default for Editor {
    fn default() -> Self {
        Editor {
            keys: EditorKeys::Emacs,
            autosave_ms: 500,
            template: "# ".to_string(),
            wrap: true,
            width: 72,
            align: Align::Left,
            cursor: CursorShape::Drawn,
            blink: false,
        }
    }
}

pub const SIDEBAR_WIDTH_RANGE: std::ops::RangeInclusive<u16> = 24..=48;
pub const AUTOSAVE_RANGE: std::ops::RangeInclusive<u64> = 200..=3000;
/// Editor text width steps; `0` means full width.
pub const TEXT_WIDTH_RANGE: std::ops::RangeInclusive<u16> = 40..=160;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Align {
    #[default]
    Left,
    Center,
}

/// Editor cursor: `Drawn` is a reversed cell painted by the editor; the others use the
/// terminal's own cursor (theme colour, blink) and switch by vim mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CursorShape {
    #[default]
    Drawn,
    Bar,
    Underline,
    Block,
}

#[derive(Debug, Clone)]
pub struct Config {
    /// Canonical, deduplicated root folders in config order.
    pub roots: Vec<PathBuf>,
    /// `--dir` was given: roots are session-only and not editable.
    pub roots_fixed: bool,
    /// No config file yet: ask where the notes live.
    pub first_run: bool,
    pub appearance: Appearance,
    pub editor: Editor,
    pub theme: Palette,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
struct RawConfig {
    roots: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notes_dir: Option<String>,
    appearance: Appearance,
    editor: Editor,
    #[serde(skip_serializing_if = "RawTheme::is_empty")]
    theme: RawTheme,
}

/// Color names (`cyan`, `darkgray`, `lightblue`, …) or `#rrggbb`; unset → default.
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
struct RawTheme {
    #[serde(skip_serializing_if = "Option::is_none")]
    accent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dim: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    warn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    err: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ok: Option<String>,
}

fn color(raw: &Option<String>, fallback: Color) -> Result<Color> {
    match raw {
        Some(s) => s
            .parse::<Color>()
            .map_err(|_| anyhow::anyhow!("invalid theme color {s:?}")),
        None => Ok(fallback),
    }
}

impl RawTheme {
    fn is_empty(&self) -> bool {
        [&self.accent, &self.dim, &self.warn, &self.err, &self.ok]
            .iter()
            .all(|c| c.is_none())
    }

    fn palette(&self) -> Result<Palette> {
        let d = Palette::default();
        Ok(Palette {
            accent: color(&self.accent, d.accent)?,
            dim: color(&self.dim, d.dim)?,
            warn: color(&self.warn, d.warn)?,
            err: color(&self.err, d.err)?,
            ok: color(&self.ok, d.ok)?,
        })
    }
}

pub fn expand_tilde(p: &str) -> PathBuf {
    if p == "~"
        && let Some(home) = dirs::home_dir()
    {
        return home;
    }
    if let Some(rest) = p.strip_prefix("~/")
        && let Some(home) = dirs::home_dir()
    {
        return home.join(rest);
    }
    PathBuf::from(p)
}

pub fn contract_tilde(p: &Path) -> String {
    if let Some(home) = dirs::home_dir()
        && let Ok(rest) = p.strip_prefix(&home)
    {
        return if rest.as_os_str().is_empty() {
            "~".to_string()
        } else {
            format!("~/{}", rest.display())
        };
    }
    p.display().to_string()
}

pub fn default_notes_dir() -> PathBuf {
    dirs::document_dir()
        .or_else(dirs::home_dir)
        .unwrap_or_else(|| PathBuf::from("."))
        .join("notes")
}

pub fn config_path() -> Option<PathBuf> {
    dirs::config_dir().map(|d| d.join("tnotes").join("config.toml"))
}

fn read_raw(path: &Path) -> Result<RawConfig> {
    match std::fs::read_to_string(path) {
        Ok(s) => toml::from_str(&s).with_context(|| format!("invalid config {}", path.display())),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(RawConfig::default()),
        Err(e) => Err(e).with_context(|| format!("reading config {}", path.display())),
    }
}

/// Create (if needed) and canonicalize a root folder.
pub fn resolve_root(dir: &Path) -> Result<PathBuf> {
    std::fs::create_dir_all(dir)
        .with_context(|| format!("creating notes dir {}", dir.display()))?;
    dir.canonicalize()
        .with_context(|| format!("resolving notes dir {}", dir.display()))
}

/// Precedence: `--dir` > config `roots` > config `notes_dir`. With no config and no
/// `--dir` the roots stay empty and `first_run` is set so the app can ask where notes live.
pub fn load(cli_dir: Option<PathBuf>) -> Result<Config> {
    let path = config_path();
    let exists = path.as_ref().is_some_and(|p| p.exists());
    let raw = match &path {
        Some(p) => read_raw(p)?,
        None => RawConfig::default(),
    };
    let roots_fixed = cli_dir.is_some();
    let first_run = !roots_fixed && !exists;
    let dirs: Vec<PathBuf> = match cli_dir {
        Some(d) => vec![d],
        None if !raw.roots.is_empty() => raw.roots.iter().map(|r| expand_tilde(r)).collect(),
        None => raw
            .notes_dir
            .as_deref()
            .map(expand_tilde)
            .into_iter()
            .collect(),
    };
    let mut roots: Vec<PathBuf> = Vec::with_capacity(dirs.len());
    for d in &dirs {
        let r = resolve_root(d)?;
        // A root inside another root would load its notes twice; keep the outer one.
        if roots.iter().any(|x| r.starts_with(x)) {
            continue;
        }
        roots.retain(|x| !x.starts_with(&r));
        roots.push(r);
    }
    let mut appearance = raw.appearance;
    appearance.sidebar_width = appearance
        .sidebar_width
        .clamp(*SIDEBAR_WIDTH_RANGE.start(), *SIDEBAR_WIDTH_RANGE.end());
    let mut editor = raw.editor;
    editor.autosave_ms = editor
        .autosave_ms
        .clamp(*AUTOSAVE_RANGE.start(), *AUTOSAVE_RANGE.end());
    Ok(Config {
        roots,
        roots_fixed,
        first_run,
        appearance,
        editor,
        theme: raw.theme.palette()?,
    })
}

/// Write settings to `config.toml` (atomic). With `--dir` the on-disk roots are left untouched.
/// `[theme]` is preserved as written by the user.
pub fn save(cfg: &Config) -> Result<()> {
    let path = config_path().context("no config directory")?;
    let mut raw = read_raw(&path)?;
    if !cfg.roots_fixed {
        raw.roots = cfg.roots.iter().map(|r| contract_tilde(r)).collect();
        raw.notes_dir = None;
    }
    raw.appearance = cfg.appearance;
    raw.editor = cfg.editor.clone();
    let text = toml::to_string_pretty(&raw)?;
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).with_context(|| format!("creating {}", dir.display()))?;
    }
    crate::store::write_atomic(&path, &text)
}
