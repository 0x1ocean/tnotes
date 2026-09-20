use std::path::PathBuf;
use std::time::SystemTime;

/// One Markdown file in the vault.
#[derive(Debug, Clone)]
pub struct Note {
    /// Index of the root folder this note lives under (`Store::roots`).
    pub root: usize,
    /// Absolute path of the file on disk.
    pub path: PathBuf,
    /// Body as loaded/saved.
    pub text: String,
    pub title: String,
    /// Full tag paths (`work/project`), lowercase, deduped, sorted.
    pub tags: Vec<String>,
    /// `[[link]]` targets as written, trimmed, deduplicated case-insensitively, in text order.
    pub links: Vec<String>,
    /// Filesystem mtime after the last load/save.
    pub modified: SystemTime,
    /// Filesystem birth time; falls back to `modified` where unsupported.
    pub created: SystemTime,
}

impl Note {
    pub fn from_text(
        root: usize,
        path: PathBuf,
        text: String,
        modified: SystemTime,
        created: SystemTime,
    ) -> Self {
        Self {
            title: title_of(&text),
            tags: tags_of(&text),
            links: links_of(&text),
            root,
            path,
            text,
            modified,
            created,
        }
    }

    /// First non-empty line after the title, with leading markdown markers stripped.
    pub fn preview(&self) -> String {
        // Skip the title line (the first line with content after `#`s are stripped);
        // a bare `# ` above it is not content and is skipped too.
        let mut lines = self
            .text
            .lines()
            .filter(|l| !l.trim().trim_start_matches('#').trim().is_empty());
        lines.next();
        lines.next().map(strip_markers).unwrap_or_default()
    }
}

/// Strip leading block markers (`#`, `-`, `*`, `>`, `[ ]`) that are followed by whitespace,
/// so `#tag` at line start survives while `# Heading` / `- item` lose their prefix.
fn strip_markers(line: &str) -> String {
    let mut s = line.trim();
    loop {
        let before = s;
        let marker_len = s
            .chars()
            .take_while(|c| matches!(c, '#' | '-' | '*' | '>'))
            .count();
        if marker_len > 0 && s[marker_len..].starts_with(char::is_whitespace) {
            s = s[marker_len..].trim_start();
        }
        if let Some(rest) = s.strip_prefix("[ ]").or_else(|| s.strip_prefix("[x]")) {
            s = rest.trim_start();
        }
        if s == before {
            break;
        }
    }
    s.to_string()
}

/// First line with content once heading markers are stripped (so `# ` alone is skipped
/// and the text below becomes the title). Nothing → `Untitled`.
pub fn title_of(text: &str) -> String {
    let title = text
        .lines()
        .map(|l| l.trim().trim_start_matches('#').trim())
        .find(|l| !l.is_empty())
        .unwrap_or("");
    if title.is_empty() {
        "Untitled".to_string()
    } else {
        title.to_string()
    }
}

/// Lowercase filename stem: runs of non-alphanumerics collapse to `-`, trimmed, ≤80 chars.
pub fn slug_of(title: &str) -> String {
    let mut out = String::with_capacity(title.len());
    let mut pending_dash = false;
    for c in title.chars() {
        if c.is_alphanumeric() {
            if pending_dash && !out.is_empty() {
                out.push('-');
            }
            pending_dash = false;
            out.extend(c.to_lowercase());
        } else {
            pending_dash = true;
        }
    }
    if out.chars().count() > 80 {
        let cut = out
            .char_indices()
            .nth(80)
            .map(|(i, _)| i)
            .unwrap_or(out.len());
        out.truncate(cut);
        while out.ends_with('-') {
            out.pop();
        }
    }
    if out.is_empty() {
        "untitled".to_string()
    } else {
        out
    }
}

pub fn is_tag_char(c: char) -> bool {
    c.is_alphanumeric() || matches!(c, '_' | '-' | '/')
}

