---
title: tnotes for vim users
description: "Terminal Markdown notes for vim users - j/k, gg/G, i/a/o, u and dot-repeat in a TUI, with plain files you can still open in real vim any time."
persona: vim users
order: 2
keywords:
  - vim notes app terminal
  - vim keybindings notes tui
  - markdown notes vim
  - vimwiki alternative
---

## The problem

You already have an editor. What vim does not give you is the layer around the files: a list of notes sorted by modification time, a filter by `#tag`, a completion popup after `[[`, a panel that shows what links to the note you are in. vimwiki adds a personal wiki inside vim, with its own default syntax and `.wiki` extension unless you switch it to Markdown in your vimrc; that is a good fit if you want everything inside one process, less so if other tools need to read the notes unchanged.

tnotes is a separate program with a list, a folder tree and an editor whose keys come from a vim mode, and it stores nothing but `.md` files. When the editing gets heavy, you open the same file in vim.

## Setup in three steps

1. Install (`brew install 0x1ocean/tnotes/tnotes`, `cargo install tnotes`, or a tarball; see [getting started](/docs/getting-started)) and run `tnotes` once to pick a notes directory.
2. Switch the editor to vim keys. Either press `F2` and change it on the settings page, or write it yourself:

```toml
roots = ["~/notes"]

[editor]
keys = "vim"
autosave_ms = 500
cursor = "block"
```

3. Learn one key that vim does not have: `esc` in normal mode leaves the editor and returns to the list. Everything else in the list is `j`/`k`.

## Daily workflow

The list is the home screen and it already speaks vim: `j`/`k` move, `gg`/`G` jump to top and bottom, `/` filters, `l` or `enter` opens the selected note and pins its tab, `q` quits from the list or tree. `d` trashes a note, `u` undoes the trash, `m` moves it to a folder, `s` changes the sort. In the tree, `h` goes back to the list and `l` filters the list to the selected folder.

Inside a note, the vim mode documented in the [keys page](/docs/keys) covers:

- entering insert with `i` `a` `I` `A` `o` `O`;
- `esc` back to normal, and `esc` again back to the list;
- `u` undo, `ctrl+r` redo, `.` repeat;
- `/` search in the note, `n` and `N` to step through matches.

A typical capture looks like this: `ctrl+t` for a new note, type the title on the first line (the file is named after it), `o` to open a line below, write, `esc` `esc` to drop back to the list. There is no `:w`; the note is written after `autosave_ms` of idle time, and the save is atomic.

Links and tags do not need commands. Type `[[` and `tab` to complete a title from the vault, `#` and `tab` for a tag. `alt+enter` on a link follows it and creates the target if it does not exist; `alt+enter` on a line without a link opens the backlinks overlay. Lists continue on `enter`, and `tab` / `shift+tab` nest or un-nest a list item.

When a note needs a real editor, open it in vim from another tmux pane. tnotes watches the directory: a clean tab reloads when vim writes the file, and the cursor stays where it was. If the tnotes tab had unsaved edits, they are kept and a `name (conflict <time>).md` copy is written next to the file rather than losing either side.

## Where it differs from vim

The editing keys are provided by edtui, not by vim, and tnotes documents only the keys listed above. There is no ex command line, no `.vimrc`, no text objects or registers promised by the README, no `gt` for tabs. Tabs are switched with `alt+.` / `alt+,` or `1-9`. The in-app `?` page is the source of truth for what a given build supports; if a motion is not on it, do not plan around it.

The `/` key is overloaded by scope: in the list it filters notes, in the editor it searches the current note. `ctrl+c` and `ctrl+q` quit from anywhere; in the list `q` does too.

## What to pair it with

- **vim itself** for macros, substitutions and anything that touches many lines. The files are ordinary Markdown and tnotes reloads them live.
- **tmux** so the list, vim and a shell sit side by side.
- The **CLI** for scripted capture from a shell function: `tnotes append inbox "- $*"` needs no TUI at all.

## Read next

- [Keys](/docs/keys), the full table including the emacs alternatives.
- [Configuration](/docs/config) for `cursor`, `width`, `wrap` and the highlight mode.
- [tnotes vs vimwiki](/compare/tnotes-vs-vimwiki) if you are weighing a plugin against a separate program.
