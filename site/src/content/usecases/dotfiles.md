---
title: tnotes for dotfiles people
description: "Keep the tnotes config.toml in your dotfiles repo, set roots per machine, and put the vault in git with .Trash ignored: a reproducible notes setup."
persona: Dotfiles
order: 5
keywords:
  - notes app config in dotfiles
  - reproducible notes setup
  - git notes vault
  - toml config notes terminal
---

## The problem

A new machine should be one `git clone` and one bootstrap script away from feeling like the old one. Notes apps fight that: settings live in a SQLite database, a JSON blob with machine-specific paths, or a cloud account. You can version the notes, or the settings, rarely both, and never with a diff you can read.

tnotes has one config file in TOML, one state file you do not want to version, and a vault that is a directory of Markdown. All three boundaries are clean, so the whole setup fits a dotfiles repo plus a notes repo.

## Setup in three steps

1. Track the config. It lives at `~/.config/tnotes/config.toml` on Linux and `~/Library/Application Support/tnotes/config.toml` on macOS. Symlink it from your repo or let your dotfiles manager place it. Only keys you change need to be present; unset keys keep their defaults:

```toml
roots = ["~/notes"]

[appearance]
compact = true
sidebar_width = 28

[editor]
keys = "emacs"
autosave_ms = 300
width = 80
cursor = "bar"

[theme]
accent = "#8fbcbb"
tag = "#a3be8c"
```

The theme keys take names like `cyan` or hex values, and the seven-colour default follows your terminal palette, so a config that names no colours already matches whatever theme your terminal uses on each machine.

2. Do not track the state. Open tabs, filter, sort, focus and folded sections are saved to `~/.local/state/tnotes/state.toml` and restored on launch. That file is per machine by nature and would produce a conflict on every commit.

3. Make the vault a repo of its own:

```sh
cd ~/notes
git init
printf '.Trash/\n' > .gitignore
git add . && git commit -m "notes"
```

Every root has its own `.Trash/` that mirrors the folder layout. Ignoring it keeps trashed notes out of history; leaving it tracked keeps them recoverable on every machine. Both are defensible; pick one and write it down.

## Roots per machine

`roots` is a list, and `~` expands per user, so a path like `~/notes` resolves correctly on a laptop and a desktop with different usernames. When machines differ in which vaults they should see, the choices are:

- one config per machine class, kept as separate files in the repo and linked by hostname in the bootstrap script;
- a templated config from a dotfiles manager that emits a different `roots` line per hostname;
- `tnotes --dir ~/scratch` for a folder that should never enter the config.

Avoid `tnotes ~/some/path` on a managed machine: that form appends the path to `roots` and saves the config, which is exactly the kind of drift a dotfiles repo exists to prevent. Use `--dir` for one-offs.

Legacy note: a `notes_dir = "…"` key from 0.x is still read when `roots` is absent. If your repo predates 1.0, migrate it to `roots` so the file matches the documented format.

## The settings page writes the file

`F2` (or `,` from the list) opens an in-app settings page that writes `config.toml` for you. That is convenient and slightly at odds with a tracked file: after you change something in-app, run `git diff` in the dotfiles repo and either commit the change or revert it. If you prefer the file to be the only source of truth, treat `F2` as a preview and make the edit in the repo.

## Daily workflow

Commit the vault the way you commit anything: a shell alias, a cron job, or a `git commit -am` at the end of the day. tnotes writes atomically, so a commit never catches a half-written note, and `.git/` is a dot-prefixed directory, so it never appears as a folder in the tree.

When you pull on another machine, a running tnotes reloads clean tabs live and keeps the cursor where it was. A tab with unsaved edits keeps them, and if the pulled file differs, tnotes writes `name (conflict <time>).md` next to it rather than losing either version. The conflict copy shows up as an ordinary note, so it will be in your next `git status`.

To confirm a fresh machine came up correctly, use the CLI instead of clicking around:

```sh
tnotes ls --limit 5           # newest five notes: proves the roots resolved
tnotes ls --json | jq length  # total count, compare with the other machine
```

## What tnotes does not do

It does not sync. git, Syncthing or any file sync does that, and tnotes is designed to sit under them. It does not encrypt: keep the vault on FileVault, `gocryptfs` or LUKS and point `roots` at the mounted path. It does not template the config; hostname-dependent values are the job of your dotfiles manager's templates or a shell script.

## Read next

- [Configuration](/docs/config) for every key and its allowed range.
- [Sync and encryption](/docs/sync) for the conflict-copy rules in detail.
- [Plain-text notes sync](/guides/plain-text-notes-sync) for a full git-based setup.
