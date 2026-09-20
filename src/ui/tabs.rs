//! Tab bar above the editor.

use super::*;

pub(super) fn draw_tabs(f: &mut Frame, app: &App, area: Rect, hits: &mut Hits) {
    let dim_s = Style::new().fg(dim());
    let plus = " + ";
    let width = area.width as usize;
    // Title length adapts to the bar: share the width between tabs, minus per-tab chrome
    // (` N `, gaps, ` × ` on the active one), and stay within a readable range.
    let n = app.tabs.len().max(1);
    let chrome = 6;
    let per_tab = width.saturating_sub(plus.len() + 4) / n;
    let title_w = per_tab.saturating_sub(chrome).clamp(6, 28);
    if app.tabs.is_empty() {
        let mut spans = vec![Span::styled(" no open notes ", dim_s)];
        let gap = width.saturating_sub(15 + plus.len());
        spans.push(Span::raw(" ".repeat(gap)));
        spans.push(Span::styled(plus, dim_s));
        f.render_widget(Line::from(spans), area);
        hits.tab_new = Rect::new(
            area.right().saturating_sub(plus.len() as u16),
            area.y,
            plus.len() as u16,
            1,
        );
        return;
    }

    // Per-tab spans and widths.
    let mut items: Vec<(Vec<Span>, usize)> = Vec::with_capacity(app.tabs.len());
    for (i, tab) in app.tabs.iter().enumerate() {
        let title = app
            .tab_note(i)
            .map(|n| n.title.clone())
            .unwrap_or_else(|| "?".into());
        let active = i == app.active;
        let mut style = if active {
            Style::new().add_modifier(Modifier::REVERSED | Modifier::BOLD)
        } else {
            dim_s
        };
        if !tab.pinned {
            style = style.add_modifier(Modifier::ITALIC);
        }
        let mut spans = Vec::new();
        let numbered = app.cfg.appearance.tab_numbers == TabNumbers::Always || app.tabs.len() > 1;
        let text = if numbered {
            format!(" {} {}", i + 1, truncate(&title, title_w))
        } else {
            format!(" {}", truncate(&title, title_w))
        };
        spans.push(Span::styled(text, style));
        if tab.dirty {
            spans.push(Span::styled(
                " ●",
                if active {
                    style
                } else {
                    Style::new().fg(warn())
                },
            ));
        }
        if active {
            spans.push(Span::styled(" × ", style.remove_modifier(Modifier::BOLD)));
        } else {
            spans.push(Span::styled(" ", style));
        }
        spans.push(Span::raw(" "));
        let w: usize = spans.iter().map(|s| s.width()).sum();
        items.push((spans, w));
    }

    // Window of tabs containing the active one.
    let n = items.len();
    let widths: Vec<usize> = items.iter().map(|(_, w)| *w).collect();
    let sum = |a: usize, b: usize| widths[a..=b].iter().sum::<usize>();
    let fit = |avail: usize| -> (usize, usize) {
        let mut start = 0;
        while start < app.active && sum(start, app.active) > avail {
            start += 1;
        }
        let mut end = app.active;
        while end + 1 < n && sum(start, end + 1) <= avail {
            end += 1;
        }
        (start, end)
    };
    let avail = width.saturating_sub(plus.len());
    let (mut start, mut end) = fit(avail);
    let clipped = start > 0 || end + 1 < n;
    if clipped {
        (start, end) = fit(avail.saturating_sub(2));
    }

    let mut spans: Vec<Span> = Vec::new();
    let mut x = area.x;
    if start > 0 {
        spans.push(Span::styled("‹", dim_s));
        x += 1;
    }
    for (i, (item, w)) in items.into_iter().enumerate() {
        if i < start || i > end {
            continue;
        }
        let rect = Rect::new(x, area.y, w as u16, 1);
        if i == app.active {
            // " × " sits before the trailing gap space.
            hits.tab_close = Rect::new(x + w as u16 - 4, area.y, 3, 1);
            hits.tabs
                .push((i, Rect::new(x, area.y, (w as u16).saturating_sub(4), 1)));
        } else {
            hits.tabs.push((i, rect));
        }
        spans.extend(item);
        x += w as u16;
    }
    if end + 1 < n {
        spans.push(Span::styled("›", dim_s));
        x += 1;
    }
    let used = (x - area.x) as usize;
    spans.push(Span::raw(
        " ".repeat(width.saturating_sub(used + plus.len())),
    ));
    spans.push(Span::styled(plus, dim_s));
    hits.tab_new = Rect::new(
        area.right().saturating_sub(plus.len() as u16),
        area.y,
        plus.len() as u16,
        1,
    );
    f.render_widget(Line::from(spans), area);
}
