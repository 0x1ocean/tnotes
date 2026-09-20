//! Live markdown highlighting for the editor, expressed as edtui `Highlight` ranges.
//!
//! edtui applies the *first* matching range to a cell, so inner elements (code, tags, emphasis)
//! are pushed before line-level styles (headings, quotes, done tasks).

use edtui::{EditorState, Highlight, Index2, Lines};
use ratatui::style::{Modifier, Style};

use crate::note::is_tag_char;
use crate::ui::theme::dim;

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
fn inline(line: &[char], from: usize, s: &mut Sink) {
    let dim_s = Style::new().fg(dim());
    let n = line.len();
    let mut i = from;
    while i < n {
        let c = line[i];
        match c {
            '`' => {
                if let Some(j) = (i + 1..n).find(|&j| line[j] == '`') {
                    s.push(i, j, dim_s);
                    i = j + 1;
                    continue;
                }
            }
            '[' => {
                // [text](url)
                if let Some(close) = (i + 1..n).find(|&j| line[j] == ']')
                    && line.get(close + 1) == Some(&'(')
                    && let Some(end) = (close + 2..n).find(|&j| line[j] == ')')
                {
                    s.push(i, i, dim_s);
                    s.push(
                        i + 1,
                        close - 1,
                        Style::new().add_modifier(Modifier::UNDERLINED),
                    );
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
                    s.push(i, i + body, dim_s);
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
pub fn highlights(lines: &Lines) -> Vec<Highlight> {
    let dim_s = Style::new().fg(dim());
    let bold = Style::new().add_modifier(Modifier::BOLD);
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
            s.push(0, n - 1, dim_s);
            continue;
        }
        // Headings: `#`s dim, text bold.
        let hashes = line.iter().take_while(|c| **c == '#').count();
        if (1..=6).contains(&hashes) && line.get(hashes) == Some(&' ') {
            s.push(0, hashes, dim_s);
            inline(line, hashes + 1, &mut s);
            s.push(hashes + 1, n - 1, bold);
            continue;
        }
        // Horizontal rule.
        if n >= 3 && (line.iter().all(|&c| c == '-') || line.iter().all(|&c| c == '*')) {
            s.push(0, n - 1, dim_s);
            continue;
        }
        // Blockquote: `>` dim, text italic.
        let indent = line.iter().take_while(|c| **c == ' ').count();
        if line.get(indent) == Some(&'>') {
            s.push(indent, indent, dim_s);
            inline(line, indent + 1, &mut s);
            s.push(
                indent + 1,
                n - 1,
                Style::new().add_modifier(Modifier::ITALIC),
            );
            continue;
        }
        // List item: marker and checkbox dim; done items dim + crossed out.
        if let Some(item) = list_line(line) {
            if item.done {
                s.push(0, n - 1, dim_s.add_modifier(Modifier::CROSSED_OUT));
                continue;
            }
            s.push(indent, item.text_start.saturating_sub(1), dim_s);
            inline(line, item.text_start, &mut s);
            continue;
        }
        inline(line, 0, &mut s);
        // First line without `#` is still the title.
        if row == 0 {
            s.push(0, n - 1, bold);
        }
    }
    out
}

pub fn refresh(editor: &mut EditorState) {
    editor.highlights = highlights(&editor.lines);
}
