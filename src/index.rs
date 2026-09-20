use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};

use nucleo_matcher::pattern::{CaseMatching, Normalization, Pattern};
use nucleo_matcher::{Matcher, Utf32Str};

use crate::note::Note;
use crate::store::Store;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagRow {
    pub path: String,
    pub name: String,
    pub depth: usize,
    pub count: usize,
    pub has_children: bool,
}

#[derive(Debug, Default)]
pub struct TagTree {
    pub rows: Vec<TagRow>,
}

pub fn is_under(tag: &str, prefix: &str) -> bool {
    tag == prefix
        || (tag.len() > prefix.len()
            && tag.starts_with(prefix)
            && tag.as_bytes()[prefix.len()] == b'/')
}

pub fn tag_key(path: &str) -> String {
    format!("tag:{path}")
}

pub fn dir_key(dir: &Path) -> String {
    format!("dir:{}", dir.display())
}

/// Every tag path plus all its prefixes; count = notes with the tag or any descendant.
/// `collapsed` holds `tag:{path}` keys.
pub fn build_tag_tree(notes: &[Note], collapsed: &HashSet<String>) -> TagTree {
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for note in notes {
        let mut seen: HashSet<&str> = HashSet::new();
        for tag in &note.tags {
            let ends = tag
                .match_indices('/')
                .map(|(i, _)| i)
                .chain(std::iter::once(tag.len()));
            for end in ends {
                let prefix = &tag[..end];
                if seen.insert(prefix) {
                    *counts.entry(prefix.to_string()).or_default() += 1;
                }
            }
        }
    }
    let paths: Vec<&String> = counts.keys().collect();
    let mut rows = Vec::with_capacity(paths.len());
    let mut hidden_prefix: Option<String> = None;
    for (i, path) in paths.iter().enumerate() {
        if let Some(h) = &hidden_prefix {
            if is_under(path, h) && path.as_str() != h.as_str() {
                continue;
            }
            hidden_prefix = None;
        }
        let depth = path.matches('/').count();
        let name = path.rsplit('/').next().unwrap_or(path).to_string();
        let has_children = paths.get(i + 1).is_some_and(|n| is_under(n, path));
        if has_children && collapsed.contains(&tag_key(path)) {
            hidden_prefix = Some(path.to_string());
        }
        rows.push(TagRow {
            path: path.to_string(),
            name,
            depth,
            count: counts[path.as_str()],
            has_children,
        });
    }
    TagTree { rows }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FolderRow {
    pub dir: PathBuf,
    pub name: String,
    pub depth: usize,
    pub count: usize,
    pub has_children: bool,
    pub root: usize,
}

/// Roots at depth 0 followed by their (non-collapsed) subfolders; `collapsed` holds `dir:{abs}` keys.
pub fn build_folder_tree(store: &Store, collapsed: &HashSet<String>) -> Vec<FolderRow> {
    fn push(
        store: &Store,
        collapsed: &HashSet<String>,
        rows: &mut Vec<FolderRow>,
        dir: &Path,
        name: String,
        depth: usize,
        root: usize,
    ) {
        let children: Vec<&PathBuf> = store
            .folders
            .iter()
            .filter(|f| f.parent() == Some(dir))
            .collect();
        let count = store
            .notes
            .iter()
            .filter(|n| n.path.starts_with(dir))
            .count();
        let has_children = !children.is_empty();
        rows.push(FolderRow {
            dir: dir.to_path_buf(),
            name,
            depth,
            count,
            has_children,
            root,
        });
        if has_children && collapsed.contains(&dir_key(dir)) {
            return;
        }
        for c in children {
            let name = c
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();
            push(store, collapsed, rows, c, name, depth + 1, root);
        }
    }
    let mut rows = Vec::new();
    for (i, r) in store.roots.iter().enumerate() {
        push(store, collapsed, &mut rows, &r.dir, r.name.clone(), 0, i);
    }
    rows
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Filter {
    All,
    Trash,
    Tag(String),
    /// Absolute directory (a root or a subfolder).
    Folder(PathBuf),
}

/// Indices into `notes` matching `filter` and `query`, best first.
pub fn visible(notes: &[Note], filter: &Filter, query: &str, matcher: &mut Matcher) -> Vec<usize> {
    let mut idx: Vec<usize> = (0..notes.len())
        .filter(|&i| match filter {
            Filter::All | Filter::Trash => true,
            Filter::Tag(t) => notes[i].tags.iter().any(|tag| is_under(tag, t)),
            Filter::Folder(d) => notes[i].path.starts_with(d),
        })
        .collect();
    let query = query.trim();
    if query.is_empty() {
        idx.sort_by(|&a, &b| notes[b].modified.cmp(&notes[a].modified));
        return idx;
    }
    let pattern = Pattern::parse(query, CaseMatching::Ignore, Normalization::Smart);
    let mut buf = Vec::new();
    let mut scored: Vec<(u32, usize)> = idx
        .into_iter()
        .filter_map(|i| {
            let hay = format!("{}\n{}", notes[i].title, notes[i].text);
            pattern
                .score(Utf32Str::new(&hay, &mut buf), matcher)
                .map(|s| (s, i))
        })
        .collect();
    scored.sort_by(|a, b| {
        b.0.cmp(&a.0)
            .then_with(|| notes[b.1].modified.cmp(&notes[a.1].modified))
    });
    scored.into_iter().map(|(_, i)| i).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::Root;
    use std::time::SystemTime;

    fn note(text: &str) -> Note {
        note_at("/x.md", text)
    }

    fn note_at(path: &str, text: &str) -> Note {
        let t = SystemTime::UNIX_EPOCH;
        Note::from_text(0, path.into(), text.into(), t, t)
    }

    #[test]
    fn tree_has_prefixes_and_counts_descendants() {
        let notes = vec![
            note("#work/project"),
            note("#work"),
            note("#work/meeting/x"),
        ];
        let tree = build_tag_tree(&notes, &HashSet::new());
        let paths: Vec<_> = tree
            .rows
            .iter()
            .map(|r| (r.path.as_str(), r.count, r.depth, r.has_children))
            .collect();
        assert_eq!(
            paths,
            vec![
                ("work", 3, 0, true),
                ("work/meeting", 1, 1, true),
                ("work/meeting/x", 1, 2, false),
                ("work/project", 1, 1, false),
            ]
        );
    }

    #[test]
    fn collapsed_hides_descendants() {
        let notes = vec![note("#a/b/c #d")];
        let collapsed: HashSet<String> = [tag_key("a")].into();
        let tree = build_tag_tree(&notes, &collapsed);
        let paths: Vec<_> = tree.rows.iter().map(|r| r.path.as_str()).collect();
        assert_eq!(paths, vec!["a", "d"]);
    }

    #[test]
    fn tag_filter_matches_descendants_not_siblings() {
        let notes = vec![note("#work/project"), note("#workshop"), note("#home")];
        let mut m = Matcher::new(nucleo_matcher::Config::DEFAULT);
        assert_eq!(
            visible(&notes, &Filter::Tag("work".into()), "", &mut m),
            vec![0]
        );
        assert_eq!(visible(&notes, &Filter::All, "home", &mut m), vec![2]);
        assert_eq!(
            visible(&notes, &Filter::All, "zzz", &mut m),
            Vec::<usize>::new()
        );
    }

    #[test]
    fn folder_tree_and_filter() {
        let store = Store {
            roots: vec![
                Root {
                    dir: "/r/a".into(),
                    name: "a".into(),
                },
                Root {
                    dir: "/r/b".into(),
                    name: "b".into(),
                },
            ],
            notes: vec![
                note_at("/r/a/work/p.md", "# P"),
                note_at("/r/a/workshop/q.md", "# Q"),
                note_at("/r/b/z.md", "# Z"),
            ],
            trash: vec![],
            folders: vec![
                "/r/a/work".into(),
                "/r/a/work/sub".into(),
                "/r/a/workshop".into(),
            ],
        };
        let rows = build_folder_tree(&store, &HashSet::new());
        let got: Vec<_> = rows
            .iter()
            .map(|r| (r.name.as_str(), r.depth, r.count, r.has_children))
            .collect();
        assert_eq!(
            got,
            vec![
                ("a", 0, 2, true),
                ("work", 1, 1, true),
                ("sub", 2, 0, false),
                ("workshop", 1, 1, false),
                ("b", 0, 1, false)
            ]
        );
        let collapsed: HashSet<String> = [dir_key(Path::new("/r/a"))].into();
        let rows = build_folder_tree(&store, &collapsed);
        let got: Vec<_> = rows.iter().map(|r| r.name.as_str()).collect();
        assert_eq!(got, vec!["a", "b"]);

        let mut m = Matcher::new(nucleo_matcher::Config::DEFAULT);
        assert_eq!(
            visible(
                &store.notes,
                &Filter::Folder("/r/a/work".into()),
                "",
                &mut m
            ),
            vec![0]
        );
        assert_eq!(
            visible(&store.notes, &Filter::Folder("/r/a".into()), "", &mut m),
            vec![0, 1]
        );
    }
}
