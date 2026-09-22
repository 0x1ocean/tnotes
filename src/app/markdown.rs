//! Live markdown highlighting for the editor, expressed as edtui `Highlight` ranges.
//!
//! edtui applies the *first* matching range to a cell, so inner elements (code, tags, emphasis)
//! are pushed before line-level styles (headings, quotes, done tasks).

use edtui::{EditorState, Highlight, Index2, Lines};
use ratatui::style::{Modifier, Style};

use crate::config;
use crate::note::is_tag_char;
use crate::ui::theme::{accent, code, dim, tag};

/// Styles for one highlight mode; `highlights` never builds a `Style` itself.
pub struct Scheme {
    pub h1: Style,
    pub h2: Style,
    pub h3: Style,
    pub code: Style,
    pub link: Style,
    pub tag: Style,
    pub quote: Style,
    pub task: Style,
    pub dim: Style,
}

impl Scheme {
    pub fn mono() -> Scheme {
        let dim = Style::new().fg(dim());
        let bold = Style::new().add_modifier(Modifier::BOLD);
        Scheme {
            h1: bold,
            h2: bold,
            h3: bold,
            code: dim,
            link: Style::new().add_modifier(Modifier::UNDERLINED),
            tag: dim,
            quote: Style::new().add_modifier(Modifier::ITALIC),
            task: dim,
            dim,
        }
    }

    pub fn color() -> Scheme {
        let dim = Style::new().fg(dim());
        let accent = Style::new().fg(accent());
        Scheme {
            h1: accent.add_modifier(Modifier::BOLD),
            h2: accent,
            h3: accent.add_modifier(Modifier::DIM),
            code: Style::new().fg(code()),
            link: accent.add_modifier(Modifier::UNDERLINED),
            tag: Style::new().fg(tag()),
            quote: dim.add_modifier(Modifier::ITALIC),
            task: accent,
            dim,
        }
    }

    pub fn for_mode(m: config::Highlight) -> Scheme {
        match m {
            config::Highlight::Color => Scheme::color(),
            config::Highlight::Mono => Scheme::mono(),
        }
    }
}

/// `(marker end, checkbox range)` for a list line: `  - [x] text` → marker `  - ` and box `[x]`.
pub struct ListLine {
    /// Column where the item text starts (after marker and checkbox).
    pub text_start: usize,
    /// Marker text as typed (`- `, `* `, `3. `), without indentation.
    pub marker: String,
    pub indent: usize,
    /// Column of `[` when the item has a checkbox.
    pub checkbox: Option<usize>,
    pub done: bool,
}

/// Parse a list item line; `None` when the line is not a list item.
pub fn list_line(line: &[char]) -> Option<ListLine> {
    let indent = line.iter().take_while(|c| **c == ' ').count();
    let rest = &line[indent..];
    let marker_len = match rest {
        ['-' | '*' | '+', ' ', ..] => 2,
        _ => {
            let digits = rest.iter().take_while(|c| c.is_ascii_digit()).count();
            if digits > 0 && rest.get(digits) == Some(&'.') && rest.get(digits + 1) == Some(&' ') {
                digits + 2
            } else {
                return None;
            }
        }
    };
    let marker: String = rest[..marker_len].iter().collect();
    let after = &rest[marker_len..];
    let (checkbox, done, text_start) = match after {
        ['[', m, ']', ' ', ..] | ['[', m, ']'] if matches!(m, ' ' | 'x' | 'X') => (
            Some(indent + marker_len),
            *m != ' ',
            indent + marker_len + 4.min(after.len()),
        ),
        _ => (None, false, indent + marker_len),
    };
    Some(ListLine {
        text_start,
        marker,
        indent,
        checkbox,
        done,
    })
}

/// Tag under column `col` (`#work/project` → `work/project`), if any.
pub fn tag_at(line: &[char], col: usize) -> Option<String> {
    let mut i = 0;
    while i < line.len() {
        if line[i] == '#' && (i == 0 || line[i - 1].is_whitespace()) {
            let body = line[i + 1..]
                .iter()
                .take_while(|c| is_tag_char(**c))
                .count();
            if body > 0 && (i..=i + body).contains(&col) {
                let tag: String = line[i + 1..i + 1 + body].iter().collect();
                let tag = tag.trim_end_matches(['/', '-']).to_lowercase();
                return (!tag.is_empty() && !tag.chars().all(|c| c.is_ascii_digit()))
                    .then_some(tag);
            }
            i += body + 1;
            continue;
        }
        i += 1;
    }
    None
}

