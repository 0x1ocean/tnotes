//! Session state (`state.toml`): restore on launch, persist on change.

use super::*;

impl App {
    pub(super) fn restore_session(&mut self) {
        let s: Session = session::load();
        self.collapsed = s.collapsed.into_iter().collect();
        self.sort = SortMode::parse(&s.sort);
        self.filter = match s.filter.as_str() {
            "trash" => Filter::Trash,
            f if f.starts_with("folder:") => Filter::Folder(PathBuf::from(&f["folder:".len()..])),
            f if f.starts_with("tag:") => Filter::Tag(f["tag:".len()..].to_string()),
            _ => Filter::All,
        };
        self.refresh();
        for (i, p) in s.tabs.into_iter().enumerate() {
            self.push_tab(p, s.pinned.get(i).copied().unwrap_or(true));
        }
        self.active = s.active.min(self.tabs.len().saturating_sub(1));
        self.sidebar = s.sidebar;
        self.tree_folded = s.tree_folded;
        self.focus = match s.focus.as_str() {
            "tree" => Pane::Tree,
            "editor" if !self.tabs.is_empty() => Pane::Editor,
            _ => Pane::List,
        };
        if let Some(i) = self
            .tree_rows
            .iter()
            .position(|r| r.filter().as_ref() == Some(&self.filter))
        {
            self.tree_sel = i;
        }
        self.refresh();
        if let Some(root) = self.start_filter.take() {
            self.select_root_in_tree(&root);
            self.focus = Pane::List;
        }
        self.session_ready = true;
        if self.cfg.first_run && self.cfg.roots.is_empty() {
            self.open_first_run();
        }
    }

    pub(super) fn persist_session(&mut self) {
        if !self.session_ready {
            return;
        }
        let s = Session {
            tabs: self.tabs.iter().map(|t| t.path.clone()).collect(),
            pinned: self.tabs.iter().map(|t| t.pinned).collect(),
            active: self.active,
            filter: match &self.filter {
                Filter::All => "all".to_string(),
                Filter::Trash => "trash".to_string(),
                Filter::Folder(d) => format!("folder:{}", d.display()),
                Filter::Tag(t) => format!("tag:{t}"),
            },
            collapsed: self.collapsed.iter().cloned().collect(),
            sidebar: self.sidebar,
            tree_folded: self.tree_folded,
            sort: self.sort.name().to_string(),
            focus: match self.focus {
                Pane::Tree => "tree",
                Pane::List => "list",
                Pane::Editor => "editor",
            }
            .to_string(),
        };
        if let Err(e) = session::save(&s)
            && !self.session_warned
        {
            self.session_warned = true;
            self.set_status(StatusKind::Failed, format!("session not saved: {e}"));
        }
    }
}
