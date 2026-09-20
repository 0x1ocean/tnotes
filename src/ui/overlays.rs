//! Centered overlays: help, confirm, prompt, picker, settings, context menu.

use super::*;

pub(super) fn draw_confirm(
    f: &mut Frame,
    app: &App,
    area: Rect,
    action: &ConfirmAction,
    hits: &mut Hits,
) {
    let question = match action {
        ConfirmAction::Purge(_) => "delete this note permanently?".to_string(),
        ConfirmAction::DeleteFolder(d) => format!(
            "delete folder \"{}\" and move its notes to trash?",
            d.file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default()
        ),
        ConfirmAction::RemoveRoot(i) => format!(
            "remove \"{}\" from tnotes? (files stay on disk)",
            app.store
                .roots
                .get(*i)
                .map(|r| r.name.clone())
                .unwrap_or_default()
        ),
    };
    let yes = " y yes ";
    let no = " esc no ";
    let w = (question.chars().count() as u16 + 4)
        .max(yes.len() as u16 + no.len() as u16 + 6)
        .min(area.width);
    let rect = centered(area, w, 4);
    f.render_widget(Clear, rect);
    let block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(err()));
    let inner = block.inner(rect);
    f.render_widget(block, rect);
    hits.overlay = rect;
    hits.overlay_rows.clear();
    f.render_widget(
        Span::raw(format!(
            " {}",
            truncate(&question, inner.width as usize - 2)
        )),
        Rect::new(inner.x, inner.y, inner.width, 1),
    );
    // Buttons on the second row: `y yes` (accent) and `esc no`.
    let by = inner.y + 1;
    let yes_rect = Rect::new(inner.x + 1, by, yes.len() as u16, 1);
    let no_rect = Rect::new(yes_rect.right() + 2, by, no.len() as u16, 1);
    f.render_widget(
        Span::styled(yes, Style::new().bg(err()).add_modifier(Modifier::BOLD)),
        yes_rect,
    );
    f.render_widget(Span::styled(no, Style::new().bg(dim())), no_rect);
    hits.overlay_rows.push(yes_rect);
    hits.overlay_rows.push(no_rect);
}

pub(super) fn draw_prompt(
    f: &mut Frame,
    app: &mut App,
    area: Rect,
    kind: &PromptKind,
    hits: &mut Hits,
) {
    let name = |p: &std::path::Path| {
        p.file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default()
    };
    let title = match kind {
        PromptKind::NewFolder(d) => format!(" new folder in {} ", app.store.folder_label(d)),
        PromptKind::RenameFolder(d) => format!(" rename {} ", name(d)),
        PromptKind::NewRoot(p) => format!(" new folder in {} ", config::contract_tilde(p)),
        PromptKind::Template => " new note template (\\n for newline) ".to_string(),
    };
    let has_cands = !app.prompt_candidates.is_empty();
    let rect = centered(area, 60, if has_cands { 4 } else { 3 });
    f.render_widget(Clear, rect);
    let block = overlay_block(title, " enter ok · esc cancel ".into());
    let inner = block.inner(rect);
    f.render_widget(block, rect);
    hits.overlay = rect;
    if inner.width < 3 || inner.height == 0 {
        return;
    }
    let field = Rect::new(inner.x + 1, inner.y, inner.width - 2, 1);
    render_field(f, &mut app.prompt, field);
    if has_cands && inner.height > 1 {
        let text = truncate(&app.prompt_candidates.join("  "), inner.width as usize - 2);
        f.render_widget(
            Span::styled(format!(" {text}"), Style::new().fg(dim())),
            Rect::new(inner.x, inner.y + 1, inner.width, 1),
        );
    }
}