/// `[[target]]` opening at `i`: the closing `]]` index `j` and the trimmed inner range
/// `(start, end_inclusive)`; same acceptance rule as `note::links_of`.
fn wikilink(line: &[char], i: usize) -> Option<(usize, usize, usize)> {
    if line.get(i) != Some(&'[') || line.get(i + 1) != Some(&'[') {
        return None;
    }
    let j =
        (i + 2..line.len().saturating_sub(1)).find(|&j| line[j] == ']' && line[j + 1] == ']')?;
    if line[i + 2..j].iter().any(|&c| c == '[' || c == ']') {
        return None;
    }
    let start = (i + 2..j).find(|&k| !line[k].is_whitespace())?;
    let end = (i + 2..j).rev().find(|&k| !line[k].is_whitespace())?;
    Some((j, start, end))
}

/// `[[target]]` under column `col` → trimmed target.
pub fn link_at(line: &[char], col: usize) -> Option<String> {
    let mut i = 0;
    while i + 1 < line.len() {
        if let Some((j, start, end)) = wikilink(line, i) {
            if (i..=j + 1).contains(&col) {
                return Some(line[start..=end].iter().collect());
            }
            i = j + 2;
            continue;
        }
        i += 1;
    }
    None
}

/// Marker for the line that follows `line` when Enter is pressed: `1. ` → `2. `.
pub fn next_marker(l: &ListLine) -> String {
    let mut out = " ".repeat(l.indent);
    if let Ok(n) = l.marker.trim_end_matches(". ").parse::<u32>() {
        out.push_str(&format!("{}. ", n + 1));
    } else {
        out.push_str(&l.marker);
    }
    if l.checkbox.is_some() {
        out.push_str("[ ] ");
    }
    out
}

struct Sink<'a> {
    row: usize,
    out: &'a mut Vec<Highlight>,
}

impl Sink<'_> {
    fn push(&mut self, start: usize, end_inclusive: usize, style: Style) {
        if end_inclusive >= start {
            self.out.push(Highlight::new(
                Index2::new(self.row, start),
                Index2::new(self.row, end_inclusive),
                style,
            ));
        }
    }
}

fn is_word(c: Option<&char>) -> bool {
    c.is_some_and(|c| c.is_alphanumeric())
}

/// Inline elements: code spans, links, bold/italic, tags.
fn inline(line: &[char], from: usize, s: &mut Sink, sc: &Scheme) {
    let dim_s = sc.dim;
    let n = line.len();
    let mut i = from;
    while i < n {
        let c = line[i];
        match c {
            '`' => {
                if let Some(j) = (i + 1..n).find(|&j| line[j] == '`') {
                    s.push(i, i, dim_s);
                    if j > i + 1 {
                        s.push(i + 1, j - 1, sc.code);
                    }
                    s.push(j, j, dim_s);
                    i = j + 1;
                    continue;
                }
            }
            '[' => {
                // [[wikilink]]
                if let Some((j, _, _)) = wikilink(line, i) {
                    s.push(i, i + 1, dim_s);
                    s.push(i + 2, j - 1, sc.link);
                    s.push(j, j + 1, dim_s);
                    i = j + 2;
                    continue;
                }
                // [text](url)
                if let Some(close) = (i + 1..n).find(|&j| line[j] == ']')
                    && line.get(close + 1) == Some(&'(')
                    && let Some(end) = (close + 2..n).find(|&j| line[j] == ')')
                {
                    s.push(i, i, dim_s);
                    s.push(i + 1, close - 1, sc.link);
                    s.push(close, end, dim_s);
                    i = end + 1;
                    continue;
                }
            }
            '*' | '_' => {
                let double = line.get(i + 1) == Some(&c);
                let len = if double { 2 } else { 1 };
                let opens = !is_word(line.get(i.wrapping_sub(1))) && is_word(line.get(i + len));
                if opens {
                    let mut j = i + len;
                    let mut found = None;
                    while j + len <= n {
                        if line[j..j + len].iter().all(|&x| x == c) && is_word(line.get(j - 1)) {
                            found = Some(j);
                            break;
                        }
                        j += 1;
                    }
                    if let Some(j) = found {
                        let style = if double {
                            Style::new().add_modifier(Modifier::BOLD)
                        } else {
                            Style::new().add_modifier(Modifier::ITALIC)
                        };
                        s.push(i, i + len - 1, dim_s);
                        s.push(i + len, j - 1, style);
                        s.push(j, j + len - 1, dim_s);
                        i = j + len;
                        continue;
                    }
                }
            }
            '#' => {
                let at_boundary = i == 0 || line[i - 1].is_whitespace();
                let body = line[i + 1..]
                    .iter()
                    .take_while(|c| is_tag_char(**c))
                    .count();
                if at_boundary
                    && body > 0
                    && !line[i + 1..i + 1 + body].iter().all(|c| c.is_ascii_digit())
                {
                    s.push(i, i + body, sc.tag);
                    i += body + 1;
                    continue;
                }
            }
            _ => {}
        }
        i += 1;
    }
}

