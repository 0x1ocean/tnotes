//! Full-screen settings page: sections column on the left, rows on the right.

use super::*;
use crate::app::{SETTINGS_SECTIONS, SettingKind};

const SECTIONS_W: u16 = 20;
const LABEL_W: usize = 22;

pub(super) fn draw_settings_page(
    f: &mut Frame,
    app: &App,
    area: Rect,
    status: Rect,
    hits: &mut Hits,
) {
    let dim_s = Style::new().fg(dim());
    let page = app.settings.unwrap_or_default();
    if area.width < 30 || area.height < 4 {
        return;
    }

    // Header row: `‹ back` is a button (same gray block as the panel switch).
    let button = Style::new().bg(dim());
    f.render_widget(
        Line::from(vec![
            Span::styled(" ‹ back ", button),
            Span::styled("  settings", Style::new().add_modifier(Modifier::BOLD)),
        ]),
        Rect::new(area.x, area.y, area.width, 1),
    );
    hits.back = Rect::new(area.x, area.y, 8.min(area.width), 1);

    let body = Rect::new(area.x, area.y + 1, area.width, area.height - 1);
    let sections_w = SECTIONS_W.min(body.width / 3);
    let sep_x = body.x + sections_w;
    draw_vline(f, sep_x, area.y, area.height);

    // Sections.
    hits.settings_sections.clear();
    let mut lines = Vec::with_capacity(SETTINGS_SECTIONS.len());
    for (i, name) in SETTINGS_SECTIONS.iter().enumerate() {
        let selected = i == page.section;
        let marker = if selected {
            Span::styled("▎", dim_s)
        } else {
            Span::raw(" ")
        };
        let style = if selected {
            Style::new().bg(dim())
        } else {
            Style::new()
        };
        let text = format!("{name:<w$}", w = sections_w as usize - 2);
        lines.push(Line::from(vec![marker, Span::styled(text, style)]));
        hits.settings_sections
            .push(Rect::new(body.x, body.y + i as u16, sections_w, 1));
    }
    f.render_widget(
        Paragraph::new(lines),
        Rect::new(body.x, body.y, sections_w, body.height),
    );

    // Rows. Selected row gets a `▎` marker; values are rendered as controls.
    let content = Rect::new(sep_x + 2, body.y, body.width - sections_w - 3, body.height);
    let rows = app.settings_rows();
    hits.settings_rows.clear();
    hits.settings_controls.clear();
    let mut lines: Vec<Line> = Vec::with_capacity(rows.len() + 2);
    let w = content.width as usize;
    let on = Style::new().bg(dim());
    let off = dim_s;
    // Scroll so the selected row stays visible (hint takes the last two lines).
    let cap = (content.height as usize).saturating_sub(2).max(1);
    let offset = page
        .row
        .saturating_sub(cap - 1)
        .min(rows.len().saturating_sub(cap));
    for (i, row) in rows.iter().enumerate().skip(offset).take(cap) {
        let selected = i == page.row && row.editable();
        let y = content.y + (i - offset) as u16;
        let marker = if selected { "▎" } else { " " };
        let mut spans: Vec<Span> = vec![Span::styled(marker.to_string(), dim_s)];
        let mut x = content.x + 1;
        let push = |spans: &mut Vec<Span>, text: String, style: Style, x: &mut u16| -> Rect {
            let width = text.chars().count() as u16;
            let rect = Rect::new(*x, y, width, 1);
            spans.push(Span::styled(text, style));
            *x += width;
            rect
        };
        match &row.kind {
            SettingKind::Choice { options, current } => {
                let label = format!("{:<w$}", truncate(&row.label, LABEL_W), w = LABEL_W);
                push(&mut spans, label, Style::new(), &mut x);
                for (j, opt) in options.iter().enumerate() {
                    let style = if j == *current { on } else { off };
                    let rect = push(&mut spans, format!(" {opt} "), style, &mut x);
                    hits.settings_controls.push((i, j, rect));
                    push(&mut spans, " ".into(), Style::new(), &mut x);
                }
            }
            SettingKind::Number(value) => {
                let label = format!("{:<w$}", truncate(&row.label, LABEL_W), w = LABEL_W);
                push(&mut spans, label, Style::new(), &mut x);
                let l = push(&mut spans, " ‹ ".into(), off, &mut x);
                hits.settings_controls.push((i, 0, l));
                push(&mut spans, value.clone(), Style::new(), &mut x);
                let r = push(&mut spans, " › ".into(), off, &mut x);
                hits.settings_controls.push((i, 1, r));
            }
            SettingKind::Text(value) => {
                let label = format!("{:<w$}", truncate(&row.label, LABEL_W), w = LABEL_W);
                push(&mut spans, label, Style::new(), &mut x);
                let avail = w.saturating_sub(LABEL_W + 10);
                let rect = push(
                    &mut spans,
                    format!(" {} ", truncate(value, avail)),
                    on,
                    &mut x,
                );
                hits.settings_controls.push((i, 0, rect));
                push(&mut spans, "  edit…".into(), off, &mut x);
            }
            SettingKind::Root(count) => {
                let x_btn = if app.cfg.roots_fixed { "" } else { " × " };
                let right = count.chars().count() + 2 + x_btn.len();
                let label = truncate(&row.label, w.saturating_sub(right + 2));
                let pad = w.saturating_sub(label.chars().count() + right + 1);
                push(
                    &mut spans,
                    format!("{label}{}", " ".repeat(pad)),
                    Style::new(),
                    &mut x,
                );
                push(&mut spans, count.clone(), off, &mut x);
                if !x_btn.is_empty() {
                    let rect = push(&mut spans, x_btn.into(), off, &mut x);
                    hits.settings_controls.push((i, 0, rect));
                }
            }
            SettingKind::Action => {
                push(&mut spans, row.label.clone(), off, &mut x);
            }
            SettingKind::Info(value) => {
                let label = format!("{:<w$}", truncate(&row.label, LABEL_W), w = LABEL_W);
                push(&mut spans, label, off, &mut x);
                push(
                    &mut spans,
                    truncate(value, w.saturating_sub(LABEL_W + 2)),
                    Style::new(),
                    &mut x,
                );
            }
        }
        lines.push(Line::from(spans));
        hits.settings_rows
            .push((i, Rect::new(content.x, y, content.width, 1)));
    }
    let hint = app.settings_hint();
    if !hint.is_empty() {
        lines.push(Line::raw(""));
        lines.push(Line::from(Span::styled(format!(" {hint}"), dim_s)));
    }
    f.render_widget(Paragraph::new(lines), content);

    // Status bar: the button on the left mirrors the main screen's panel switch.
    let cfg = config::config_path()
        .map(|p| config::contract_tilde(&p))
        .unwrap_or_default();
    let left = format!("  changes apply immediately · saved to {cfg}");
    let right = "esc back ";
    let gap = (status.width as usize).saturating_sub(4 + left.chars().count() + right.len());
    f.render_widget(
        Line::from(vec![
            Span::styled(" ‹‹ ", button),
            Span::styled(left, dim_s),
            Span::raw(" ".repeat(gap)),
            Span::styled(right, dim_s),
        ]),
        status,
    );
    hits.settings_back = Rect::new(
        status.right().saturating_sub(right.len() as u16),
        status.y,
        right.len() as u16,
        1,
    );
}