/// Inline `#a/b/c` tags outside fenced code blocks; lowercase, deduped, sorted.
pub fn tags_of(text: &str) -> Vec<String> {
    let mut tags: Vec<String> = Vec::new();
    let mut in_fence = false;
    for line in text.lines() {
        if line.trim_start().starts_with("```") {
            in_fence = !in_fence;
            continue;
        }
        if in_fence {
            continue;
        }
        let mut prev_ok = true; // start of line counts as whitespace
        let mut iter = line.char_indices().peekable();
        while let Some((i, c)) = iter.next() {
            if c == '#' && prev_ok {
                let body: String = line[i + 1..]
                    .chars()
                    .take_while(|&c| is_tag_char(c))
                    .collect();
                let body = body.trim_end_matches(['/', '-']);
                if !body.is_empty() && !body.chars().all(|c| c.is_ascii_digit()) {
                    let tag = body.to_lowercase();
                    if !tags.contains(&tag) {
                        tags.push(tag);
                    }
                }
                // skip the consumed body
                let skip = body.chars().count();
                for _ in 0..skip {
                    iter.next();
                }
                prev_ok = false;
                continue;
            }
            prev_ok = c.is_whitespace();
        }
    }
    tags.sort();
    tags
}

/// Normalised comparison key of a link target or title.
pub fn link_key(s: &str) -> String {
    s.trim().to_lowercase()
}

/// `[[target]]` span starting at byte `open` (which must point at `[[`): the byte range of
/// the trimmed inner text and the byte index just past the closing `]]`. A target is the
/// text up to the next `]]` on the line; it is rejected when empty after trimming or when
/// it contains `[` or `]`.
fn link_span(line: &str, open: usize) -> Option<(std::ops::Range<usize>, usize)> {
    let rest = &line[open + 2..];
    let close = rest.find("]]")?;
    let inner = &rest[..close];
    if inner.contains(['[', ']']) {
        return None;
    }
    let trimmed = inner.trim();
    if trimmed.is_empty() {
        return None;
    }
    let start = open + 2 + (trimmed.as_ptr() as usize - inner.as_ptr() as usize);
    Some((start..start + trimmed.len(), open + 2 + close + 2))
}

/// Lines outside fenced code blocks, with their fence state tracked.
fn unfenced(text: &str) -> impl Iterator<Item = &str> {
    let mut in_fence = false;
    text.lines().filter(move |line| {
        if line.trim_start().starts_with("```") {
            in_fence = !in_fence;
            return false;
        }
        !in_fence
    })
}

/// `[[target]]` occurrences outside fenced code blocks. A target is the text between
/// `[[` and the next `]]` on the same line; it is rejected when empty after trimming or
/// when it contains `[` or `]`.
pub fn links_of(text: &str) -> Vec<String> {
    let mut links: Vec<String> = Vec::new();
    for line in unfenced(text) {
        let mut from = 0;
        while let Some(off) = line[from..].find("[[") {
            let open = from + off;
            match link_span(line, open) {
                Some((inner, end)) => {
                    let target = &line[inner];
                    let key = link_key(target);
                    if !links.iter().any(|l| link_key(l) == key) {
                        links.push(target.to_string());
                    }
                    from = end;
                }
                None => from = open + 1,
            }
        }
    }
    links
}

