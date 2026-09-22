---
title: "Terminal note-taking in 2026: four approaches and how to pick one"
description: "Editor plus grep, CLI tools like nb and jrnl, TUI apps, or editor plugins like vimwiki and org-mode: tradeoffs, a decision table, and a minimal setup for each."
published: 2026-09-22
keywords: ["terminal notes", "command line note taking", "markdown notes cli", "tui notes", "vimwiki"]
order: 1
---

"Notes in the terminal" covers four different designs that happen to share a window. They differ in who owns the editing, whether links and tags are first-class, and how much of the tool you have to learn before it pays off. This guide lays out the four, states what each is bad at, and gives a minimal working setup for each so you can try them in an afternoon.

The one thing they have in common: plain files in a directory you control. If a tool needs a database you cannot `cat`, it is not in this guide.

## Approach 1: an editor and grep

The baseline is a folder of `.md` files, your editor, and a search tool. Nothing to install beyond what a developer machine already has.

What it does well: zero lock-in, works over SSH, composes with every other Unix tool, survives any future change of editor. [ripgrep](https://github.com/BurntSushi/ripgrep) over a few thousand small files returns in milliseconds, and `fzf` turns a file list into an interactive picker.

What it does not do: links between notes are just text. Nothing tells you which notes point at the one you are reading, renaming a file silently breaks every reference to it, and tags are whatever regex you decide they are. You will end up writing shell functions to paper over this, and those functions become the tool.

Minimal setup:

```sh
mkdir -p ~/notes
n() { ${EDITOR:-vi} ~/notes/"$1".md; }
nf() { rg -l --glob '*.md' "$1" ~/notes | fzf --preview 'bat --style=plain {}' | xargs -r ${EDITOR:-vi}; }
```

That is a complete system. Most people outgrow it the first time they want to know what links to a note.

## Approach 2: CLI note tools

These add structure on top of the folder while still delegating editing to `$EDITOR`. You interact through subcommands; the editor opens when you create or edit.

**nb** is a single Bash script. Its README lists plain-text storage, `[[wiki-style linking]]`, tagging, search, Git-backed versioning and syncing, encryption, Pandoc conversion, bookmarks with cached page content, and a local web browser view ([xwmx/nb](https://github.com/xwmx/nb)). It requires Bash, Git, and an editor with command-line support. Notes can be addressed by id or title, and links can cross notebooks (`[[demo:Example Title]]`). Git runs in the background to record every change and sync notebooks with a remote. The cost is that nb is opinionated about layout (notebooks, ids) and is a large script; it does a lot, and you carry all of it.

**jrnl** is a journal, not a wiki. Entries are timestamped, typed from the command line in natural language (`jrnl today at 3am: I just met Steve Buscemi in a bar!` is the documented example), and stored in plain-text files that can be encrypted with AES ([jrnl.sh](https://jrnl.sh/en/stable/usage/)). There are no links between entries and no folders; filtering is by date and tag. If what you want is an append-only log with a good query syntax, it is hard to beat. If you want a graph of notes, it is the wrong shape.

**zk** is a Go binary aimed at Zettelkasten-style notebooks: notes created from templates, filtering by tags, links and mentions, an `fzf`-powered interactive browser, and an LSP server so Neovim, Emacs and VS Code get completion and navigation ([zk-org/zk](https://github.com/zk-org/zk)). Its README is explicit that it is not an editor. It supports Markdown links and `[[wikilinks]]`, `#hashtags`, and YAML frontmatter.

Common tradeoff: two tools with two mental models. The editor knows nothing about the notes; the CLI knows nothing about the cursor. zk narrows the gap with LSP; nb narrows it with a web view. Neither closes it.

Minimal setup for zk:

```sh
brew install zk            # or pacman -S zk, apk add zk
mkdir ~/notes && cd ~/notes && zk init
zk new --title "First note"
zk list --interactive
```

See [tnotes vs nb](/compare/tnotes-vs-nb) and [tnotes vs jrnl](/compare/tnotes-vs-jrnl) for direct comparisons.

## Approach 3: TUI applications

A [TUI](/glossary/tui) is a full-screen program drawn in the terminal: sidebar, editor pane, status line, usually mouse support. It owns editing, so links, tags, completion and navigation can all be one keystroke away without leaving the buffer.

The tradeoff is the mirror image of approach 2: you get one integrated model, but the editor inside the app is not your editor. The good ones ship vim and emacs keymaps; none of them ship your plugins. If you live in a heavily customised Neovim, a TUI will feel like a step down for editing and a step up for navigating.

tnotes is in this category and is covered in its own section below.

## Approach 4: editor plugins

If the editor is where you spend the day, the plugin route makes notes a mode of that editor rather than a separate program.

**vimwiki** turns Vim into a personal wiki. `<Leader>ww` opens the index, Enter on a word turns it into a `[[link]]` and follows it, Backspace goes back, and there is a diary with per-day files ([vimwiki docs](https://raw.githubusercontent.com/vimwiki/vimwiki/dev/doc/vimwiki.txt)). Links resolve to files relative to the wiki root, with `[[/index]]` for root-absolute paths, `[[projects/Important Project 1]]` for subdirectories, and `wiki1:` prefixes for cross-wiki links. `:VimwikiRenameFile` renames a page and updates links to it. The default extension is `.wiki`; Markdown is a per-wiki option (`'syntax': 'markdown', 'ext': '.md'`). See [tnotes vs vimwiki](/compare/tnotes-vs-vimwiki).

**Org mode** is the deepest of these: an outline format with TODO states, scheduling, an agenda view, tables that compute, and literate programming through source blocks ([orgmode.org](https://orgmode.org/)). It is Emacs-native; the org site lists Vim, VS Code, Android and iOS implementations with "basic functionality", and states the full feature set is exclusive to Emacs. The format is Org, not Markdown, which matters if other tools need to read the files. See [tnotes vs org-mode](/compare/tnotes-vs-org-mode).

**Neorg** is a Neovim plugin built on its own `.norg` format, requires Neovim 0.10+, installs through luarocks, and the README warns to pin the version because breaking workflow changes happen ([nvim-neorg/neorg](https://github.com/nvim-neorg/neorg)). It exports to Markdown but does not store notes as Markdown. See [tnotes vs neorg](/compare/tnotes-vs-neorg).

Common tradeoff: maximum editing comfort, minimum portability. The notes are only fully usable from the editor that wrote them, and two of the three use a non-Markdown format. Scripts and agents that want to read the notes need a parser for that format.

Minimal setup for vimwiki with Markdown files:

```sh
# in ~/.vimrc, after installing the plugin
let g:vimwiki_list = [{'path': '~/notes/', 'syntax': 'markdown', 'ext': '.md'}]
```

Then `vim`, `\ww`, and start typing.

## Decision table

| Need | Editor + grep | CLI tool (nb, zk, jrnl) | TUI app | Editor plugin |
|---|---|---|---|---|
| Keep your own editor and plugins | yes | yes | no (vim/emacs keys at best) | yes |
| Links with backlinks, without scripting | no | nb (plugin), zk | usually | vimwiki (`:VimwikiBacklinks`); Org and Neorg depend on extra packages |
| Rename a note without breaking links | no | tool-dependent | usually | vimwiki (`:VimwikiRenameFile`); others plugin-dependent |
| Files stay Markdown | yes | yes | yes | vimwiki (optional), org and neorg no |
| Scriptable from a shell or an agent | grep only | yes, that is the interface | if it has a CLI | via the editor's CLI, awkward |
| Works over plain SSH | yes | yes | yes | yes |
| Mouse | editor-dependent | no | usually | editor-dependent |
| Learning cost | none | subcommands | one app | the editor plus the plugin |

The row that decides most cases is the first one. If giving up your editor for note editing is unacceptable, you are choosing between approaches 2 and 4. If you are willing to use a second editor for notes, approach 3 gets you the most integrated experience for the least configuration.

## Where tnotes fits

tnotes is a TUI (approach 3) with a headless CLI bolted on (approach 2), which is why it sits between the columns above. According to its README it stores one `.md` file per note, treats folders as directories, parses `#tags` from the text, resolves `[[Note title]]` links by title and then by file stem, shows backlinks, and rewrites links when a note's title changes. Editing keys are emacs or vim. It is mouse-first, which is unusual for the category.

The CLI (`tnotes ls | search | cat | new | append | write | trash | restore`, all with `--json`) is the same binary and works on the same files, and a running TUI picks up CLI changes live. That makes it usable from scripts and AI agents without leaving the TUI open or closed; see [/for/ai-agents](/for/ai-agents).

What it does not do: it is not your editor. There is no plugin system, no LSP, no agenda, no Org-style scheduling, no encryption of its own, no mobile app. Sync and encryption are delegated to the file system. If any of those are hard requirements, the comparison pages on [/compare](/compare) say which alternative covers them. Install is `brew install 0x1ocean/tnotes/tnotes` or `cargo install tnotes`; details in [Getting started](/docs/getting-started).

## Deciding in one sitting

Try them in this order, ten minutes each:

1. The shell functions from approach 1. If after a week you have not wanted backlinks, stop here.
2. If you want links but not another editor: zk (Markdown, LSP) or nb (Git history, bookmarks). See [wikilinks in the terminal](/guides/markdown-wikilinks-terminal) for how each resolves `[[links]]`.
3. If you are in Vim or Emacs all day anyway: vimwiki or Org. Accept that the notes are tied to the editor.
4. If you want links, tags, mouse, and a CLI in one binary and can live with emacs or vim keys instead of your full config: a TUI, tnotes among them.

Whatever you pick, keep the folder plain and syncable; [Syncing plain-text notes](/guides/plain-text-notes-sync) covers the rest.