pub(super) fn draw_picker(
    f: &mut Frame,
    app: &mut App,
    area: Rect,
    note: &std::path::Path,
    hits: &mut Hits,
) {
    let title = app
        .note_by_path(note)
        .map(|(n, _)| n.title.clone())
        .unwrap_or_default();
    let rect = centered(area, 60, 14);
    f.render_widget(Clear, rect);
    let block = overlay_block(
        format!(" move \"{}\" to ", truncate(&title, 40)),
        " enter move · esc cancel ".into(),
    );
    let inner = block.inner(rect);
    f.render_widget(block, rect);
    hits.overlay = rect;
    hits.overlay_rows.clear();
    if inner.width < 5 || inner.height < 2 {
        return;
    }
    f.render_widget(
        Span::styled(" / ", Style::new().fg(dim())),
        Rect::new(inner.x, inner.y, 3, 1),
    );
    render_field(
        f,
        &mut app.picker_query,
        Rect::new(inner.x + 3, inner.y, inner.width - 3, 1),
    );
    let body = Rect::new(inner.x, inner.y + 1, inner.width, inner.height - 1);
    let cap = body.height as usize;
    let offset = app.picker_sel.saturating_sub(cap - 1);
    let mut lines = Vec::with_capacity(cap);
    for (i, dir) in app.picker_rows.iter().enumerate().skip(offset).take(cap) {
        let label = truncate(&app.store.folder_label(dir), inner.width as usize - 2);
        if i == app.picker_sel {
            lines.push(Line::from(vec![
                Span::styled(" ▎", Style::new().fg(accent())),
                Span::styled(label, Style::new().add_modifier(Modifier::BOLD)),
            ]));
        } else {
            lines.push(Line::from(Span::raw(format!("  {label}"))));
        }
        hits.overlay_rows.push(Rect::new(
            body.x,
            body.y + (i - offset) as u16,
            body.width,
            1,
        ));
    }
    if app.picker_rows.is_empty() {
        lines.push(Line::from(Span::styled(
            "  no other folders",
            Style::new().fg(dim()),
        )));
    }
    f.render_widget(Paragraph::new(lines), body);
}

pub(super) fn draw_menu(f: &mut Frame, area: Rect, menu: &Menu, hits: &mut Hits) {
    let label_w = menu
        .items
        .iter()
        .map(|i| i.label.chars().count())
        .max()
        .unwrap_or(0);
    let has_keys = menu.items.iter().any(|i| i.key.is_some());
    // " label   k " + borders
    let inner_w = label_w + 2 + if has_keys { 4 } else { 0 };
    let w = (inner_w as u16 + 2).min(area.width);
    let h = (menu.items.len() as u16 + 2).min(area.height);
    let x = menu.at.x.min(area.right().saturating_sub(w));
    let y = if menu.at.y + 1 + h <= area.bottom() {
        menu.at.y + 1
    } else {
        menu.at.y.saturating_sub(h)
    };
    let rect = Rect::new(x, y.max(area.y), w, h);
    f.render_widget(Clear, rect);
    let block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(dim()));
    let inner = block.inner(rect);
    f.render_widget(block, rect);
    hits.overlay = rect;
    hits.overlay_rows.clear();
    let lines: Vec<Line> = menu
        .items
        .iter()
        .enumerate()
        .map(|(i, it)| {
            let selected = i == menu.sel;
            let base = if selected {
                Style::new().bg(dim())
            } else {
                Style::new()
            };
            let key = it
                .key
                .map(|k| {
                    if k == ' ' {
                        "␣".to_string()
                    } else {
                        k.to_string()
                    }
                })
                .unwrap_or_default();
            let pad = label_w.saturating_sub(it.label.chars().count());
            let mut spans = vec![Span::styled(
                format!(" {}{} ", it.label, " ".repeat(pad)),
                base,
            )];
            if has_keys {
                let key_style = if selected {
                    base
                } else {
                    Style::new().fg(dim())
                };
                spans.push(Span::styled(format!("{key:>3} "), key_style));
            }
            hits.overlay_rows
                .push(Rect::new(inner.x, inner.y + i as u16, inner.width, 1));
            Line::from(spans)
        })
        .collect();
    f.render_widget(Paragraph::new(lines), inner);
}

