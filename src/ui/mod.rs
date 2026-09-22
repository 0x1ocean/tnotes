use chrono::{DateTime, Datelike, Local};
use edtui::{EditorMode, EditorState, EditorTheme, EditorView};
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout as RLayout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph, Widget, Wrap};

use crate::app::{
    App, Browser, ConfirmAction, DirRow, Layout, Menu, Overlay, Pane, PromptKind, SortMode,
    StatusKind, TreeRow,
};
use crate::config::{self, Dates, EditorKeys, TabNumbers};
use crate::index::Filter;
use crate::note::Note;

/// Seven-token palette, set once at startup from `config.toml` `[theme]`.
pub mod theme {
    use std::sync::OnceLock;

    use ratatui::style::Color;

    #[derive(Debug, Clone, Copy)]
    pub struct Palette {
        pub accent: Color,
        pub dim: Color,
        pub warn: Color,
        pub err: Color,
        pub ok: Color,
        pub code: Color,
        pub tag: Color,
    }

    impl Default for Palette {
        fn default() -> Self {
            Palette {
                accent: Color::Cyan,
                dim: Color::DarkGray,
                warn: Color::Yellow,
                err: Color::Red,
                ok: Color::Green,
                code: Color::Yellow,
                tag: Color::Green,
            }
        }
    }

    static PALETTE: OnceLock<Palette> = OnceLock::new();

    pub fn init(p: Palette) {
        let _ = PALETTE.set(p);
    }

    fn get() -> &'static Palette {
        PALETTE.get_or_init(Palette::default)
    }

    pub fn accent() -> Color {
        get().accent
    }
    pub fn dim() -> Color {
        get().dim
    }
    pub fn warn() -> Color {
        get().warn
    }
    pub fn err() -> Color {
        get().err
    }
    pub fn ok() -> Color {
        get().ok
    }
    pub fn code() -> Color {
        get().code
    }
    pub fn tag() -> Color {
        get().tag
    }
}
use theme::*;

mod editor;
mod navigator;
mod overlays;
mod settings;
mod status;
mod tabs;

use editor::{draw_editor, draw_popup};
use navigator::{centered_text, draw_navigator};
use overlays::{draw_backlinks, draw_browser, draw_confirm, draw_menu, draw_picker, draw_prompt};
use settings::draw_settings_page;
use status::draw_status;
use tabs::draw_tabs;

const WIDE_MIN: u16 = 80;

const TREE_INDENT: u16 = 4;

/// Rects rendered by the last `draw`, used for mouse hit-testing.
#[derive(Debug, Default, Clone)]
pub struct Hits {
    /// `(tab index, rect)` for every tab visible in the bar.
    pub tabs: Vec<(usize, Rect)>,
    pub tab_close: Rect,
    pub tab_new: Rect,
    /// Wide layout: the separator line and the blank column left of it.
    pub separator: Rect,
    pub tree: Rect,
    pub tree_rows: Vec<Rect>,
    pub search: Rect,
    pub list: Rect,
    pub list_rows: Vec<Rect>,
    pub editor: Rect,
    pub popup_rows: Vec<Rect>,
    pub sidebar_toggle: Rect,
    pub settings_sections: Vec<Rect>,
    /// `(absolute row index, rect)`: rows scroll, so the index is stored explicitly.
    pub settings_rows: Vec<(usize, Rect)>,
    /// Settings page: `‹ back` header button and `esc back` in the footer.
    pub back: Rect,
    pub settings_back: Rect,
    /// `(row, control index, rect)` for option buttons, stepper arrows and `×`.
    pub settings_controls: Vec<(usize, usize, Rect)>,
    pub status_keys: Rect,
    pub status_help: Rect,
    pub status_sort: Rect,
    pub status_filter: Rect,
    /// `↩ N` backlinks count of the active note.
    pub status_backlinks: Rect,
    pub settings_link: Rect,
    /// Narrow layout: fold header above the tree.
    pub tree_header: Rect,
    pub overlay: Rect,
    /// `(absolute row index, rect)`: overlay lists scroll, so the index is stored explicitly.
    pub overlay_rows: Vec<(usize, Rect)>,
    /// Folder browser breadcrumb segments.
    pub crumbs: Vec<Rect>,
}

impl Hits {
    /// Absolute index of the overlay row under `pos`.
    pub fn overlay_row(&self, pos: ratatui::layout::Position) -> Option<usize> {
        self.overlay_rows
            .iter()
            .find(|(_, r)| r.contains(pos))
            .map(|(i, _)| *i)
    }
}

