mod app;
mod config;
mod index;
mod note;
mod session;
mod store;
mod ui;
mod watch;

use std::io::stdout;
use std::path::PathBuf;

use clap::Parser;
use ratatui::crossterm::event::{
    DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste, EnableMouseCapture,
};
use ratatui::crossterm::execute;

use crate::app::App;
use crate::store::Store;

/// Minimalist Markdown notes in the terminal.
#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// Single notes folder for this session (overrides config `roots`).
    #[arg(long)]
    dir: Option<PathBuf>,
    /// Folder to add to your roots (saved to config) and open.
    folder: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut cfg = config::load(args.dir)?;
    let mut start = None;
    if let Some(folder) = args.folder {
        let root = config::resolve_root(&folder)?;
        if !cfg.roots_fixed && !cfg.roots.contains(&root) {
            cfg.roots.push(root.clone());
            config::save(&cfg)?;
        }
        start = Some(root);
    }
    ui::theme::init(cfg.theme);
    let store = Store::load(&cfg.roots)?;
    let (watchers, rx) = watch::spawn(&cfg.roots)?;

    let mut term = ratatui::init();
    execute!(stdout(), EnableMouseCapture, EnableBracketedPaste)?;
    let mut app = App::new(cfg, store, rx, watchers);
    app.start_filter = start;
    let res = app.run(&mut term);
    let _ = execute!(stdout(), DisableMouseCapture, DisableBracketedPaste);
    ratatui::restore();
    res
}
