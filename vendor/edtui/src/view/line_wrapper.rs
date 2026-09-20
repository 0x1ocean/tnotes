//! Soft wrapping at word boundaries.
//!
//! tnotes patch: upstream edtui wraps at the last character that fits. Here a row is broken
//! after the last space that fits; a word longer than the width is still split hard.
//! `wrap_line` (mouse mapping), `wrap_spans` (rendering) and `row_count` (scrolling) all use
//! the same `break_points`, so screen rows agree everywhere.

use crate::helper::char_width;
use ratatui_core::style::Style;
use ratatui_core::text::Span;

#[derive(Default)]
pub(crate) struct LineWrapper;

impl LineWrapper {
    /// Splits a given line width into multiple smaller widths, ensuring each width
    /// is no larger than the specified maximum width.
    pub(crate) fn determine_split(line_width: usize, max_width: usize) -> Vec<usize> {
        if line_width == 0 {
            return vec![0];
        }

        let mut remaining_width = line_width;
        let mut split_widths = Vec::new();

        while remaining_width > 0 {
            let current_chunk = std::cmp::min(remaining_width, max_width);
            split_widths.push(current_chunk);
            remaining_width = remaining_width.saturating_sub(max_width);
        }

        split_widths
    }

    /// Start index of every visual row of `line` (the first is always `0`).
    ///
    /// A row breaks after the last space that fits; the space stays at the end of the row
    /// so character counts add up to the line length (cursor mapping relies on that).
    pub(crate) fn break_points(line: &[char], max_width: usize, tab_width: usize) -> Vec<usize> {
        let max_width = max_width.max(1);
        let mut rows = vec![0];
        let mut row_start = 0;
        let mut width = 0;
        let mut last_space: Option<usize> = None;

        for (i, &ch) in line.iter().enumerate() {
            let w = char_width(ch, tab_width);
            if width + w > max_width && i > row_start {
                if ch == ' ' {
                    // The overflowing space hangs at the end of this row. When it is the
                    // last char an empty row follows so the end-of-line cursor lands there.
                    let bp = i + 1;
                    rows.push(bp);
                    row_start = bp;
                    width = 0;
                    last_space = None;
                    continue;
                }
                let bp = match last_space {
                    Some(sp) if sp >= row_start => sp + 1,
                    _ => i,
                };
                rows.push(bp);
                row_start = bp;
                width = line[bp..=i]
                    .iter()
                    .map(|&c| char_width(c, tab_width))
                    .sum();
                last_space = None;
                continue;
            }
            width += w;
            if ch == ' ' {
                last_space = Some(i);
            }
        }
        rows
    }

    /// Number of screen rows `line` occupies (at least 1).
    pub(crate) fn row_count(line: &[char], max_width: usize, tab_width: usize) -> usize {
        if line.is_empty() {
            return 1;
        }
        Self::break_points(line, max_width, tab_width).len()
    }

    pub(crate) fn wrap_line(line: &[char], max_width: usize, tab_width: usize) -> Vec<Vec<char>> {
        if line.is_empty() {
            return Vec::new();
        }
        let bps = Self::break_points(line, max_width, tab_width);
        bps.iter()
            .enumerate()
            .map(|(k, &start)| {
                let end = bps.get(k + 1).copied().unwrap_or(line.len());
                line[start..end].to_vec()
            })
            .collect()
    }

    pub(crate) fn wrap_spans(
        spans: Vec<Span<'_>>,
        max_width: usize,
        tab_width: usize,
    ) -> Vec<Vec<Span<'_>>> {
        // Flatten to (char, style), wrap the chars, rebuild spans per row.
        let cells: Vec<(char, Style)> = spans
            .iter()
            .flat_map(|s| s.content.chars().map(move |c| (c, s.style)))
            .collect();
        let chars: Vec<char> = cells.iter().map(|(c, _)| *c).collect();
        if chars.is_empty() {
            return Vec::new();
        }
        let bps = Self::break_points(&chars, max_width, tab_width);
        bps.iter()
            .enumerate()
            .map(|(k, &start)| {
                let end = bps.get(k + 1).copied().unwrap_or(cells.len());
                let mut row: Vec<Span<'_>> = Vec::new();
                let mut text = String::new();
                let mut style: Option<Style> = None;
                for &(c, st) in &cells[start..end] {
                    if style != Some(st) {
                        if let Some(s) = style {
                            row.push(Span::styled(std::mem::take(&mut text), s));
                        }
                        style = Some(st);
                    }
                    text.push(c);
                }
                if let Some(s) = style {
                    row.push(Span::styled(text, s));
                }
                row
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn break_points_invariants() {
        let samples = [
            "hello big world", "a b c d e f g h i j k l m n o p", "   leading spaces here",
            "trailing spaces   ", "x", "", "abcdefghijklmnopqrstuvwxyz", "ab cd ef gh ij kl mn op qr st",
            "wide 你好世界 chars 你好 mixed words here", "tab\there\tand\tmore", "a  b   c    d     e",
        ];
        for s in samples {
            let line: Vec<char> = s.chars().collect();
            for w in 1..12 {
                let bps = LineWrapper::break_points(&line, w, 4);
                assert_eq!(bps[0], 0, "{s:?} w={w}");
                for k in 1..bps.len() {
                    assert!(bps[k] > bps[k - 1] && bps[k] <= line.len(), "{s:?} w={w} bps={bps:?}");
                }
                let rows = LineWrapper::wrap_line(&line, w, 4);
                let total: usize = rows.iter().map(Vec::len).sum();
                assert_eq!(total, line.len(), "{s:?} w={w}");
                for (k, r) in rows.iter().enumerate() {
                    // Only a trailing row after a hanging space may be empty.
                    assert!(!r.is_empty() || k + 1 == rows.len(), "{s:?} w={w}");
                    // each row fits unless it is a single overflowing char or ends with the hanging space
                    let width: usize = r.iter().map(|&c| char_width(c, 4)).sum();
                    let hanging = r.last() == Some(&' ');
                    assert!(width <= w || r.len() == 1 || hanging, "{s:?} w={w} row={:?}", r.iter().collect::<String>());
                }
            }
        }
    }

    #[test]
    fn wraps_at_spaces() {
        let line: Vec<char> = "hello big world".chars().collect();
        let rows = LineWrapper::wrap_line(&line, 9, 4);
        let rows: Vec<String> = rows.iter().map(|r| r.iter().collect()).collect();
        assert_eq!(rows, vec!["hello big ", "world"]);
        assert_eq!(rows.iter().map(String::len).sum::<usize>(), line.len());
    }

    #[test]
    fn splits_long_words_hard() {
        let line: Vec<char> = "abcdefghij".chars().collect();
        let rows = LineWrapper::wrap_line(&line, 4, 4);
        let rows: Vec<String> = rows.iter().map(|r| r.iter().collect()).collect();
        assert_eq!(rows, vec!["abcd", "efgh", "ij"]);
    }

    #[test]
    fn spans_keep_styles() {
        let spans = vec![Span::raw("Hello "), Span::raw("World")];
        let wrapped = LineWrapper::wrap_spans(spans, 6, 0);
        assert_eq!(wrapped[0], vec![Span::raw("Hello ")]);
        assert_eq!(wrapped[1], vec![Span::raw("World")]);
    }

    #[test]
    fn determine_split_counts() {
        assert_eq!(LineWrapper::determine_split(5, 3), vec![3, 2]);
        assert_eq!(LineWrapper::determine_split(6, 3), vec![3, 3]);
    }
}
