//! Left column: folder/tag tree and the notes list.

use super::*;

pub(super) fn draw_navigator(f: &mut Frame, app: &mut App, area: Rect, hits: &mut Hits) {
    if area.width < 4 || area.height < 3 {
        return;
    }
    let mut inner = area;
    let narrow = app.layout == Layout::Narrow;
    if narrow {
        // Header row: current filter; tap folds/unfolds the tree underneath.
        let glyph = if app.tree_folded { "▸" } else { "▾" };
        let text = format!(" {glyph} {}", app.filter_label());
        let header = Rect::new(inner.x, inner.y, inner.width, 1);
        f.render_widget(
            Span::styled(
                truncate(&text, inner.width as usize),
                Style::new().fg(dim()),
            ),
            header,
        );
        hits.tree_header = header;
        inner = Rect::new(inner.x, inner.y + 1, inner.width, inner.height - 1);
    }
    let tree_h = if narrow && app.tree_folded {
        0
    } else {
        (app.tree_rows.len() as u16).min(inner.height / 2)
    };
    if tree_h > 0 {
        draw_tree(
            f,
            app,
            Rect::new(inner.x, inner.y, inner.width, tree_h),
            hits,
        );
    }
    let sep = Rect::new(inner.x, inner.y + tree_h, inner.width, 1);
    f.render_widget(
        Span::styled(
            format!(" {}", "─".repeat(inner.width as usize - 2)),
            Style::new().fg(dim()),
        ),
        sep,
    );
    let list = Rect::new(inner.x, sep.y + 1, inner.width, inner.height - tree_h - 1);
    draw_list(f, app, list, hits);
}

pub(super) fn draw_tree(f: &mut Frame, app: &mut App, area: Rect, hits: &mut Hits) {
    let height = area.height as usize;
    if height == 0 {
        return;
    }
    if app.tree_sel < app.tree_offset {
        app.tree_offset = app.tree_sel;
    } else if app.tree_sel >= app.tree_offset + height {
        app.tree_offset = app.tree_sel + 1 - height;
    }
    let offset = app.tree_offset.min(app.tree_rows.len().saturating_sub(1));
    app.tree_offset = offset;

    let focused = app.focus == Pane::Tree;
    let inner_w = area.width as usize - 1;
    let mut lines: Vec<Line> = Vec::with_capacity(height);
    hits.tree = area;
    hits.tree_rows.clear();
    for (i, row) in app.tree_rows.iter().enumerate().skip(offset).take(height) {
        let active = row.filter().as_ref() == Some(&app.filter);
        let marker = if active {
            Span::styled("▎", Style::new().fg(dim()))
        } else {
            Span::raw(" ")
        };
        let glyph = |has: bool, key: String, depth: usize| {
            let g = if !has {
                "  "
            } else if app.collapsed.contains(&key) {
                "▸ "
            } else {
                "▾ "
            };
            format!("{}{g}", " ".repeat(depth * TREE_INDENT as usize))
        };
        let (label, count, style): (String, Option<usize>, Style) = match row {
            TreeRow::All => (
                "All notes".into(),
                Some(app.store.notes.len()),
                Style::new(),
            ),
            TreeRow::Trash => ("Trash".into(), Some(app.store.trash.len()), Style::new()),
            TreeRow::Header("tags") => (
                format!(
                    "{} tags",
                    if app.collapsed.contains(crate::app::TAGS_KEY) {
                        "▸"
                    } else {
                        "▾"
                    }
                ),
                None,
                Style::new().fg(dim()),
            ),
            TreeRow::Header(h) => (h.to_string(), None, Style::new().fg(dim())),
            TreeRow::Folder(d) => (
                format!(
                    "{}{}",
                    glyph(d.has_children, crate::index::dir_key(&d.dir), d.depth),
                    d.name
                ),
                Some(d.count),
                Style::new(),
            ),
            TreeRow::Tag(t) => (
                format!(
                    "{}{}",
                    glyph(t.has_children, crate::index::tag_key(&t.path), t.depth),
                    t.name
                ),
                Some(t.count),
                Style::new(),
            ),
        };
        let count_s = if app.cfg.appearance.counts {
            count.map(|c| c.to_string()).unwrap_or_default()
        } else {
            String::new()
        };
        let label_w = inner_w.saturating_sub(1 + count_s.len() + 1);
        let label = truncate(&label, label_w);
        let pad = label_w.saturating_sub(label.chars().count());
        let mut style = style;
        // Selection is a gray block: never combine it with gray text (dim on dim vanishes).
        let selected = focused && i == app.tree_sel && row.selectable();
        let count_style = if selected {
            Style::new().bg(dim())
        } else {
            Style::new().fg(dim())
        };
        if selected {
            style = Style::new().bg(dim());
        }
        lines.push(Line::from(vec![
            marker,
            Span::styled(format!("{label}{} ", " ".repeat(pad)), style),
            Span::styled(count_s, count_style),
        ]));
        let y = area.y + (i - offset) as u16;
        hits.tree_rows.push(Rect::new(area.x, y, area.width, 1));
    }
    f.render_widget(Paragraph::new(lines), area);
}

