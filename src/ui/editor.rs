//! Editor pane, markdown preview and the tag popup.

use super::*;

pub(super) fn draw_editor(f: &mut Frame, app: &mut App, area: Rect, hits: &mut Hits) {
    let focused = app.focus == Pane::Editor;
    let in_trash = app.tab_in_trash(app.active);
    if area.width < 4 || area.height < 2 {
        return;
    }
    // One blank row under the tab bar, padding on each side, then the text column
    // limited to `editor.width` and placed per `editor.align`.
    let m: u16 = if app.layout == Layout::Narrow { 1 } else { 2 };
    let full = Rect::new(area.x + m, area.y + 1, area.width - 2 * m, area.height - 1);
    let max = app.cfg.editor.width;
    let w = if max == 0 {
        full.width
    } else {
        full.width.min(max)
    };
    let x = match app.cfg.editor.align {
        config::Align::Left => full.x,
        config::Align::Center => full.x + (full.width - w) / 2,
    };
    let inner = Rect::new(x, full.y, w, full.height);
    hits.editor = inner;
    let Some(note) = app.current_note() else {
        centered_text(f, inner, vec!["no open notes · ctrl+t to create".into()]);
        return;
    };
    let text = note.text.clone();
    let preview = app.active_tab().is_some_and(|t| t.preview) || in_trash;
    let wrap = app.cfg.editor.wrap;
    let native = app.cfg.editor.cursor != config::CursorShape::Drawn;

    let tab = app.active_tab_mut().expect("checked above");
    if preview {
        let md: Text =
            tui_markdown::from_str_with_options(&text, &tui_markdown::Options::new(MdStyle));
        let paragraph = Paragraph::new(md).wrap(Wrap { trim: false });
        // Scroll range comes from the *rendered* (wrapped) height, not the source line count,
        // and stops once the last line is on screen.
        let rendered = paragraph.line_count(inner.width);
        tab.preview_max = rendered.saturating_sub(inner.height as usize) as u16;
        tab.preview_scroll = tab.preview_scroll.min(tab.preview_max);
        f.render_widget(paragraph.scroll((tab.preview_scroll, 0)), inner);
        return;
    }

    let cursor = if focused && !native {
        Style::new().add_modifier(Modifier::REVERSED)
    } else {
        Style::reset()
    };
    EditorView::new(&mut tab.editor)
        .wrap(wrap)
        .theme(
            EditorTheme::default()
                .base(Style::reset())
                .cursor_style(cursor)
                .selection_style(Style::new().bg(dim()))
                .hide_status_line(),
        )
        .render(inner, f.buffer_mut());
    if focused
        && native
        && let Some(pos) = tab.editor.cursor_screen_position()
    {
        f.set_cursor_position(pos);
    }
}

pub(super) fn draw_popup(f: &mut Frame, app: &App, area: Rect, hits: &mut Hits) {
    let Some(popup) = &app.popup else { return };
    let Some(pos) = app
        .active_tab()
        .and_then(|t| t.editor.cursor_screen_position())
    else {
        return;
    };
    let longest = popup
        .candidates
        .iter()
        .map(|c| c.chars().count())
        .max()
        .unwrap_or(0);
    let w = (longest as u16 + 3).min(area.width);
    let h = popup.candidates.len() as u16 + 2;
    let x = pos.x.min(area.right().saturating_sub(w));
    let y = if pos.y + 1 + h <= area.bottom() {
        pos.y + 1
    } else {
        pos.y.saturating_sub(h)
    };
    let rect = Rect::new(x, y, w, h.min(area.height));
    f.render_widget(Clear, rect);
    let block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(dim()));
    let inner = block.inner(rect);
    f.render_widget(block, rect);
    let lines: Vec<Line> = popup
        .candidates
        .iter()
        .enumerate()
        .map(|(i, c)| {
            if i == popup.sel {
                Line::from(vec![
                    Span::styled("▎", Style::new().fg(accent())),
                    Span::raw(c.clone()),
                ])
            } else {
                Line::from(Span::styled(format!(" {c}"), Style::new().fg(dim())))
            }
        })
        .collect();
    hits.popup_rows = (0..popup.candidates.len() as u16)
        .map(|i| Rect::new(inner.x, inner.y + i, inner.width, 1))
        .collect();
    f.render_widget(Paragraph::new(lines), inner);
}

/// Markdown preview styles restricted to the five theme tokens.
#[derive(Clone, Copy, Debug)]
struct MdStyle;

impl tui_markdown::StyleSheet for MdStyle {
    fn heading(&self, level: u8) -> Style {
        match level {
            1 => Style::new().fg(accent()).add_modifier(Modifier::BOLD),
            _ => Style::new().fg(accent()),
        }
    }
    fn heading_marker(&self, level: u8) -> &str {
        if level == 1 { "" } else { "#" }
    }
    fn code(&self) -> Style {
        Style::new().fg(dim())
    }
    fn link(&self) -> Style {
        Style::new().fg(dim()).add_modifier(Modifier::UNDERLINED)
    }
    fn blockquote(&self) -> Style {
        Style::new().fg(dim())
    }
    fn metadata_block(&self) -> Style {
        Style::new().fg(dim())
    }
    fn math_inline(&self) -> Style {
        Style::new()
    }
    fn math_display(&self) -> Style {
        Style::new()
    }
    fn table_header(&self) -> Style {
        Style::new().add_modifier(Modifier::BOLD)
    }
    fn table_border(&self) -> Style {
        Style::new().fg(dim())
    }
    fn alert(&self, _kind: tui_markdown::AlertKind) -> Style {
        Style::new().fg(warn())
    }
}
