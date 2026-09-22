---
title: tnotes for sysadmins
description: "Runbooks and incident notes for sysadmins over ssh - a static musl binary, tmux-friendly TUI, --dir per host, and append from scripts to log what you did."
persona: Sysadmins
order: 4
keywords:
  - sysadmin notes terminal
  - runbook notes ssh
  - incident log cli
  - notes over ssh tmux
---

## The problem

The notes you need during an incident are on your laptop, and you are in a tmux session on a box that has no browser, no GUI and possibly no glibc you recognise. Runbooks end up in a wiki nobody updates, incident timelines get reconstructed from shell history, and the one-liner that fixed the disk last time is in a chat scrollback.

tnotes is a single binary that runs where you are already logged in. It reads a directory of `.md` files, so a runbooks folder in a repo is a vault with no conversion step, and the CLI lets a script write to the same notes.

## Setup in three steps

1. Put the static binary on the host. The musl build has no dependency on the system's C library, so the same tarball works on Alpine, an old CentOS or Debian:

```sh
V=1.3.0; T=x86_64-unknown-linux-musl
B=https://github.com/0x1ocean/tnotes/releases/download/v$V/tnotes-$V-$T.tar.gz
curl -sLO $B && curl -sL $B.sha256      # compare with: sha256sum tnotes-$V-$T.tar.gz
tar xzf tnotes-$V-$T.tar.gz && install tnotes-$V-$T/tnotes ~/.local/bin/
```

2. Point it at the runbooks directory for the session only. `--dir` uses that folder and leaves the config alone, which is what you want on a shared jump host:

```sh
tnotes --dir /srv/ops/runbooks
```

3. If the terminal is an 80-column serial console or a narrow tmux pane, tnotes drops to a single-panel layout on its own. For a plain terminal without a colour theme, switch highlighting to `mono` in `~/.config/tnotes/config.toml`:

```toml
[editor]
highlight = "mono"
width = 0

[appearance]
compact = true
dates = "absolute"
```

`compact = true` gives one-line rows and `dates = "absolute"` prints timestamps you can quote in a postmortem.

## Daily workflow

**Finding the runbook.** From the list, `/` filters; from a shell, `tnotes search "raid rebuild" --dir /srv/ops/runbooks` returns the best match first. `--folder` and `--tag` narrow it; `--limit 5` keeps the output short in a small pane.

**Logging the incident as it happens.** Open a note per incident and append from the shell you are already typing in. Each line lands at the end of the note, and the TUI in the next pane updates live:

```sh
N=$(tnotes new "INC-2291 nfs stale handles" --tag incident --folder incidents --dir /srv/ops/runbooks)
tnotes append "$N" "- $(date -u +%H:%M) remounted /export on web03, load back to 2" --dir /srv/ops/runbooks
```

`new` prints the id it created, so a shell variable carries it through the rest of the incident. A second `new` with a title that already exists is refused unless you pass `--duplicate`, which stops two people from opening two `INC-2291` notes.

**Capturing command output.** Pipe it in as the body of a new note or onto an existing one:

```sh
smartctl -a /dev/sda | tnotes new "sda SMART 2026-09-22" --tag hardware --stdin --dir /srv/ops/runbooks
journalctl -u nfs-server --since -1h | tnotes append "$N" --stdin --dir /srv/ops/runbooks
```

**Copying out.** Over ssh, use `shift+drag` for the terminal's own selection so the text ends up on your local machine; a plain drag is copied by tnotes, which is running on the remote host.

**Quitting on a full disk.** If a note cannot be saved because the filesystem is full or the permissions changed under you, quitting writes its text under `~/.local/state/tnotes/unsaved/` and prints the path. Trash, folder delete and reload refuse to proceed after a failed save, so a bad disk does not compound the damage.

## Runbooks next to the code

Keep the runbooks directory in the same git repository as the Ansible roles or Terraform that they describe. tnotes writes nothing beside the `.md` files and ignores dot-prefixed entries, so `.git/` never shows up as a folder. Commit the incident notes with the fix. If two people edited the same runbook on two hosts before pushing, tnotes keeps its own version as `name (conflict <time>).md` instead of overwriting; resolve it like any merge.

## What tnotes does not do

There is no multi-user server, no access control and no audit log; file permissions and git history are those things. There is no encryption, so put a sensitive vault on a `gocryptfs` or LUKS volume and point `--dir` at the mounted path. There is no alerting or ticket integration: `tnotes append` is a line in a shell script, and the script is where the integration goes.

## Read next

- [CLI reference](/docs/cli) for `--json` output you can feed to `jq` in a status script.
- [Sync and encryption](/docs/sync) for git, Syncthing and encrypted volumes.
- [Terminal note-taking](/guides/terminal-note-taking) for the general workflow.