pub(super) fn draw_browser(f: &mut Frame, app: &mut App, area: Rect, hits: &mut Hits) {
    let dim_s = Style::new().fg(dim());
    let rect = centered(area, 66, 20);
    f.render_widget(Clear, rect);
    let title = if app.browser.first_run {
        " where are your notes? "
    } else {
        " add folder "
    };
    let block = overlay_block(
        title.into(),
        " enter open · tab add this folder · alt+. hidden · esc ".into(),
    );
    let inner = block.inner(rect);
    f.render_widget(block, rect);
    hits.overlay = rect;
    hits.overlay_rows.clear();
    hits.crumbs.clear();
    if inner.width < 10 || inner.height < 4 {
        return;
    }

    // Breadcrumb: ` / › home › roma › Documents `, each segment clickable.
    let mut spans: Vec<Span> = Vec::new();
    let mut x = inner.x + 1;
    let crumbs = app.browser.crumbs.clone();
    for (i, p) in crumbs.iter().enumerate() {
        let label = Browser::crumb_label(p);
        let last = i + 1 == crumbs.len();
        let style = if last {
            Style::new().add_modifier(Modifier::BOLD)
        } else {
            dim_s
        };
        let w = label.chars().count() as u16;
        hits.crumbs.push(Rect::new(x, inner.y, w, 1));
        spans.push(Span::styled(label, style));
        x += w;
        if !last {
            spans.push(Span::styled(" › ", dim_s));
            x += 3;
        }
    }
    f.render_widget(
        Line::from(spans),
        Rect::new(inner.x + 1, inner.y, inner.width - 2, 1),
    );

    // Filter field.
    let field_y = inner.y + 1;
    f.render_widget(
        Span::styled(" / ", dim_s),
        Rect::new(inner.x, field_y, 3, 1),
    );
    render_field(
        f,
        &mut app.browser.query,
        Rect::new(inner.x + 3, field_y, inner.width - 3, 1),
    );

    // Rows.
    let body = Rect::new(inner.x, inner.y + 2, inner.width, inner.height - 2);
    let cap = body.height as usize;
    let sel = app.browser.sel;
    let offset = sel.saturating_sub(cap - 1);
    let rows = app.browser.rows.clone();
    let here = config::contract_tilde(&app.browser.dir);
    let w = body.width as usize;
    let mut lines: Vec<Line> = Vec::with_capacity(cap);
    for (i, row) in rows.iter().enumerate().skip(offset).take(cap) {
        let base = if i == sel {
            Style::new().bg(dim())
        } else {
            Style::new()
        };
        let marker = Span::styled(if i == sel { "▎" } else { " " }, dim_s);
        let line = match row {
            DirRow::AddHere => {
                let text = format!(
                    " add this folder  {}",
                    truncate(&here, w.saturating_sub(20))
                );
                Line::from(vec![
                    marker,
                    Span::styled(
                        format!("{text:<w$}", w = w - 1),
                        base.add_modifier(Modifier::BOLD),
                    ),
                ])
            }
            DirRow::Up => Line::from(vec![
                marker,
                Span::styled(format!("{:<w$}", " ..", w = w - 1), base),
            ]),
            DirRow::Dir(d) => {
                let name = d
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_default();
                let count = Browser::count_label(app.browser.count(d));
                let name_w = w.saturating_sub(count.chars().count() + 3);
                let name = truncate(&name, name_w);
                let pad = name_w.saturating_sub(name.chars().count());
                Line::from(vec![
                    marker,
                    Span::styled(format!(" {name}{}", " ".repeat(pad)), base),
                    Span::styled(format!("{count} "), if i == sel { base } else { dim_s }),
                ])
            }
            DirRow::CreateDefault => {
                let text = format!(
                    " create {}  (recommended)",
                    config::contract_tilde(&config::default_notes_dir())
                );
                Line::from(vec![
                    marker,
                    Span::styled(
                        format!("{text:<w$}", w = w - 1),
                        base.add_modifier(Modifier::BOLD),
                    ),
                ])
            }
            DirRow::NewFolder => Line::from(vec![
                marker,
                Span::styled(
                    format!("{:<w$}", " + new folder here…", w = w - 1),
                    if i == sel { base } else { dim_s },
                ),
            ]),
        };
        lines.push(line);
        hits.overlay_rows.push(Rect::new(
            body.x,
            body.y + (i - offset) as u16,
            body.width,
            1,
        ));
    }
    f.render_widget(Paragraph::new(lines), body);
}