/// Compute highlights for the whole buffer.
pub fn highlights(lines: &Lines, sc: &Scheme) -> Vec<Highlight> {
    let dim_s = sc.dim;
    let mut out = Vec::new();
    let mut in_fence = false;
    for (row, line) in lines.iter_row().enumerate() {
        let line: &[char] = line;
        let n = line.len();
        if n == 0 {
            continue;
        }
        let mut s = Sink { row, out: &mut out };
        let trimmed: String = line.iter().collect::<String>().trim_start().to_string();
        if trimmed.starts_with("```") {
            s.push(0, n - 1, dim_s);
            in_fence = !in_fence;
            continue;
        }
        if in_fence {
            s.push(0, n - 1, sc.code);
            continue;
        }
        // Headings: `#`s dim, text by level.
        let hashes = line.iter().take_while(|c| **c == '#').count();
        if (1..=6).contains(&hashes) && line.get(hashes) == Some(&' ') {
            s.push(0, hashes, dim_s);
            inline(line, hashes + 1, &mut s, sc);
            let style = match hashes {
                1 => sc.h1,
                2 => sc.h2,
                _ => sc.h3,
            };
            s.push(hashes + 1, n - 1, style);
            continue;
        }
        // Horizontal rule.
        if n >= 3 && (line.iter().all(|&c| c == '-') || line.iter().all(|&c| c == '*')) {
            s.push(0, n - 1, dim_s);
            continue;
        }
        // Blockquote: `>` dim, text `quote`.
        let indent = line.iter().take_while(|c| **c == ' ').count();
        if line.get(indent) == Some(&'>') {
            s.push(indent, indent, dim_s);
            inline(line, indent + 1, &mut s, sc);
            s.push(indent + 1, n - 1, sc.quote);
            continue;
        }
        // List item: marker dim, open checkbox `task`; done items dim + crossed out.
        if let Some(item) = list_line(line) {
            if item.done {
                s.push(0, n - 1, dim_s.add_modifier(Modifier::CROSSED_OUT));
                continue;
            }
            match item.checkbox {
                Some(b) => {
                    s.push(indent, b.saturating_sub(1), dim_s);
                    s.push(b, b + 2, sc.task);
                }
                None => s.push(indent, item.text_start.saturating_sub(1), dim_s),
            }
            inline(line, item.text_start, &mut s, sc);
            continue;
        }
        inline(line, 0, &mut s, sc);
        // First line without `#` is still the title.
        if row == 0 {
            s.push(0, n - 1, sc.h1);
        }
    }
    out
}

