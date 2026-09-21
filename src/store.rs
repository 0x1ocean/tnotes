use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use anyhow::{Context, Result, bail};

use crate::note::{self, Note, link_key, slug_of, title_of};

pub const TRASH_DIR: &str = ".Trash";

pub struct Root {
    pub dir: PathBuf,
    /// Last path component; ` (parent)` suffix when two roots share a name.
    pub name: String,
}

pub struct Store {
    pub roots: Vec<Root>,
    pub notes: Vec<Note>,
    pub trash: Vec<Note>,
    /// Every non-hidden directory under every root (absolute, roots excluded), sorted.
    pub folders: Vec<PathBuf>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SaveOutcome {
    Saved,
    Conflict(PathBuf),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Reload {
    Added,
    Updated,
    Removed,
    Ignored,
    /// A directory-level change: caller should `rescan`.
    Rescan,
}

fn mtime(path: &Path) -> Result<SystemTime> {
    Ok(fs::metadata(path)?.modified()?)
}

fn file_name(path: &Path) -> &str {
    path.file_name().and_then(|n| n.to_str()).unwrap_or("")
}

fn hidden(name: &str) -> bool {
    name.starts_with('.')
}

pub fn is_md(path: &Path) -> bool {
    path.extension().is_some_and(|e| e == "md")
}

fn load_note(root: usize, path: &Path) -> Result<Note> {
    let text = fs::read_to_string(path)?;
    let meta = fs::metadata(path)?;
    let modified = meta.modified()?;
    let created = meta.created().unwrap_or(modified);
    Ok(Note::from_text(
        root,
        path.to_path_buf(),
        text,
        modified,
        created,
    ))
}

/// Recursively collect notes and folders under `dir`, skipping dot-prefixed entries.
fn walk(root: usize, dir: &Path, notes: &mut Vec<Note>, folders: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(dir).with_context(|| format!("reading {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        if hidden(file_name(&path)) {
            continue;
        }
        let ft = entry.file_type()?;
        if ft.is_dir() {
            folders.push(path.clone());
            walk(root, &path, notes, folders)?;
        } else if ft.is_file() && is_md(&path) {
            match load_note(root, &path) {
                Ok(n) => notes.push(n),
                Err(e) => eprintln!("skipping {}: {e}", path.display()),
            }
        }
    }
    Ok(())
}

fn root_names(dirs: &[PathBuf]) -> Vec<String> {
    let base: Vec<String> = dirs.iter().map(|d| file_name(d).to_string()).collect();
    base.iter()
        .enumerate()
        .map(|(i, name)| {
            let dup = base.iter().filter(|n| *n == name).count() > 1;
            match dirs[i].parent().map(file_name) {
                Some(parent) if dup && !parent.is_empty() => format!("{name} ({parent})"),
                _ => name.clone(),
            }
        })
        .collect()
}

/// `dir/stem.md`, or `dir/stem-2.md`, `-3`, … until the path is free or equals `keep`.
fn free_path(dir: &Path, stem: &str, keep: Option<&Path>) -> PathBuf {
    let mut n = 1;
    loop {
        let name = if n == 1 {
            format!("{stem}.md")
        } else {
            format!("{stem}-{n}.md")
        };
        let candidate = dir.join(name);
        if keep == Some(candidate.as_path()) || !candidate.exists() {
            return candidate;
        }
        n += 1;
    }
}

/// `parent/name`, or `parent/name-2`, … until the directory path is free.
fn free_dir(parent: &Path, name: &str) -> PathBuf {
    let mut n = 1;
    loop {
        let candidate = if n == 1 {
            parent.join(name)
        } else {
            parent.join(format!("{name}-{n}"))
        };
        if !candidate.exists() {
            return candidate;
        }
        n += 1;
    }
}

fn stem_of(path: &Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("untitled")
        .to_string()
}

/// Write via a dot-prefixed temp file in the same directory, then rename over the target.
pub(crate) fn write_atomic(target: &Path, text: &str) -> Result<()> {
    let dir = target.parent().context("target has no parent")?;
    let name = target
        .file_name()
        .and_then(|n| n.to_str())
        .context("target has no file name")?;
    let tmp = dir.join(format!(".{name}.tmp-{}", std::process::id()));
    let result = (|| -> Result<()> {
        let mut f = fs::File::create(&tmp)?;
        f.write_all(text.as_bytes())?;
        f.sync_all()?;
        fs::rename(&tmp, target)?;
        Ok(())
    })();
    if result.is_err() {
        let _ = fs::remove_file(&tmp);
    }
    result
}

fn valid_folder_name(name: &str) -> Result<()> {
    if name.is_empty() || name.contains('/') || name.contains('\\') || hidden(name) || name == ".."
    {
        bail!("invalid folder name");
    }
    Ok(())
}

impl Store {
    pub fn load(dirs: &[PathBuf]) -> Result<Store> {
        let names = root_names(dirs);
        let mut roots = Vec::with_capacity(dirs.len());
        let mut notes = Vec::new();
        let mut trash = Vec::new();
        let mut folders = Vec::new();
        for (i, dir) in dirs.iter().enumerate() {
            fs::create_dir_all(dir).with_context(|| format!("creating {}", dir.display()))?;
            walk(i, dir, &mut notes, &mut folders)?;
            let trash_dir = dir.join(TRASH_DIR);
            if trash_dir.is_dir() {
                let mut ignored = Vec::new();
                walk(i, &trash_dir, &mut trash, &mut ignored)?;
            }
            roots.push(Root {
                dir: dir.clone(),
                name: names[i].clone(),
            });
        }
        notes.sort_by_key(|n| std::cmp::Reverse(n.modified));
        trash.sort_by_key(|n| std::cmp::Reverse(n.modified));
        folders.sort();
        Ok(Store {
            roots,
            notes,
            trash,
            folders,
        })
    }

    /// Reload everything from disk, keeping the same roots in the same order.
    pub fn rescan(&mut self) -> Result<()> {
        let dirs: Vec<PathBuf> = self.roots.iter().map(|r| r.dir.clone()).collect();
        *self = Store::load(&dirs)?;
        Ok(())
    }

    /// Root containing `path` (longest matching prefix).
    pub fn root_of(&self, path: &Path) -> Option<usize> {
        self.roots
            .iter()
            .enumerate()
            .filter(|(_, r)| path.starts_with(&r.dir))
            .max_by_key(|(_, r)| r.dir.as_os_str().len())
            .map(|(i, _)| i)
    }

    /// `(root, in_trash)` for a visible path, `None` for hidden or foreign paths.
    pub fn classify(&self, path: &Path) -> Option<(usize, bool)> {
        let root = self.root_of(path)?;
        let rel = path.strip_prefix(&self.roots[root].dir).ok()?;
        let mut comps = rel
            .components()
            .map(|c| c.as_os_str().to_str().unwrap_or("."));
        let mut first = comps.next();
        let in_trash = first == Some(TRASH_DIR);
        if in_trash {
            first = comps.next();
        }
        if first.into_iter().chain(comps).any(hidden) {
            return None;
        }
        Some((root, in_trash))
    }

    /// `"root/rel"` label for a folder path (root itself → `"root"`).
    pub fn folder_label(&self, dir: &Path) -> String {
        let Some(r) = self.root_of(dir) else {
            return dir.display().to_string();
        };
        let root = &self.roots[r];
        match dir.strip_prefix(&root.dir) {
            Ok(rel) if !rel.as_os_str().is_empty() => format!("{}/{}", root.name, rel.display()),
            _ => root.name.clone(),
        }
    }

    /// Ensure `dir` and its ancestors below the root are listed in `folders`.
    fn track_dirs(&mut self, dir: &Path) {
        let Some(r) = self.root_of(dir) else { return };
        let root = self.roots[r].dir.clone();
        let mut changed = false;
        for anc in dir.ancestors() {
            if anc == root || !anc.starts_with(&root) {
                break;
            }
            if !self.folders.iter().any(|f| f == anc) {
                self.folders.push(anc.to_path_buf());
                changed = true;
            }
        }
        if changed {
            self.folders.sort();
        }
    }

    /// Save `new_text` into note `idx`. Never overwrites a file that changed on disk since load.
    pub fn save(&mut self, idx: usize, new_text: &str) -> Result<SaveOutcome> {
        let note = &self.notes[idx];
        let root = note.root;
        let dir = note
            .path
            .parent()
            .context("note has no parent")?
            .to_path_buf();
        if note.path.exists() && mtime(&note.path)? != note.modified {
            let stamp = chrono::Local::now().format("%Y-%m-%dT%H-%M-%S");
            let stem = format!("{} (conflict {stamp})", slug_of(&title_of(new_text)));
            let target = free_path(&dir, &stem, None);
            write_atomic(&target, new_text)?;
            let conflict = load_note(root, &target)?;
            self.notes[idx] = load_note(root, &self.notes[idx].path)?;
            self.notes.insert(0, conflict);
            return Ok(SaveOutcome::Conflict(target));
        }

        let old_path = note.path.clone();
        let created = note.created;
        let target = free_path(&dir, &slug_of(&title_of(new_text)), Some(&old_path));
        write_atomic(&target, new_text)?;
        if target != old_path && old_path.exists() {
            fs::remove_file(&old_path)?;
        }
        self.notes[idx] = Note::from_text(
            root,
            target.clone(),
            new_text.to_string(),
            mtime(&target)?,
            created,
        );
        Ok(SaveOutcome::Saved)
    }

    /// Create a templated note in `dir` (must be inside a root), insert at index 0.
    pub fn create(&mut self, dir: &Path, template: &str) -> Result<usize> {
        let root = self.root_of(dir).context("folder is not inside a root")?;
        let target = free_path(dir, &slug_of(&title_of(template)), None);
        write_atomic(&target, template)?;
        self.notes.insert(0, load_note(root, &target)?);
        Ok(0)
    }

    /// Write `text` to exactly `path` (re-creating an externally deleted note), insert at index 0.
    pub fn create_at(&mut self, path: &Path, text: &str) -> Result<usize> {
        let root = self.root_of(path).context("path is not inside a root")?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        write_atomic(path, text)?;
        self.notes.retain(|n| n.path != path);
        self.notes.insert(0, load_note(root, path)?);
        Ok(0)
    }

    /// `root-name/rel/stem` — the CLI identifier of a note (`.md` dropped).
    pub fn note_id(&self, note: &Note) -> String {
        let root = &self.roots[note.root];
        let rel = note.path.strip_prefix(&root.dir).unwrap_or(&note.path);
        format!("{}/{}", root.name, rel.with_extension("").display())
    }

    /// Note index a `[[target]]` points to: title match (case-insensitive, trimmed) wins,
    /// else a file stem equal to `slug_of(target)`. Most recently modified first (`notes` order).
    pub fn resolve_link(&self, target: &str) -> Option<usize> {
        let key = link_key(target);
        if let Some(i) = self.notes.iter().position(|n| link_key(&n.title) == key) {
            return Some(i);
        }
        let stem = slug_of(target);
        self.notes.iter().position(|n| stem_of(&n.path) == stem)
    }

    /// Indices of notes (other than `idx`) containing a link that resolves to `idx`, in `notes` order.
    pub fn backlinks(&self, idx: usize) -> Vec<usize> {
        let mut cache: HashMap<String, Option<usize>> = HashMap::new();
        self.notes
            .iter()
            .enumerate()
            .filter(|(i, n)| {
                *i != idx
                    && n.links.iter().any(|l| {
                        *cache
                            .entry(link_key(l))
                            .or_insert_with(|| self.resolve_link(l))
                            == Some(idx)
                    })
            })
            .map(|(i, _)| i)
            .collect()
    }

    /// `backlinks(i)` for every note at once: each distinct link target is resolved a single
    /// time, so listing a whole vault stays linear in the number of links.
    pub fn backlinks_all(&self) -> Vec<Vec<usize>> {
        let mut cache: HashMap<String, Option<usize>> = HashMap::new();
        let mut out = vec![Vec::new(); self.notes.len()];
        for (i, n) in self.notes.iter().enumerate() {
            let mut seen = Vec::new();
            for l in &n.links {
                let target = *cache
                    .entry(link_key(l))
                    .or_insert_with(|| self.resolve_link(l));
                if let Some(t) = target
                    && t != i
                    && !seen.contains(&t)
                {
                    seen.push(t);
                    out[t].push(i);
                }
            }
        }
        out
    }

    /// Rewrite `[[old_title]]` → `[[new_title]]` on disk in every note not in `skip`.
    /// Returns the paths that were rewritten. Stops at the first write error (earlier files
    /// stay rewritten). No-op when `link_key(old) == link_key(new)`.
    pub fn relink(
        &mut self,
        old_title: &str,
        new_title: &str,
        skip: &[PathBuf],
    ) -> Result<Vec<PathBuf>> {
        let key = link_key(old_title);
        if key == link_key(new_title) {
            return Ok(Vec::new());
        }
        let mut done = Vec::new();
        for j in 0..self.notes.len() {
            let n = &self.notes[j];
            if skip.contains(&n.path) || !n.links.iter().any(|l| link_key(l) == key) {
                continue;
            }
            let text = note::replace_links(&n.text, old_title, new_title);
            let path = n.path.clone();
            write_atomic(&path, &text)?;
            self.notes[j] = load_note(n.root, &path)?;
            done.push(path);
        }
        Ok(done)
    }

    pub fn trash(&mut self, idx: usize) -> Result<()> {
        let mut note = self.notes.remove(idx);
        let root = &self.roots[note.root].dir;
        let rel_parent = note
            .path
            .parent()
            .and_then(|p| p.strip_prefix(root).ok())
            .map(Path::to_path_buf)
            .unwrap_or_default();
        let trash_dir = root.join(TRASH_DIR).join(rel_parent);
        let result = fs::create_dir_all(&trash_dir)
            .map_err(anyhow::Error::from)
            .and_then(|()| {
                let target = free_path(&trash_dir, &stem_of(&note.path), None);
                fs::rename(&note.path, &target)?;
                Ok(target)
            });
        match result {
            Ok(target) => {
                note.path = target;
                self.trash.insert(0, note);
                Ok(())
            }
            Err(e) => {
                self.notes.insert(idx, note);
                Err(e)
            }
        }
    }

    pub fn restore(&mut self, tidx: usize) -> Result<()> {
        let mut note = self.trash.remove(tidx);
        let root = self.roots[note.root].dir.clone();
        let rel_parent = note
            .path
            .parent()
            .and_then(|p| p.strip_prefix(root.join(TRASH_DIR)).ok())
            .map(Path::to_path_buf)
            .unwrap_or_default();
        let dir = root.join(rel_parent);
        let result = fs::create_dir_all(&dir)
            .map_err(anyhow::Error::from)
            .and_then(|()| {
                let target = free_path(&dir, &stem_of(&note.path), None);
                fs::rename(&note.path, &target)?;
                Ok(target)
            });
        match result {
            Ok(target) => {
                note.path = target;
                self.notes.insert(0, note);
                self.track_dirs(&dir);
                Ok(())
            }
            Err(e) => {
                self.trash.insert(tidx, note);
                Err(e)
            }
        }
    }

    pub fn purge(&mut self, tidx: usize) -> Result<()> {
        let path = self.trash[tidx].path.clone();
        match fs::remove_file(&path) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(e.into()),
        }
        self.trash.remove(tidx);
        Ok(())
    }

    /// Move note `idx` into folder `dir` (a root or one of `folders`).
    pub fn move_note(&mut self, idx: usize, dir: &Path) -> Result<()> {
        let root = self.root_of(dir).context("folder is not inside a root")?;
        let note = &mut self.notes[idx];
        if note.path.parent() == Some(dir) {
            bail!("already in that folder");
        }
        let target = free_path(dir, &stem_of(&note.path), None);
        fs::rename(&note.path, &target)?;
        note.path = target;
        note.root = root;
        Ok(())
    }

    pub fn create_folder(&mut self, parent: &Path, name: &str) -> Result<PathBuf> {
        valid_folder_name(name)?;
        self.root_of(parent)
            .context("parent is not inside a root")?;
        let dir = parent.join(name);
        if dir.exists() {
            bail!("folder exists");
        }
        fs::create_dir(&dir)?;
        self.folders.push(dir.clone());
        self.folders.sort();
        Ok(dir)
    }

    pub fn rename_folder(&mut self, dir: &Path, name: &str) -> Result<PathBuf> {
        valid_folder_name(name)?;
        let parent = dir.parent().context("folder has no parent")?;
        let new = parent.join(name);
        if new == dir {
            return Ok(new);
        }
        if new.exists() {
            bail!("folder exists");
        }
        fs::rename(dir, &new)?;
        for n in &mut self.notes {
            if let Ok(rest) = n.path.strip_prefix(dir) {
                n.path = new.join(rest);
            }
        }
        for f in &mut self.folders {
            if let Ok(rest) = f.strip_prefix(dir) {
                *f = new.join(rest);
            }
        }
        self.folders.sort();
        Ok(new)
    }

    /// Empty folder → removed; otherwise moved (with its notes) to `.Trash/<rel>`.
    pub fn delete_folder(&mut self, dir: &Path) -> Result<()> {
        let r = self.root_of(dir).context("folder is not inside a root")?;
        let root = self.roots[r].dir.clone();
        if dir == root {
            bail!("cannot delete a root folder");
        }
        let has_notes = self.notes.iter().any(|n| n.path.starts_with(dir));
        let has_subs = self.folders.iter().any(|f| f != dir && f.starts_with(dir));
        if !has_notes && !has_subs {
            fs::remove_dir(dir)?;
        } else {
            let rel = dir.strip_prefix(&root)?;
            let trash_parent = root
                .join(TRASH_DIR)
                .join(rel.parent().unwrap_or(Path::new("")));
            fs::create_dir_all(&trash_parent)?;
            let target = free_dir(&trash_parent, file_name(dir));
            fs::rename(dir, &target)?;
            let (moved, kept): (Vec<Note>, Vec<Note>) = std::mem::take(&mut self.notes)
                .into_iter()
                .partition(|n| n.path.starts_with(dir));
            self.notes = kept;
            for mut n in moved {
                if let Ok(rest) = n.path.strip_prefix(dir) {
                    n.path = target.join(rest);
                }
                self.trash.insert(0, n);
            }
        }
        self.folders.retain(|f| !f.starts_with(dir));
        Ok(())
    }

    /// Apply an external filesystem change for `path`.
    pub fn reload_path(&mut self, path: &Path) -> Reload {
        let Some((root, in_trash)) = self.classify(path) else {
            return Reload::Ignored;
        };
        if !is_md(path) {
            return Reload::Rescan;
        }
        let list = if in_trash {
            &mut self.trash
        } else {
            &mut self.notes
        };
        let pos = list.iter().position(|n| n.path == path);
        match load_note(root, path) {
            Ok(note) => match pos {
                Some(i) => {
                    if list[i].modified == note.modified && list[i].text == note.text {
                        return Reload::Ignored;
                    }
                    list[i] = note;
                    Reload::Updated
                }
                None => {
                    list.insert(0, note);
                    Reload::Added
                }
            },
            Err(_) => match pos {
                Some(i) => {
                    list.remove(i);
                    Reload::Removed
                }
                None => Reload::Ignored,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir() -> PathBuf {
        let d = std::env::temp_dir().join(format!(
            "tnotes-test-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&d).unwrap();
        d
    }

    fn load(dir: &Path) -> Store {
        Store::load(&[dir.to_path_buf()]).unwrap()
    }

    #[test]
    fn conflict_creates_copy() {
        let dir = temp_dir();
        let path = dir.join("groceries.md");
        fs::write(&path, "# Groceries\n\nmilk\n").unwrap();
        let mut store = load(&dir);
        assert_eq!(store.notes.len(), 1);

        std::thread::sleep(std::time::Duration::from_millis(1100));
        fs::write(&path, "# Groceries\n\nexternal edit\n").unwrap();

        let out = store.save(0, "# Groceries\n\nmy edit\n").unwrap();
        let conflict = match out {
            SaveOutcome::Conflict(p) => p,
            SaveOutcome::Saved => panic!("expected conflict"),
        };
        assert_eq!(
            fs::read_to_string(&path).unwrap(),
            "# Groceries\n\nexternal edit\n"
        );
        assert_eq!(
            fs::read_to_string(&conflict).unwrap(),
            "# Groceries\n\nmy edit\n"
        );
        assert!(
            conflict
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .contains("conflict")
        );
        assert_eq!(store.notes.len(), 2);
        assert!(store.notes.iter().any(|n| n.text.contains("external edit")));
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn save_renames_on_title_change_and_avoids_collisions() {
        let dir = temp_dir();
        fs::write(dir.join("taken.md"), "# Taken\n").unwrap();
        let mut store = load(&dir);
        let idx = store.create(&dir, "# ").unwrap();
        assert_eq!(fs::read_to_string(dir.join("untitled.md")).unwrap(), "# ");

        assert_eq!(store.save(idx, "# Taken\nnew").unwrap(), SaveOutcome::Saved);
        assert!(!dir.join("untitled.md").exists());
        assert_eq!(store.notes[idx].path, dir.join("taken-2.md"));
        assert_eq!(
            fs::read_to_string(dir.join("taken.md")).unwrap(),
            "# Taken\n"
        );

        // Saving again with the same title keeps the same file.
        assert_eq!(
            store.save(idx, "# Taken\nnewer").unwrap(),
            SaveOutcome::Saved
        );
        assert_eq!(store.notes[idx].path, dir.join("taken-2.md"));
        assert!(
            !fs::read_dir(&dir)
                .unwrap()
                .any(|e| { e.unwrap().file_name().to_string_lossy().starts_with('.') })
        );
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn trash_restore_purge_roundtrip() {
        let dir = temp_dir();
        fs::write(dir.join("a.md"), "# A\n").unwrap();
        let mut store = load(&dir);
        store.trash(0).unwrap();
        assert!(dir.join(TRASH_DIR).join("a.md").exists());
        assert!(store.notes.is_empty() && store.trash.len() == 1);
        store.restore(0).unwrap();
        assert!(dir.join("a.md").exists());
        store.trash(0).unwrap();
        store.purge(0).unwrap();
        assert!(!dir.join(TRASH_DIR).join("a.md").exists());
        assert!(store.trash.is_empty());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn reload_path_ignores_tmp_and_tracks_changes() {
        let dir = temp_dir();
        let mut store = load(&dir);
        assert_eq!(store.reload_path(&dir.join(".x.md.tmp-1")), Reload::Ignored);
        assert_eq!(
            store.reload_path(&dir.join(".git").join("x")),
            Reload::Ignored
        );
        assert_eq!(store.reload_path(&dir.join("sub")), Reload::Rescan);
        let p = dir.join("n.md");
        fs::write(&p, "# N\n").unwrap();
        assert_eq!(store.reload_path(&p), Reload::Added);
        assert_eq!(store.reload_path(&p), Reload::Ignored);
        fs::remove_file(&p).unwrap();
        assert_eq!(store.reload_path(&p), Reload::Removed);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn trash_mirrors_relative_path() {
        let dir = temp_dir();
        let nested = dir.join("a").join("b");
        fs::create_dir_all(&nested).unwrap();
        fs::write(nested.join("x.md"), "# X\n").unwrap();
        let mut store = load(&dir);
        assert_eq!(store.folders, vec![dir.join("a"), nested.clone()]);
        store.trash(0).unwrap();
        let trashed = dir.join(TRASH_DIR).join("a").join("b").join("x.md");
        assert!(trashed.exists());
        assert_eq!(store.trash[0].path, trashed);
        fs::remove_dir_all(&nested).unwrap();
        store.restore(0).unwrap();
        assert!(nested.join("x.md").exists());
        assert_eq!(store.notes[0].path, nested.join("x.md"));
        assert!(store.folders.contains(&nested));
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn delete_folder_moves_notes_to_trash() {
        let dir = temp_dir();
        let mut store = load(&dir);
        let work = store.create_folder(&dir, "work").unwrap();
        let empty = store.create_folder(&work, "empty").unwrap();
        assert!(store.create_folder(&dir, ".hidden").is_err());
        fs::write(work.join("p.md"), "# P\n").unwrap();
        assert_eq!(store.reload_path(&work.join("p.md")), Reload::Added);

        store.delete_folder(&empty).unwrap();
        assert!(!empty.exists());
        assert_eq!(store.folders, vec![work.clone()]);

        store.delete_folder(&work).unwrap();
        assert!(!work.exists());
        assert!(dir.join(TRASH_DIR).join("work").join("p.md").exists());
        assert!(store.notes.is_empty());
        assert_eq!(
            store.trash[0].path,
            dir.join(TRASH_DIR).join("work").join("p.md")
        );
        assert!(store.folders.is_empty());

        store.restore(0).unwrap();
        assert!(work.join("p.md").exists());
        assert_eq!(store.folders, vec![work]);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn move_note_updates_root_and_path() {
        let a = temp_dir();
        let b = temp_dir();
        fs::write(a.join("n.md"), "# N\n").unwrap();
        let mut store = Store::load(&[a.clone(), b.clone()]).unwrap();
        assert_eq!(store.notes[0].root, 0);
        assert!(store.move_note(0, &a).is_err());
        store.move_note(0, &b).unwrap();
        assert_eq!(store.notes[0].root, 1);
        assert_eq!(store.notes[0].path, b.join("n.md"));
        assert!(b.join("n.md").exists() && !a.join("n.md").exists());
        assert_eq!(store.classify(&b.join("n.md")), Some((1, false)));
        assert_eq!(
            store.classify(&b.join(TRASH_DIR).join("n.md")),
            Some((1, true))
        );
        assert_eq!(store.classify(&b.join(TRASH_DIR).join(".x")), None);
        fs::remove_dir_all(&a).unwrap();
        fs::remove_dir_all(&b).unwrap();
    }

    #[test]
    fn rename_folder_rewrites_paths() {
        let dir = temp_dir();
        let mut store = load(&dir);
        let work = store.create_folder(&dir, "work").unwrap();
        let sub = store.create_folder(&work, "sub").unwrap();
        fs::write(sub.join("n.md"), "# N\n").unwrap();
        store.reload_path(&sub.join("n.md"));
        let new = store.rename_folder(&work, "mtg").unwrap();
        assert_eq!(new, dir.join("mtg"));
        assert_eq!(store.notes[0].path, new.join("sub").join("n.md"));
        assert_eq!(store.folders, vec![new.clone(), new.join("sub")]);
        assert!(new.join("sub").join("n.md").exists());
        fs::remove_dir_all(&dir).unwrap();
    }

    fn idx(store: &Store, name: &str) -> usize {
        store
            .notes
            .iter()
            .position(|n| n.path.file_name().unwrap() == name)
            .unwrap()
    }

    #[test]
    fn resolve_link_prefers_title_then_stem() {
        let dir = temp_dir();
        fs::write(dir.join("plan.md"), "# Weekly plan\n").unwrap();
        fs::write(dir.join("other.md"), "# Other\n").unwrap();
        let store = load(&dir);
        assert_eq!(
            store.resolve_link("weekly PLAN"),
            Some(idx(&store, "plan.md"))
        );
        assert_eq!(store.resolve_link("other"), Some(idx(&store, "other.md")));
        assert_eq!(store.resolve_link("plan"), Some(idx(&store, "plan.md")));
        assert_eq!(store.resolve_link("nope"), None);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn backlinks_follow_resolved_links() {
        let dir = temp_dir();
        fs::write(dir.join("a.md"), "# A\n\n[[B]]\n").unwrap();
        fs::write(dir.join("b.md"), "# B\n").unwrap();
        fs::write(dir.join("c.md"), "# C\n").unwrap();
        let store = load(&dir);
        let (a, b) = (idx(&store, "a.md"), idx(&store, "b.md"));
        assert_eq!(store.backlinks(b), vec![a]);
        assert_eq!(store.backlinks(a), Vec::<usize>::new());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn backlinks_all_matches_backlinks() {
        let dir = temp_dir();
        fs::write(dir.join("a.md"), "# A\n\n[[B]] [[b]] [[C]] [[A]]\n").unwrap();
        fs::write(dir.join("b.md"), "# B\n\n[[C]]\n").unwrap();
        fs::write(dir.join("c.md"), "# C\n").unwrap();
        let store = load(&dir);
        let all = store.backlinks_all();
        for (i, links) in all.iter().enumerate() {
            assert_eq!(*links, store.backlinks(i), "note {i}");
        }
        assert_eq!(all[idx(&store, "c.md")].len(), 2);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn relink_rewrites_files_and_skips_listed_paths() {
        let dir = temp_dir();
        fs::write(dir.join("a.md"), "# A\n\nsee [[B]]\n").unwrap();
        fs::write(dir.join("b.md"), "# B\n").unwrap();
        fs::write(dir.join("d.md"), "# D\n\n[[B]]\n").unwrap();
        let mut store = load(&dir);
        let d = dir.join("d.md");
        let done = store.relink("B", "Bee", std::slice::from_ref(&d)).unwrap();
        assert_eq!(done, vec![dir.join("a.md")]);
        assert_eq!(
            fs::read_to_string(dir.join("a.md")).unwrap(),
            "# A\n\nsee [[Bee]]\n"
        );
        assert_eq!(fs::read_to_string(&d).unwrap(), "# D\n\n[[B]]\n");
        assert_eq!(store.notes[idx(&store, "a.md")].links, vec!["Bee"]);
        assert!(store.relink("x", "X", &[]).unwrap().is_empty());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn create_names_the_file_after_the_title() {
        let dir = temp_dir();
        let mut store = load(&dir);
        let i = store.create(&dir, "# Groceries\n").unwrap();
        assert_eq!(store.notes[i].path, dir.join("groceries.md"));
        assert_eq!(
            store.note_id(&store.notes[i]),
            format!("{}/groceries", store.roots[0].name)
        );
        let i = store.create(&dir, "# ").unwrap();
        assert_eq!(store.notes[i].path, dir.join("untitled.md"));
        fs::remove_dir_all(&dir).unwrap();
    }
}
