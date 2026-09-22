---
title: tnotes for AI agents
description: "Give Claude Code, Codex or any AI agent a notes vault it can read and write through a JSON CLI, with conflict copies instead of silent overwrites."
persona: AI agents
order: 1
keywords:
  - notes for ai agents
  - claude code memory notes
  - markdown notes json cli
  - agents.md notes vault
---

## The problem

Coding agents start every session blank. CLAUDE.md and AGENTS.md carry instructions, but not the running state of a project: what was decided last week, which host has the flaky disk, what the customer said on the call. That state lives in your notes, and most notes apps expose it through a GUI, a sync service or a plugin API that an agent cannot call from a shell.

tnotes exposes the vault as a CLI. Every subcommand takes `--json`, works on the same `.md` files the TUI shows, and needs no daemon. An agent that can run a shell command can use it.

## Setup in three steps

1. Install tnotes on the machine where the agent runs (`brew install 0x1ocean/tnotes/tnotes` or `cargo install tnotes`; see [getting started](/docs/getting-started)).
2. Point it at a vault. Either set `roots` in `~/.config/tnotes/config.toml` or pass `--dir` on every call so the agent never touches your config:

```toml
roots = ["~/Documents/notes"]
```

3. Tell the agent the vault exists. A short block in `AGENTS.md` or `CLAUDE.md` is enough:

```md
## Notes

Project memory lives in a tnotes vault. Use the CLI, never edit the files directly.

- List:    tnotes ls --json --dir ~/Documents/notes
- Search:  tnotes search "<query>" --json --dir ~/Documents/notes | jq '.[0]'
- Read:    tnotes cat <id> --json --dir ~/Documents/notes
- Create:  echo "<body>" | tnotes new "<Title>" --tag agent --stdin --dir ~/Documents/notes
- Append:  tnotes append <id> "- <line>" --dir ~/Documents/notes

Before creating a note, search for its title. `new` refuses a title that already exists.
Prefer `append` over `write`. Never delete; use `tnotes trash`.
```

## Daily workflow

Reading is one call. `tnotes ls --json` returns every note with `id`, `path`, `title`, `folder`, `tags`, `links`, `backlinks`, `created`, `modified` and a `preview`. The backlink resolution is linear, so a 500-note vault answers in tens of milliseconds and the agent can call it at the start of each task without cost.

```sh
tnotes search "postgres failover" --json | jq -r '.[0].id'
tnotes cat vault/ops/postgres-failover --json | jq -r '.text'
```

Notes link to each other with `[[Title]]`. The `links` and `backlinks` arrays let an agent walk the graph without opening every file:

```sh
# everything the runbook points at, and everything that points back
tnotes ls --json | jq -r '.[] | select(.title == "Postgres failover") | .links[], .backlinks[]'
```

Writing has two safe verbs. `append` adds text to the end of an existing note; `new` creates one, with `--stdin` for a multi-line body:

```sh
tnotes append vault/ops/postgres-failover "- 2026-09-22: replica lagged 40 s after the 17:10 deploy"
printf '## Decision\nKeep the 500 ms autosave.\n' | tnotes new "ADR 014" --tag adr --folder decisions --stdin
```

`write --stdin` replaces the whole text. Use it only when the agent has just read the note; a changed first line renames the file and rewrites every `[[link]]` to it.

## Safety rules the CLI enforces

- `new` refuses a title that already exists unless `--duplicate` is given, so a retrying agent does not produce `Meeting notes`, `Meeting notes (2)`, `Meeting notes (3)`.
- `write` with empty input is refused.
- If a note was edited between the agent's read and its `write`, the write becomes a conflict copy plus an error instead of an overwrite. The same happens when the TUI holds unsaved edits: the TUI keeps its text and saves `name (conflict <time>).md` next to it.
- `trash` moves a note into the root's `.Trash/`; `restore` brings it back. The CLI has no command that deletes a file outright.
- Every id (`root/sub/stem`) is the same in `ls`, `search`, `cat`, `append`, `write`, `trash` and `restore`, so an agent can carry it through a whole task.

While the agent works, keep the TUI open. A clean tab follows a CLI `write` or `append` live, so you watch the agent's edits land and can correct them in place.

## What tnotes does not do

There is no daemon, no API and no embedding index. Search is fuzzy over titles and bodies, not semantic. If the agent needs vector retrieval, index the `.md` files with whatever you already use; the files are plain Markdown and nothing else is written next to them. There is also no permission model: an agent that can run `tnotes` can run `tnotes trash`. Scope it with `--dir` to a vault you are willing to let it change.

## Read next

- [CLI reference](/docs/cli) for every flag and the JSON shape.
- [Notes as memory for Claude Code](/guides/notes-memory-for-claude-code), a longer walkthrough with a full CLAUDE.md.
- [Agents](/agents) for the short version to paste into your own instructions.
