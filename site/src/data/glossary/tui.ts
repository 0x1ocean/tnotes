import type { GlossaryEntry } from "../types";

export const entry: GlossaryEntry = {
  slug: "tui",
  term: "TUI (text-based user interface)",
  short:
    "A TUI is a full-screen program that draws panes, lists and editors with characters inside a terminal, taking keyboard and often mouse input.",
  body: `A TUI (text-based or terminal user interface) is an application that takes over the terminal window and paints an interface out of characters: panes, lists, status bars, dialogs. It sits between a command-line interface, which reads a line and prints a result, and a graphical interface, which draws pixels in its own window.

## Origin

The term is a retronym. Before bitmapped displays every interactive program was text-based, so nobody needed a name for it; "TUI" appeared once GUIs became the default ([Wikipedia: Text-based user interface](https://en.wikipedia.org/wiki/Text-based_user_interface)). On Unix the enabling piece was the curses library, which arrived with Berkeley Unix and gave programs like \`vi\`, \`pine\` and \`mutt\` a portable way to address any terminal type. Modern TUIs run in a terminal emulator and talk to it with ANSI escape sequences for cursor movement, colour and mouse reporting.

## CLI, TUI, GUI

A CLI is composable: its output can be piped, and it is the natural surface for scripts and AI agents. A TUI is interactive: it keeps state on screen, responds to single keys, and can show two things at once. A GUI has real fonts, images and drag-and-drop, and a mouse that works everywhere. The trade-off is where the program can run. A TUI works over SSH, inside tmux, on a server with no display, and in the same window as the shell you were already using.

## How it works

The program puts the terminal into raw mode so that keys arrive one at a time instead of one line at a time, enables the alternate screen so the shell's scrollback is preserved, and redraws changed cells each frame. Libraries handle the diffing: ncurses in C, Textual in Python, Bubble Tea in Go, ratatui in Rust ([ratatui.rs](https://ratatui.rs/)).

## Pitfalls

Key combinations are not uniform. Some terminals cannot distinguish \`ctrl+i\` from \`tab\`, and \`alt\` chords depend on the emulator's settings. Mouse support means the terminal's own text selection is taken over, so a modifier (usually shift) is needed to select text the old way. Narrow windows break multi-pane layouts unless the program has a single-column fallback. And a TUI cannot render an image or a proportional font, so Markdown is highlighted rather than typeset.`,
  inTnotes: `tnotes is a TUI built on ratatui with edtui as the text editor. It has a sidebar with a folder tree and note list, tabs, a status bar, and overlays for backlinks, folder moves and settings. The mouse works throughout: click to select, double-click to pin a tab, right-click for context menus, wheel to scroll, drag to select text (copied on release). \`shift+drag\` hands selection back to the terminal.

For small terminals there is a narrow single-panel layout; \`ctrl+b\` toggles the sidebar. Keys are emacs or vim style (\`editor.keys\` in the config).

The headless CLI is the non-interactive counterpart, and both run on the same files at the same time:

\`\`\`sh
tnotes                       # the TUI
tnotes search "standup"      # the CLI, in another shell
\`\`\`

See [/docs/keys](/docs/keys) and [/guides/terminal-note-taking](/guides/terminal-note-taking).`,
  related: ["markdown", "plain-text-notes", "vault"],
};