pub fn refresh(editor: &mut EditorState, mode: config::Highlight) {
    editor.highlights = highlights(&editor.lines, &Scheme::for_mode(mode));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chars(s: &str) -> Vec<char> {
        s.chars().collect()
    }

    #[test]
    fn list_line_parses_markers_and_checkboxes() {
        let l = list_line(&chars("  - [x] done")).unwrap();
        assert_eq!(l.indent, 2);
        assert_eq!(l.marker, "- ");
        assert_eq!(l.checkbox, Some(4));
        assert!(l.done);
        assert_eq!(l.text_start, 8);

        let l = list_line(&chars("3. third")).unwrap();
        assert_eq!(l.marker, "3. ");
        assert_eq!(l.checkbox, None);
        assert_eq!(l.text_start, 3);

        assert!(list_line(&chars("-x")).is_none());
        assert_eq!(list_line(&chars("- ")).unwrap().text_start, 2);
    }

    #[test]
    fn next_marker_increments_numbers_and_keeps_checkboxes() {
        assert_eq!(next_marker(&list_line(&chars("3. ")).unwrap()), "4. ");
        assert_eq!(
            next_marker(&list_line(&chars("  - [ ] a")).unwrap()),
            "  - [ ] "
        );
    }

    #[test]
    fn tag_at_finds_tags_under_the_cursor() {
        let l = chars("see #work/x and #123");
        assert_eq!(tag_at(&l, 5).as_deref(), Some("work/x"));
        assert_eq!(tag_at(&l, 16), None);
        assert_eq!(tag_at(&l, 1), None);
        assert_eq!(tag_at(&chars("a#b"), 2), None);
    }

    #[test]
    fn highlights_mark_headings_tags_and_done_tasks() {
        let lines = Lines::from("# Title #tag\n- [x] done\n");
        let hl = highlights(&lines, &Scheme::mono());
        let at = |row, start, end| {
            hl.iter()
                .find(|h| h.start == Index2::new(row, start) && h.end == Index2::new(row, end))
                .unwrap_or_else(|| panic!("no range {row}:{start}..={end}"))
        };
        assert_eq!(at(0, 8, 11).style.fg, Some(dim()));
        assert!(at(0, 2, 11).style.add_modifier.contains(Modifier::BOLD));
        assert!(
            at(1, 0, 9)
                .style
                .add_modifier
                .contains(Modifier::CROSSED_OUT)
        );
    }

    #[test]
    fn link_at_finds_wikilinks() {
        let l = chars("go [[Weekly plan]] now");
        assert_eq!(link_at(&l, 3).as_deref(), Some("Weekly plan"));
        assert_eq!(link_at(&l, 17).as_deref(), Some("Weekly plan"));
        assert_eq!(link_at(&l, 18), None);
        assert_eq!(link_at(&chars("[[]]"), 1), None);
        assert_eq!(link_at(&chars("[[ a ]]"), 2).as_deref(), Some("a"));
        assert_eq!(link_at(&chars("[[a[b]]"), 2), None);
    }

    #[test]
    fn highlights_underline_wikilinks() {
        let lines = Lines::from("see [[B]]\n");
        let hl = highlights(&lines, &Scheme::mono());
        let at = |start, end| {
            hl.iter()
                .find(|h| h.start == Index2::new(0, start) && h.end == Index2::new(0, end))
                .unwrap_or_else(|| panic!("no range {start}..={end}"))
        };
        assert_eq!(at(4, 5).style.fg, Some(dim()));
        assert!(at(6, 6).style.add_modifier.contains(Modifier::UNDERLINED));
        assert_eq!(at(7, 8).style.fg, Some(dim()));
    }

    #[test]
    fn color_scheme_uses_theme_tokens() {
        let lines = Lines::from("# T `c` #x [[L]]\n- [ ] t\n");
        let hl = highlights(&lines, &Scheme::color());
        let at = |row, start, end| {
            hl.iter()
                .find(|h| h.start == Index2::new(row, start) && h.end == Index2::new(row, end))
                .unwrap_or_else(|| panic!("no range {row}:{start}..={end}"))
        };
        let title = at(0, 2, 15).style;
        assert_eq!(title.fg, Some(accent()));
        assert!(title.add_modifier.contains(Modifier::BOLD));
        assert_eq!(at(0, 5, 5).style.fg, Some(code()));
        assert_eq!(at(0, 8, 9).style.fg, Some(tag()));
        let link = at(0, 13, 13).style;
        assert_eq!(link.fg, Some(accent()));
        assert!(link.add_modifier.contains(Modifier::UNDERLINED));
        assert_eq!(at(1, 0, 1).style.fg, Some(dim()));
        assert_eq!(at(1, 2, 4).style.fg, Some(accent()));
    }

    #[test]
    fn fenced_code_body_is_code_style() {
        let lines = Lines::from("```\nx\n```\n");
        let hl = highlights(&lines, &Scheme::color());
        let body = hl
            .iter()
            .find(|h| h.start == Index2::new(1, 0))
            .expect("fenced body range");
        assert_eq!(body.style, Scheme::color().code);
    }
}