/// Screen column of the fold glyph for a tree row at `depth` inside the tree rect.
pub fn tree_glyph_x(tree: Rect, depth: usize) -> u16 {
    tree.x + 1 + depth as u16 * TREE_INDENT
}

fn truncate(s: &str, width: usize) -> String {
    let n = s.chars().count();
    if n <= width {
        return s.to_string();
    }
    let mut out: String = s.chars().take(width.saturating_sub(1)).collect();
    out.push('…');
    out
}

fn overlay_block(title: String, bottom: String) -> Block<'static> {
    Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(accent()))
        .title(Line::from(Span::styled(title, Style::new().fg(dim()))))
        .title_bottom(Line::from(Span::styled(bottom, Style::new().fg(dim()))).centered())
}

fn centered(area: Rect, w: u16, h: u16) -> Rect {
    let w = w.min(area.width);
    let h = h.min(area.height);
    Rect::new(
        area.x + (area.width - w) / 2,
        area.y + (area.height - h) / 2,
        w,
        h,
    )
}

fn field_theme() -> EditorTheme<'static> {
    EditorTheme::default()
        .base(Style::reset())
        .cursor_style(Style::new().add_modifier(Modifier::REVERSED))
        .selection_style(Style::new().bg(dim()))
        .hide_status_line()
}

fn render_field(f: &mut Frame, state: &mut EditorState, area: Rect) {
    EditorView::new(state)
        .wrap(false)
        .theme(field_theme())
        .render(area, f.buffer_mut());
}

pub fn draw(f: &mut Frame, app: &mut App) -> Hits {
    let area = f.area();
    let layout = if area.width >= WIDE_MIN {
        Layout::Wide
    } else {
        Layout::Narrow
    };
    app.layout = layout;
    let [top, status] = RLayout::vertical([Constraint::Min(1), Constraint::Length(1)]).areas(area);
    let mut hits = Hits::default();

    // Sidebar toggle (or "back" on the settings page) at the bottom-left of the status bar.
    let toggle = Rect::new(status.x, status.y, 4.min(status.width), 1);
    hits.sidebar_toggle = toggle;

    if app.settings.is_some() {
        draw_settings_page(f, app, top, status, &mut hits);
        draw_overlays(f, app, area, &mut hits);
        return hits;
    }

    let nav_focused = matches!(app.focus, Pane::Tree | Pane::List);
    let show_nav = app.sidebar && (layout == Layout::Wide || nav_focused);
    let show_editor = layout == Layout::Wide || !show_nav;

    let mut editor_col = top;
    if show_nav {
        let nav_w = if show_editor {
            app.cfg.appearance.sidebar_width.min(top.width)
        } else {
            top.width
        };
        let nav = if show_editor {
            // One blank column before the separator so text never touches it.
            Rect::new(top.x, top.y + 1, nav_w - 1, top.height - 1)
        } else {
            top
        };
        draw_navigator(f, app, nav, &mut hits);
        if show_editor {
            let sep_x = top.x + nav_w;
            hits.separator = Rect::new(sep_x - 1, top.y, 2, top.height);
            draw_vline(f, sep_x, top.y, top.height, app.sidebar_drag.is_some());
            editor_col = Rect::new(sep_x + 1, top.y, top.width - nav_w - 1, top.height);
        }
    }
    if show_editor {
        let [tabbar, ed] =
            RLayout::vertical([Constraint::Length(1), Constraint::Min(1)]).areas(editor_col);
        draw_tabs(f, app, tabbar, &mut hits);
        draw_editor(f, app, ed, &mut hits);
    }
    draw_status(f, app, status, &mut hits);
    if app.overlay == Overlay::None && app.focus == Pane::Editor {
        draw_popup(f, app, area, &mut hits);
    }

    draw_overlays(f, app, area, &mut hits);
    hits
}

fn draw_vline(f: &mut Frame, x: u16, y: u16, height: u16, active: bool) {
    let style = Style::new().fg(if active { accent() } else { dim() });
    for row in y..y + height {
        f.render_widget(Span::styled("│", style), Rect::new(x, row, 1, 1));
    }
}

fn draw_overlays(f: &mut Frame, app: &mut App, area: Rect, hits: &mut Hits) {
    match app.overlay.clone() {
        Overlay::None => {}
        Overlay::Confirm(a) => draw_confirm(f, app, area, &a, hits),
        Overlay::Prompt(k) => draw_prompt(f, app, area, &k, hits),
        Overlay::Picker(p) => draw_picker(f, app, area, &p, hits),
        Overlay::Backlinks(p) => draw_backlinks(f, app, area, &p, hits),
        Overlay::Menu(m) => draw_menu(f, area, &m, hits),
        Overlay::Browse => draw_browser(f, app, area, hits),
    }
}
