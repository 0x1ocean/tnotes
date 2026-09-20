//! Bottom status bar: `<< context · message` on the left, `this note · app` on the right.

use super::*;

/// A clickable segment: text plus the `Hits` rect it should fill.
/// `alts` are progressively shorter forms (empty string = drop); lower `rank` shrinks first.
struct Seg<'a> {
    text: String,
    style: Style,
    hit: Option<&'a mut Rect>,
    rank: u8,
    alts: Vec<String>,
}

impl Seg<'_> {
    fn shrink(mut self, rank: u8, alts: Vec<String>) -> Self {
        self.rank = rank;
        self.alts = alts;
        self
    }
}

fn seg(text: impl Into<String>, style: Style) -> Seg<'static> {
    Seg {
        text: text.into(),
        style,
        hit: None,
        rank: u8::MAX,
        alts: Vec::new(),
    }
}

fn hit<'a>(text: impl Into<String>, style: Style, rect: &'a mut Rect) -> Seg<'a> {
    Seg {
        text: text.into(),
        style,
        hit: Some(rect),
        rank: u8::MAX,
        alts: Vec::new(),
    }
}

/// Lay out segments left-to-right from `x`, separated by ` · `, filling hit rects.
fn place(segs: Vec<Seg>, spans: &mut Vec<Span<'static>>, mut x: u16, y: u16, sep: Style) -> u16 {
    let n = segs.len();
    for (i, s) in segs.into_iter().enumerate() {
        let w = s.text.chars().count() as u16;
        if let Some(r) = s.hit {
            *r = Rect::new(x, y, w, 1);
        }
        spans.push(Span::styled(s.text, s.style));
        x += w;
        if i + 1 < n {
            spans.push(Span::styled(" · ", sep));
            x += 3;
        }
    }
    x
}

fn width(segs: &[Seg]) -> usize {
    segs.iter().map(|s| s.text.chars().count()).sum::<usize>() + 3 * segs.len().saturating_sub(1)
}

pub(super) fn draw_status(f: &mut Frame, app: &App, area: Rect, hits: &mut Hits) {
    let dim_s = Style::new().fg(dim());
    let n = app.visible.len();

    // Left: where am I.
    let mut left: Vec<Seg> = Vec::new();
    left.push(hit(app.filter_label(), dim_s, &mut hits.status_filter));
    // `All notes · 8` / `notes · 8` rather than `notes · 8 notes`.
    let label = app.filter_label();
    let count = if label.to_lowercase().contains("note") {
        n.to_string()
    } else {
        format!("{n} {}", if n == 1 { "note" } else { "notes" })
    };
    left.push(seg(count, dim_s).shrink(4, vec![n.to_string()]));
    if app.filter == Filter::Trash {
        left.push(seg("r restore · d delete", dim_s).shrink(3, vec!["r · d".into()]));
    } else {
        let sort = match app.sort {
            SortMode::Modified => "by modified",
            SortMode::Title => "by title",
            SortMode::Created => "by created",
        };
        left.push(hit(sort, dim_s, &mut hits.status_sort).shrink(2, vec![String::new()]));
    }
    if let Some((kind, text)) = app.status_text() {
        let color = match kind {
            StatusKind::Conflict | StatusKind::Failed => err(),
            StatusKind::Reloaded | StatusKind::Info => dim(),
        };
        // Message follows its dot with a plain space, not ` · `.
        left.push(seg(format!("● {text}"), Style::new().fg(color)).shrink(6, vec![String::new()]));
    }

    // Right: this note, then the app.
    let mut right: Vec<Seg> = Vec::new();
    if let Some(tab) = app.active_tab() {
        let (color, state) = if tab.dirty {
            (warn(), "unsaved")
        } else {
            (ok(), "saved")
        };
        right.push(seg(format!("● {state}"), Style::new().fg(color)).shrink(7, vec!["●".into()]));
        if let Some(note) = app.current_note() {
            let words = note.text.split_whitespace().count();
            right.push(seg(format!("{words} words"), dim_s).shrink(1, vec![String::new()]));
        }
        if let Some(idx) = app.store.notes.iter().position(|n| n.path == tab.path) {
            let n = app.store.backlinks(idx).len();
            if n > 0 {
                right.push(
                    hit(format!("↩ {n}"), dim_s, &mut hits.status_backlinks)
                        .shrink(1, vec![String::new()]),
                );
            }
        }
        if app.tab_in_trash(app.active) {
            right.push(seg("read-only", dim_s));
        } else if tab.preview {
            right.push(seg("preview · ctrl+l edit", dim_s).shrink(3, vec!["preview".into()]));
        } else {
            let (mode, short) = match app.keys {
                EditorKeys::Emacs => ("emacs".to_string(), "emacs".to_string()),
                EditorKeys::Vim => (
                    format!(
                        "vim {}",
                        match tab.editor.mode {
                            EditorMode::Normal => "NORMAL",
                            EditorMode::Insert => "INSERT",
                            EditorMode::Visual => "VISUAL",
                            EditorMode::Search => "SEARCH",
                        }
                    ),
                    "vim".to_string(),
                ),
            };
            right.push(hit(mode, dim_s, &mut hits.status_keys).shrink(5, vec![short]));
        }
    }
    right.push(hit("? help", dim_s, &mut hits.status_help).shrink(4, vec!["?".into()]));
    right.push(hit("settings", dim_s, &mut hits.settings_link).shrink(4, vec!["⚙".into()]));

    // The switch is a small button: ` << ` on the same gray block used for selections.
    let glyph = app.panel_switch_glyph();
    let toggle = format!(" {glyph} ");
    let prefix_w = toggle.len() + 1;
    let avail = (area.width as usize).saturating_sub(prefix_w + 1);
    // Shrink lowest-rank segments first until both groups fit.
    while width(&left) + width(&right) + 2 > avail {
        let next = left
            .iter_mut()
            .chain(right.iter_mut())
            .filter(|s| !s.alts.is_empty())
            .min_by_key(|s| s.rank);
        match next {
            Some(s) => s.text = s.alts.remove(0),
            None => break,
        }
    }
    left.retain(|s| !s.text.is_empty());
    right.retain(|s| !s.text.is_empty());

    let left_w = prefix_w + width(&left);
    let right_w = width(&right) + 1;
    let gap = (area.width as usize).saturating_sub(left_w + right_w);

    let button = if glyph.trim().is_empty() {
        Style::new()
    } else {
        Style::new().bg(dim())
    };
    let mut spans: Vec<Span> = vec![Span::styled(toggle.clone(), button), Span::raw(" ")];
    let x = place(left, &mut spans, area.x + prefix_w as u16, area.y, dim_s);
    spans.push(Span::raw(" ".repeat(gap)));
    place(right, &mut spans, x + gap as u16, area.y, dim_s);
    spans.push(Span::raw(" "));
    f.render_widget(Line::from(spans), area);
}