/// Rewrite every `[[X]]` whose `link_key(X) == link_key(old)` to `[[new]]`, outside fences.
/// Returns the text unchanged when nothing matched.
pub fn replace_links(text: &str, old: &str, new: &str) -> String {
    let key = link_key(old);
    let mut out = String::with_capacity(text.len());
    let mut in_fence = false;
    for line in text.split_inclusive('\n') {
        let body = line.strip_suffix('\n').unwrap_or(line);
        if body.trim_start().starts_with("```") {
            in_fence = !in_fence;
            out.push_str(line);
            continue;
        }
        if in_fence {
            out.push_str(line);
            continue;
        }
        let mut from = 0;
        while let Some(off) = body[from..].find("[[") {
            let open = from + off;
            match link_span(body, open) {
                Some((inner, end)) => {
                    if link_key(&body[inner.clone()]) == key {
                        out.push_str(&body[from..inner.start]);
                        out.push_str(new);
                        out.push_str(&body[inner.end..end]);
                    } else {
                        out.push_str(&body[from..end]);
                    }
                    from = end;
                }
                None => {
                    out.push_str(&body[from..open + 1]);
                    from = open + 1;
                }
            }
        }
        out.push_str(&line[from..]);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn title_strips_heading_markers() {
        assert_eq!(title_of("# Sprint plan\nbody"), "Sprint plan");
        assert_eq!(title_of("\n\n  ## Two  \n"), "Two");
        assert_eq!(title_of(""), "Untitled");
        assert_eq!(title_of("#\n"), "Untitled");
    }

    #[test]
    fn slug_normalizes() {
        assert_eq!(slug_of("Sprint plan"), "sprint-plan");
        assert_eq!(slug_of("  Hello,   World! "), "hello-world");
        assert_eq!(slug_of("Привет мир"), "привет-мир");
        assert_eq!(slug_of("!!!"), "untitled");
        let long = "a".repeat(100);
        assert_eq!(slug_of(&long).chars().count(), 80);
    }

    #[test]
    fn tags_nested_and_case() {
        assert_eq!(
            tags_of("# Title\nbody #work/proj and #Work"),
            vec!["work", "work/proj"]
        );
    }

    #[test]
    fn tags_reject_numeric_heading_and_inline_hash() {
        assert_eq!(tags_of("#123 no"), Vec::<String>::new());
        assert_eq!(tags_of("# Heading"), Vec::<String>::new());
        assert_eq!(tags_of("a#b"), Vec::<String>::new());
    }

    #[test]
    fn tags_skip_fenced_blocks() {
        assert_eq!(tags_of("```\n#x\n```\n"), Vec::<String>::new());
        assert_eq!(tags_of("```\n#x\n```\n#y"), vec!["y"]);
    }

    #[test]
    fn tags_unicode_and_trailing_punct() {
        assert_eq!(tags_of("#дом/кухня."), vec!["дом/кухня"]);
        assert_eq!(tags_of("#a/ #b-"), vec!["a", "b"]);
    }

    #[test]
    fn title_falls_through_bare_heading_marker() {
        assert_eq!(title_of("# \n\nfirst line\nsecond"), "first line");
        let t = SystemTime::UNIX_EPOCH;
        let n = Note::from_text(0, "/x.md".into(), "# \n\nfirst line\nsecond\n".into(), t, t);
        assert_eq!(n.preview(), "second");
    }

    #[test]
    fn preview_strips_markers() {
        let t = SystemTime::UNIX_EPOCH;
        let n = Note::from_text(0, "/x.md".into(), "# T\n\n- [ ] milk\n".into(), t, t);
        assert_eq!(n.preview(), "milk");
        let n = Note::from_text(0, "/x.md".into(), "# T\n#work/x done\n".into(), t, t);
        assert_eq!(n.preview(), "#work/x done");
    }

    #[test]
    fn links_parse_and_dedup() {
        assert_eq!(
            links_of("see [[Weekly plan]] and [[weekly PLAN]] and [[a[b]]"),
            vec!["Weekly plan"]
        );
        assert_eq!(links_of("```\n[[x]]\n```\n"), Vec::<String>::new());
        assert_eq!(links_of("[[ ]] [[a]][[b]]"), vec!["a", "b"]);
    }

    #[test]
    fn replace_links_is_case_insensitive_and_keeps_other_text() {
        assert_eq!(
            replace_links("[[Old]] · [[old]] · [[Older]]", "old", "New"),
            "[[New]] · [[New]] · [[Older]]"
        );
        assert_eq!(
            replace_links("x [[ old ]]\n```\n[[old]]\n```\n[[old]]", "old", "N"),
            "x [[ N ]]\n```\n[[old]]\n```\n[[N]]"
        );
    }
}
