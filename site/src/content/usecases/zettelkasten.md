---
title: tnotes for Zettelkasten
description: "A terminal Zettelkasten with title-based [[links]], a backlinks overlay, nested #tags and CLI capture, plus an honest note on what it lacks: IDs, graph view."
persona: Zettelkasten
order: 6
keywords:
  - zettelkasten terminal
  - zettelkasten markdown cli
  - plain text zettelkasten
  - backlinks terminal notes
---

## The problem

A Zettelkasten lives or dies on friction. If writing a new note and linking it to two existing ones takes more than a few seconds, the notes stay in your head. If the tool hides the files behind a database, you cannot trust the box to outlive the app. Most terminal-friendly setups solve one of these: a fast editor with no link awareness, or a link-aware app that is not fast in a terminal.

tnotes handles the linking layer over plain `.md` files: `[[Title]]` links resolve by title, a completion popup appears after `[[`, and a backlinks overlay shows what points at the current note. The rest is deliberately absent, and this page says which parts.

## Setup in three steps

1. Choose a flat or shallow layout. Folders are directories on disk and the tree shows them, but a Zettelkasten usually wants structure to come from links and tags rather than hierarchy. Two folders, `inbox` and `zettel`, are enough:

```toml
roots = ["~/zettel"]

[editor]
template = "# "
autosave_ms = 300

[appearance]
compact = true
counts = true
```

2. Keep the title on the first line. tnotes names the file after it (`# Spaced repetition decays with interval` becomes `spaced-repetition-decays-with-interval.md`) and rewrites `[[old title]]` to `[[new title]]` in every other note when it changes. This is what makes title-based links safe to refactor.

3. Decide on tags. `#tags` are parsed from the body, and nested tags such as `#topic/memory` work, so a tag hierarchy can replace folders. `ctrl+click` on a tag filters the list to it; `tnotes ls --tag topic/memory` does the same from a shell.

## Daily workflow

**Capture.** `ctrl+t` opens a new note with the template's `# ` in place; type the title, `enter`, write the idea. From a shell, the same is one line:

```sh
echo "Interval growth should be multiplicative, not additive. See [[Spaced repetition decays with interval]]." \
  | tnotes new "Ease factor" --tag topic/memory --folder inbox --stdin
```

`new` refuses a title that already exists unless you pass `--duplicate`. In a Zettelkasten that is a feature: two notes with the same title mean either a duplicate idea or a title that is not specific enough.

**Link while writing.** Type `[[`, a few letters, and `tab` to complete against every title in the vault. If the note you want does not exist yet, write the link anyway and press `alt+enter` on it: the target is created next to the current note and opened. Missing targets are how structure notes grow.

**Follow backlinks.** With the cursor on a line that has no link, `alt+enter` opens the backlinks overlay; the status bar shows `↩ N` for the count. This is the "what else connects here" question, answered without leaving the editor.

**Process the inbox.** In the list, `/` filters, `m` moves the selected note from `inbox` to `zettel`, `s` changes the sort. Since the note's id changes with its folder but the title does not, links keep working.

**Audit from the shell.** `ls --json` includes `links` and `backlinks` for every note, so orphan detection is a `jq` filter:

```sh
tnotes ls --json | jq -r '.[] | select(.backlinks == [] and .links == []) | .title'
```

Run it weekly. Orphans are either entry points that need a link from a structure note or ideas that were never connected.

## What tnotes does not do

- **No unique IDs.** There is no Luhmann-style `1a2b` numbering and no automatic `202609221430` timestamp prefix. Links resolve by title, then by file stem. If you want IDs, put them in the title yourself (`# 202609221430 Ease factor`); the filename will follow.
- **No graph view.** The `links` and `backlinks` arrays in `--json` are the graph. Render them with Graphviz or open the same folder in Obsidian when you want to see it.
- **No transclusion or block references.** A link points at a whole note.
- **No aliases.** A note has one title, and a link must use that title or the file stem. If two phrasings should reach the same note, write the second one as a one-line note that links to the first.

## Pair it with

- A **timestamp alias** in your shell if you want IDs anyway: `zk() { tnotes new "$(date +%Y%m%d%H%M) $*" --folder inbox; }`.
- **git** on the vault for history. tnotes writes only `.md` files and ignores `.git/`, so the repo stays clean.
- **Obsidian** on the same directory for the graph on the days you want one; see [tnotes for Obsidian users](/for/obsidian-users).

## Read next

- [Zettelkasten](/glossary/zettelkasten) and [backlink](/glossary/backlink) in the glossary.
- [Markdown wikilinks in the terminal](/guides/markdown-wikilinks-terminal) for how resolution and renaming work.
- [tnotes vs Zettlr](/compare/tnotes-vs-zettlr) if you are comparing with a desktop Zettelkasten editor.