pub(super) fn date_of(note: &Note, sort: SortMode, dates: Dates) -> String {
    let when = if sort == SortMode::Created {
        note.created
    } else {
        note.modified
    };
    let d = DateTime::<Local>::from(when).date_naive();
    if dates == Dates::Absolute {
        return d.format("%Y-%m-%d").to_string();
    }
    let today = Local::now().date_naive();
    if d == today {
        "today".into()
    } else if today.pred_opt() == Some(d) {
        "yesterday".into()
    } else if d.year() == today.year() {
        d.format("%d %b").to_string().to_lowercase()
    } else {
        d.format("%b %Y").to_string().to_lowercase()
    }
}

pub(super) fn draw_list(f: &mut Frame, app: &mut App, inner: Rect, hits: &mut Hits) {
    let focused = app.focus == Pane::List;
    if inner.width < 4 || inner.height < 1 {
        return;
    }
    hits.list = inner;
    hits.list_rows.clear();

    // Row 0: search field.
    let search = Rect::new(inner.x, inner.y, inner.width, 1);
    hits.search = search;
    if app.search_active {
        let field = Rect::new(search.x + 3, search.y, search.width - 3, 1);
        f.render_widget(Span::styled(" / ", Style::new().fg(dim())), search);
        render_field(f, &mut app.query, field);
    } else {
        let q = app.query_text();
        let text = if q.is_empty() {
            " / search".to_string()
        } else {
            format!(" / {q}")
        };
        f.render_widget(
            Span::styled(
                truncate(&text, inner.width as usize),
                Style::new().fg(dim()),
            ),
            search,
        );
    }

    let body = Rect::new(
        inner.x,
        inner.y + 1,
        inner.width,
        inner.height.saturating_sub(1),
    );
    if body.height == 0 {
        return;
    }
    if app.visible.is_empty() {
        let q = app.query_text();
        let msg: Vec<String> = if !q.trim().is_empty() {
            vec!["nothing matches".into()]
        } else {
            match &app.filter {
                Filter::All => vec!["no notes yet".into(), "ctrl+t to create".into()],
                Filter::Trash => vec!["trash is empty".into()],
                Filter::Tag(t) => vec![format!("no notes with #{t}")],
                Filter::Folder(_) => vec!["empty folder".into(), "ctrl+t to create".into()],
            }
        };
        centered_text(f, body, msg);
        return;
    }

    let compact = app.cfg.appearance.compact;
    let item_h = if compact { 1 } else { 2 };
    let cap = (body.height as usize / item_h).max(1);
    if app.list_sel < app.list_offset {
        app.list_offset = app.list_sel;
    } else if app.list_sel >= app.list_offset + cap {
        app.list_offset = app.list_sel + 1 - cap;
    }
    let offset = app.list_offset;
    let notes = app.active_notes();
    let dates = app.cfg.appearance.dates;
    let w = body.width as usize - 1;
    let mut lines: Vec<Line> = Vec::with_capacity(cap * item_h);
    for (row, &idx) in app.visible.iter().enumerate().skip(offset).take(cap) {
        let note = &notes[idx];
        let selected = row == app.list_sel;
        let hilite = selected && focused;
        let title_style = if hilite {
            Style::new().bg(dim())
        } else if selected {
            Style::new().add_modifier(Modifier::BOLD)
        } else {
            Style::new()
        };
        let sub_style = if hilite {
            Style::new().bg(dim())
        } else {
            Style::new().fg(dim())
        };
        let date = date_of(note, app.sort, dates);
        if compact {
            // ` title ……… date`
            let date_w = date.chars().count();
            let title = truncate(&note.title, w.saturating_sub(date_w + 2));
            let pad = w.saturating_sub(title.chars().count() + date_w + 1);
            lines.push(Line::from(vec![
                Span::styled(format!(" {title}{}", " ".repeat(pad)), title_style),
                Span::styled(format!("{date} "), sub_style),
            ]));
        } else {
            let title = truncate(&note.title, w);
            let pad = w.saturating_sub(title.chars().count());
            lines.push(Line::from(Span::styled(
                format!(" {title}{}", " ".repeat(pad)),
                title_style,
            )));
            let preview = note.preview();
            let sub = if preview.is_empty() {
                date
            } else {
                format!("{date} · {preview}")
            };
            let sub = truncate(&sub, w);
            let pad = w.saturating_sub(sub.chars().count());
            lines.push(Line::from(Span::styled(
                format!(" {sub}{}", " ".repeat(pad)),
                sub_style,
            )));
        }
        let y = body.y + ((row - offset) * item_h) as u16;
        hits.list_rows.push(Rect::new(
            body.x,
            y,
            body.width,
            (item_h as u16).min(body.bottom().saturating_sub(y)),
        ));
    }
    f.render_widget(Paragraph::new(lines), body);
}

pub(super) fn centered_text(f: &mut Frame, area: Rect, lines: Vec<String>) {
    let n = lines.len() as u16;
    let top = area.y + area.height.saturating_sub(n) / 2;
    let rect = Rect::new(area.x, top, area.width, n.min(area.height));
    let text: Vec<Line> = lines
        .into_iter()
        .map(|l| Line::from(Span::styled(l, Style::new().fg(dim()))))
        .collect();
    f.render_widget(Paragraph::new(text).alignment(Alignment::Center), rect);
}
