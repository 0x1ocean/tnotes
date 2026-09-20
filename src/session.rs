use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// UI state restored on launch: open tabs, filter, folds, sort, focus.
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Session {
    pub tabs: Vec<PathBuf>,
    pub pinned: Vec<bool>,
    pub active: usize,
    /// `all` | `trash` | `folder:{abs}` | `tag:{name}`
    pub filter: String,
    pub collapsed: Vec<String>,
    #[serde(default = "yes")]
    pub sidebar: bool,
    pub tree_folded: bool,
    /// `modified` | `title` | `created`
    pub sort: String,
    /// `tree` | `list` | `editor`
    pub focus: String,
}

fn yes() -> bool {
    true
}
pub fn path() -> Option<PathBuf> {
    dirs::state_dir()
        .or_else(dirs::data_local_dir)
        .map(|d| d.join("tnotes").join("state.toml"))
}

/// Missing or invalid state → `Default`.
pub fn load() -> Session {
    path()
        .and_then(|p| std::fs::read_to_string(p).ok())
        .and_then(|s| toml::from_str(&s).ok())
        .unwrap_or_default()
}

pub fn save(s: &Session) -> Result<()> {
    let path = path().context("no state directory")?;
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).with_context(|| format!("creating {}", dir.display()))?;
    }
    crate::store::write_atomic(&path, &toml::to_string_pretty(s)?)
}
