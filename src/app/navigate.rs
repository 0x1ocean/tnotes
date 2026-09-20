//! Tree/list cursor movement, filters, folds, sort and focus.

use super::*;

#[derive(Debug, Clone)]
pub enum TreeRow {
    All,
    Folder(FolderRow),
    Trash,
    Header(&'static str),
    Tag(TagRow),
}

impl TreeRow {
    pub fn selectable(&self) -> bool {
        !matches!(self, TreeRow::Header(""))
    }

    pub fn is_tags_header(&self) -> bool {
        matches!(self, TreeRow::Header("tags"))
    }

    /// Identity (ignores counts).
    pub(super) fn same_as(&self, other: &TreeRow) -> bool {
        match (self, other) {
            (TreeRow::All, TreeRow::All) | (TreeRow::Trash, TreeRow::Trash) => true,
            (TreeRow::Header(a), TreeRow::Header(b)) => a == b,
            (TreeRow::Folder(a), TreeRow::Folder(b)) => a.dir == b.dir,
            (TreeRow::Tag(a), TreeRow::Tag(b)) => a.path == b.path,
            _ => false,
        }
    }

    pub fn filter(&self) -> Option<Filter> {
        match self {
            TreeRow::All => Some(Filter::All),
            TreeRow::Trash => Some(Filter::Trash),
            TreeRow::Folder(f) => Some(Filter::Folder(f.dir.clone())),
            TreeRow::Tag(t) => Some(Filter::Tag(t.path.clone())),
            TreeRow::Header(_) => None,
        }
    }
}

impl App {
    pub(super) fn select_list(&mut self, i: usize) {
        if self.visible.is_empty() {
            self.list_sel = 0;
            return;
        }
        let i = i.min(self.visible.len() - 1);
        self.list_sel = i;
        let path = self.active_notes()[self.visible[i]].path.clone();
        self.show_note(path, false);
    }

    pub(super) fn move_list(&mut self, delta: isize) {
        let i = (self.list_sel as isize + delta).max(0) as usize;
        self.select_list(i);
    }

    pub(super) fn move_tree(&mut self, delta: isize) {
        let n = self.tree_rows.len();
        if n == 0 {
            return;
        }
        let step = delta.signum();
        let mut i = self.tree_sel as isize;
        for _ in 0..delta.abs() {
            let mut j = i + step;
            while j >= 0 && (j as usize) < n && !self.tree_rows[j as usize].selectable() {
                j += step;
            }
            if j < 0 || j as usize >= n {
                break;
            }
            i = j;
        }
        self.tree_sel = i as usize;
    }

    pub(super) fn set_filter(&mut self, f: Filter) {
        self.filter = f;
        self.list_sel = 0;
        self.list_offset = 0;
        self.refresh();
        self.persist_session();
    }

    pub(super) fn apply_tree(&mut self, i: usize, focus_list: bool) {
        if self.tree_rows.get(i).is_some_and(TreeRow::is_tags_header) {
            self.tree_sel = i;
            self.toggle_collapse(i);
            return;
        }
        let Some(f) = self.tree_rows.get(i).and_then(TreeRow::filter) else {
            return;
        };
        self.tree_sel = i;
        self.set_filter(f);
        if focus_list {
            self.focus = Pane::List;
        }
    }

    pub(super) fn toggle_collapse(&mut self, i: usize) {
        let key = match self.tree_rows.get(i) {
            Some(TreeRow::Folder(f)) if f.has_children => dir_key(&f.dir),
            Some(TreeRow::Tag(t)) if t.has_children => tag_key(&t.path),
            Some(TreeRow::Header("tags")) => TAGS_KEY.to_string(),
            _ => return,
        };
        if !self.collapsed.remove(&key) {
            self.collapsed.insert(key);
        }
        self.refresh();
        self.persist_session();
    }

    pub(super) fn tree_folder(&self) -> Option<PathBuf> {
        match self.tree_rows.get(self.tree_sel) {
            Some(TreeRow::Folder(f)) => Some(f.dir.clone()),
            _ => None,
        }
    }

    pub(super) fn is_root(&self, dir: &Path) -> bool {
        self.store.roots.iter().any(|r| r.dir == dir)
    }

    pub(super) fn cycle_sort(&mut self) {
        self.sort = match self.sort {
            SortMode::Modified => SortMode::Title,
            SortMode::Title => SortMode::Created,
            SortMode::Created => SortMode::Modified,
        };
        self.refresh();
        self.persist_session();
    }

    pub(super) fn cycle_focus(&mut self, forward: bool) {
        if !self.sidebar {
            self.focus = Pane::Editor;
            self.search_active = false;
            self.tag_popup = None;
            return;
        }
        const ORDER: [Pane; 3] = [Pane::Tree, Pane::List, Pane::Editor];
        let pos = ORDER.iter().position(|&p| p == self.focus).unwrap_or(0);
        let next = if forward {
            (pos + 1) % 3
        } else {
            (pos + 2) % 3
        };
        self.focus = ORDER[next];
        self.search_active = false;
        self.tag_popup = None;
    }

    /// Footer `<<`/`>>`: wide layout hides/shows the sidebar; narrow layout switches
    /// between the navigator and the editor screen.
    pub(super) fn toggle_sidebar(&mut self) {
        if self.layout == Layout::Narrow {
            self.sidebar = true;
            self.focus = if self.focus == Pane::Editor {
                Pane::List
            } else if self.tabs.is_empty() {
                return;
            } else {
                Pane::Editor
            };
            self.search_active = false;
            self.tag_popup = None;
            return;
        }
        self.sidebar = !self.sidebar;
        if !self.sidebar && self.focus != Pane::Editor {
            self.focus = Pane::Editor;
            self.search_active = false;
        } else if self.sidebar && self.tabs.is_empty() {
            self.focus = Pane::List;
        }
        self.tag_popup = None;
        self.persist_session();
    }

    /// Glyph for the footer switch, matching `toggle_sidebar`.
    pub fn panel_switch_glyph(&self) -> &'static str {
        match (self.layout, self.focus == Pane::Editor, self.sidebar) {
            (Layout::Narrow, true, _) => "<<",
            (Layout::Narrow, false, _) if !self.tabs.is_empty() => ">>",
            (Layout::Narrow, false, _) => "  ",
            (Layout::Wide, _, true) => "<<",
            (Layout::Wide, _, false) => ">>",
        }
    }

    /// Filter by `tag` (if any note carries it) and show it in the tree.
    pub(super) fn filter_by_tag(&mut self, tag: &str) {
        let exists = self
            .store
            .notes
            .iter()
            .any(|n| n.tags.iter().any(|t| index::is_under(t, tag)));
        if !exists {
            return;
        }
        if !self.sidebar {
            self.toggle_sidebar();
        }
        self.collapsed.remove(TAGS_KEY);
        self.set_filter(Filter::Tag(tag.to_string()));
        if let Some(i) = self
            .tree_rows
            .iter()
            .position(|r| r.filter().as_ref() == Some(&self.filter))
        {
            self.tree_sel = i;
        }
        self.set_status(StatusKind::Info, format!("showing #{tag}"));
    }
}
