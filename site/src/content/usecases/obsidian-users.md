---
title: tnotes for Obsidian users
description: "Open your Obsidian vault in the terminal with tnotes: same folders, [[wikilinks]] and #tags, no plugins, canvas or properties. Run both on one directory."
persona: Obsidian users
order: 3
keywords:
  - obsidian terminal
  - obsidian vault cli
  - open obsidian notes in terminal
  - obsidian alternative terminal
---

## The problem

You are in a terminal most of the day and your notes are in an Electron window. Alt-tabbing to Obsidian to check a runbook or jot a line is a small cost, paid often. Over ssh it is not possible at all. What you want is a way to read and edit the same vault from the shell, without converting anything and without losing Obsidian for the days you want the graph view.

An Obsidian vault is a directory of Markdown files, and that is exactly what tnotes opens. Nothing is imported or exported; both programs read and write the same `.md` files.

## Setup in three steps

1. Install tnotes ([getting started](/docs/getting-started)).
2. Add the vault as a root. `tnotes ~/Obsidian/Work` saves it to the config and opens it; or write the config directly:

```toml
roots = ["~/Obsidian/Work", "~/Obsidian/Personal"]

[editor]
template = "# "
```

3. Leave `.obsidian/` where it is. Dot-prefixed files and directories are ignored, so Obsidian's configuration folder, its workspace state and plugin data never appear in the tnotes list.

Set `template = "# "` (the default) so new notes begin with a heading. tnotes names the file after the first line, so `# Postgres failover` becomes `postgres-failover.md`, which Obsidian then shows under that filename.

## What carries over

- **Folders.** Obsidian folders are directories on disk and the tnotes tree shows the same hierarchy. `m` in the list moves a note between them.
- **`[[Wikilinks]]`.** Obsidian's default link format is `[[Note title]]`. tnotes resolves the same syntax by title (case-insensitive), then by file stem, completes it after `[[` with `tab`, follows it with `alt+enter` or `ctrl+click`, and creates a missing target on follow, as Obsidian does. When you rename a note in tnotes, `[[old title]]` is rewritten to `[[new title]]` in every other note, matching Obsidian's automatic link update.
- **`#tags`.** Inline tags are parsed from the text, including nested ones with a slash such as `#inbox/to-read`, the same form Obsidian uses. `ctrl+click` on a tag filters the list to it.
- **Backlinks.** `alt+enter` on a line without a link opens a backlinks overlay; the status bar shows the count as `↩ N`.
- **Checkboxes.** `- [ ]` items toggle with `alt+x` or a click.

## Daily workflow

Keep Obsidian open on the desktop and tnotes in a tmux pane. Both watch the directory. A note you edit in Obsidian reloads in a clean tnotes tab, cursor position kept; a note you edit in tnotes is written atomically after `autosave_ms` of idle time, so Obsidian never sees a half-written file. If both sides have unsaved changes to the same note, tnotes keeps its own text as `name (conflict <time>).md` rather than overwriting Obsidian's save. Conflict files are listed as normal notes, so you see them.

For quick capture from a shell, skip the TUI:

```sh
tnotes new "Standup 2026-09-22" --tag work --folder Meetings
tnotes append standup-2026-09-22 "- [ ] ask Ann about the migration window"
```

The note appears in Obsidian's file explorer as soon as it is written.

## What does not carry over

Be clear about this before moving a heavy vault:

- **Plugins.** tnotes has none, community or core. Dataview queries, Templater templates and everything else a plugin renders stay in Obsidian. Their source files remain in the vault; tnotes lists the `.md` ones as plain text.
- **Canvas.** `.canvas` files are JSON, not notes. tnotes shows one `.md` file per note and nothing else.
- **Properties.** Obsidian stores properties as a YAML block at the top of the file, including `tags:` and `aliases:`. tnotes has no properties view; its tags are `#hashtags` parsed from the text. It also takes the first line of the file as the title and names the file after it, which matters for notes whose first line is `---`.
- **Link forms beyond `[[Title]]`.** Heading links (`[[Note#Heading]]`), block references (`[[Note#^id]]`), display text (`[[Note|text]]`) and embeds (`![[Note]]`) are Obsidian syntax with no documented tnotes counterpart.
- **Graph view, Obsidian Sync, Publish, mobile apps.** None of these exist in tnotes. The vault stays syncable by Obsidian Sync, Syncthing or git because tnotes writes nothing beside your notes.

## Read next

- [tnotes vs Obsidian](/compare/tnotes-vs-obsidian) for a side-by-side table.
- [Sync and encryption](/docs/sync) on running the same folder through Syncthing, iCloud or git.
- [Wikilink](/glossary/wikilink) and [vault](/glossary/vault) in the glossary.
