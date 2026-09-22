---
title: Configuration
description: Every key in tnotes' config.toml with its default — roots, appearance, editor (emacs/vim, autosave, wrap, width) and theme colours.
order: 3
---

`~/.config/tnotes/config.toml` (macOS: `~/Library/Application Support/tnotes/config.toml`). The in-app settings page (`F2` or `,`) writes this file for you, so you rarely need to edit it by hand.

Every key with its default (`notes_dir = "…"` from 0.x is still read when `roots` is absent):

```toml
roots = ["~/Documents/notes"]

[appearance]
compact = false          # one-line list items instead of title + preview
dates = "relative"       # relative | absolute
counts = true            # note counts on the right of tree rows
sidebar_width = 32       # 24..=48
tab_numbers = "always"   # always | multi (only when 2+ tabs)

[editor]
keys = "emacs"           # emacs | vim
autosave_ms = 500        # idle time before a dirty tab is written; 200..=3000
template = "# "          # initial text of a new note; cursor lands at the end of the first line
wrap = true
highlight = "color"      # color (headings/links accent, code, tags) | mono (bold/italic/underline only)
width = 72               # max text column width in cells, 40..=160; 0 = full editor width
align = "left"           # left | center
cursor = "drawn"         # drawn (painted by the editor) | bar | underline | block (terminal cursor)
blink = false

[theme]                  # colour names (cyan, darkgray, #rrggbb, ...); unset keys keep the defaults
accent = "cyan"          # headings, links, open checkboxes, selection
dim = "darkgray"         # markers, quotes, secondary text
warn = "yellow"
err = "red"
ok = "green"
code = "yellow"          # inline and fenced code (highlight = "color")
tag = "green"            # #tags (highlight = "color")
```

## Roots

`roots` is a list of folders. Each shows up as a top-level entry in the tree with its own `.Trash/`. `tnotes ~/path` appends to the list; `tnotes --dir ~/path` uses a folder for one session without touching the config.

## Theme

The seven colours default to named terminal colours, so tnotes follows whatever palette your terminal uses. Set `#rrggbb` values to pin them. `highlight = "mono"` drops colour from Markdown entirely and uses only bold, italic and underline.

## Session state

Open tabs, filter, sort, focus and folded tree sections are stored in `~/.local/state/tnotes/state.toml` and restored on the next launch. Delete the file to start clean.
