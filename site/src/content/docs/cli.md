---
title: CLI reference
description: The headless tnotes commands — ls, search, cat, new, append, write, trash, restore — with --json output, filters, stdin input and note ids.
order: 4
---

Every subcommand works on the same files as the TUI. `--dir` and `--json` apply to all of them, and a running TUI picks the changes up live.

```sh
tnotes ls [--tag work] [--folder notes/sub] [--limit 20]   # id · title · tags, newest first
tnotes search "milk" [--tag …] [--folder …] [--limit …]    # fuzzy over titles and bodies, best first
tnotes cat weekly-plan                                     # print the note (--json adds text)
tnotes new "Weekly plan" --tag work [--folder …]           # prints the new id; refuses an existing title unless --duplicate
echo "body" | tnotes new "Piped" --stdin                   # body below the title (or whole note without a title)
tnotes append weekly-plan "- [ ] milk"                     # or --stdin
cat new.md | tnotes write weekly-plan --stdin              # replace the text; a new title renames the file and updates [[links]]
tnotes trash weekly-plan                                   # move to the root's .Trash
tnotes restore weekly-plan                                 # and back
tnotes ls --json                                           # id, path, title, folder, tags, links, backlinks, created, modified, preview
```

## Addressing a note

A note is addressed by its **id** (`root/sub/stem`, as printed by `ls`), by a **path**, or by a unique file **stem**. The id is stable across `ls`, `search`, `cat`, `append`, `write`, `trash` and `restore`.

## Commands

### `ls`

Lists notes, newest first. `--tag` (with or without the leading `#`), `--folder` and `--limit` narrow the result.

### `search <query>`

Fuzzy search over titles and bodies, best match first. Same filters as `ls`.

### `cat <note>`

Prints the note text. With `--json` the record includes the `text` field.

### `new <title>`

Creates a note and prints its id. `--tag` adds tags on the line below the title, `--folder` places it. A title that already exists is refused unless `--duplicate` is given; a blank title is rejected. With `--stdin`, the piped text becomes the body below the title — or the whole note when no title is given.

### `append <note> [text]`

Appends a line (or `--stdin`) to the end of the note. Text starting with `-` is fine: `tnotes append plan "- [ ] milk"`.

### `write <note> --stdin`

Replaces the whole text. A changed first line renames the file and rewrites `[[links]]` in other notes. Empty input is refused. If the note has unsaved edits in a running TUI, the TUI keeps its text and saves a conflict copy; the CLI reports an error.

### `trash <note>` · `restore <note>`

Move a note to the root's `.Trash/` (mirroring the folder layout) and back.

## JSON output

`--json` on `ls` and `search` returns an array of records:

```json
{
  "id": "vault/weekly-plan",
  "path": "/home/me/notes/vault/weekly-plan.md",
  "title": "Weekly plan",
  "folder": "vault",
  "tags": ["work", "planning"],
  "links": ["tnotes roadmap", "Meeting notes"],
  "backlinks": ["tnotes roadmap"],
  "created": "2026-09-21T09:12:04Z",
  "modified": "2026-09-22T08:40:11Z",
  "preview": "#work #planning - [x] Review pull requests …"
}
```

`links` are the `[[titles]]` this note references; `backlinks` are the notes that reference it. Backlink resolution is linear in the number of notes (about 40 ms for 500 notes).

## Concurrency with the TUI

The CLI and the TUI can work on the same vault at once. A note that is open but clean in the TUI follows a CLI `write`/`append` live; a note with unsaved edits keeps them and the TUI saves a conflict copy next to it. Nothing is ever silently overwritten.
