---
title: "Syncing plain-text notes: git vs Syncthing vs iCloud vs Dropbox"
description: "git, Syncthing, iCloud Drive and Dropbox for a Markdown notes folder: conflict files, atomic writes, junk entries, and encryption via gocryptfs, fscrypt or age."
published: 2026-09-22
keywords: ["sync markdown notes", "syncthing notes", "git notes sync", "notes encryption", "gocryptfs"]
order: 4
---

A folder of `.md` files is the easiest thing in the world to sync, which is why people pick [plain-text notes](/glossary/plain-text-notes) in the first place. What separates the transports is the failure cases: the same note edited on two machines before either syncs, a write that lands while the syncer is reading, a laptop that was offline for a week. For each of git, Syncthing, iCloud Drive and Dropbox, this guide answers four questions: what a conflict leaves on disk, whether a half-written file can be shipped, what junk ends up in the folder, and who can read the bytes. Then atomic writes, encryption, and how tnotes behaves underneath.

## git

Nothing moves until you say so. Sync is `git pull` and `git push`, and each device has full history.

**Conflicts.** A merge that touches the same lines on both sides stops and writes markers into the file: `<<<<<<<`, `=======`, `>>>>>>>`, with your side above the `=======` and theirs below ([git-merge: how conflicts are presented](https://git-scm.com/docs/git-merge#_how_conflicts_are_presented)). Setting `merge.conflictStyle` to `zdiff3` adds a `|||||||` section with the original text, which for prose is the difference between guessing and knowing. Changes to different parts of the same note merge cleanly, which no file-level syncer can do.

**Partial writes.** None. git commits snapshots; a half-written file is either in the commit or not.

**Junk.** A `.git/` directory at the root, and conflict markers inside notes until you resolve them. No notes app understands the markers, so a conflicted note renders as a note containing `<<<<<<<`.

**Trust.** Depends on the remote. A bare repository over SSH on a host you own exposes plaintext only to that host; GitHub or GitLab can read every note.

**The cost.** Somebody has to commit: a cron job or alias on one machine, a pull before and a push after on several, and forgetting either is how conflicts happen. Mobile is poor. Right for a vault that is also a project and for dotfiles-style setups; see [/for/dotfiles](/for/dotfiles).

```sh
cd ~/notes && git init && git add -A && git commit -m notes
git config merge.conflictStyle zdiff3
printf '.Trash/\n' >> .gitignore
```

## Syncthing

Peer-to-peer, continuous, no server. A device that is off catches up later.

**Conflicts.** Detected and kept. When a file was modified on two devices and the content differs, the copy with the older modification time is renamed to `<filename>.sync-conflict-<date>-<time>-<modifiedBy>.<ext>`; equal times are broken by device id, and a modification that loses to a deletion becomes a conflict copy too. Conflict files are ordinary files and propagate to every device ([Syncthing docs: Understanding synchronization](https://docs.syncthing.net/users/syncing.html)). Nothing is merged, nothing is lost; you get two files and a decision.

**Partial writes.** Syncthing never writes the destination directly: it assembles `.syncthing.<name>.tmp` and moves it into place. A file is rehashed whenever its modification time, size or permissions change, so one picked up half-written is rescanned once the writer finishes. The watcher batches changes for 10 seconds and delays deletions a further minute.

**Junk.** `.stfolder` in the root, `.syncthing.*.tmp` during transfers, `.stversions/` if versioning is on, and `*.sync-conflict-*` files. Versioning is off by default and only archives changes arriving from other devices, never your own local edits ([Syncthing docs: File versioning](https://docs.syncthing.net/users/versioning.html)).

**Trust.** Device-to-device traffic is TLS, a peer is admitted only if its certificate fingerprint (the device ID) is on your list, and relays cannot inspect the data ([Syncthing docs: Security principles](https://docs.syncthing.net/users/security.html)). No third party holds the notes; each device's folder is plaintext.

**The cost.** Two devices must both be up; a sleeping phone does not receive. Conflict copies accumulate if ignored. For two or three machines you own, it is the least surprising option here.

## iCloud Drive

Built into macOS and iOS; nothing to install inside Apple's platforms, nothing to use outside them.

**Conflicts.** Apple's documentation describes conflicts arising when more than one offline device edits the same document; when they come online, a dialog asks which versions to keep. Kept versions are numbered ("Seven Wonders" and "Seven Wonders 2"), and versions you do not select are deleted from iCloud Drive on every device ([Apple: If document versions conflict in iCloud Drive](https://support.apple.com/guide/mac-help/mh40780/mac)). Resolution is interactive, in Finder or the app, not a file you can grep for.

**Partial writes.** Not documented at the level Syncthing documents it. Treat it as opaque.

**Junk.** Files evicted from local storage appear as `.name.md.icloud` placeholders until downloaded. An app that ignores dot-prefixed entries will not show them, and will not see the note until it is fetched.

**Trust.** Apple holds the data. How much of it Apple can read depends on account-level protection settings this guide does not verify; encrypting the folder yourself removes the question.

**The cost.** Everything is implicit: when a file uploads, when it is evicted, when a conflict is noticed. Zero configuration for Mac and iPhone; Linux does not participate.

## Dropbox

Hosted; a client per device; a web view.

**Conflicts.** When the same file is edited at the same time, edited offline by two people, or left open in an auto-saving app on another machine, Dropbox keeps both: the later save becomes a file with the editor's username, "conflicted copy", and the date in the name, and Dropbox's guidance is to merge by hand ([Dropbox: What's a conflicted copy?](https://help.dropbox.com/organize/conflicted-copy)). The auto-save case is the one that bites notes apps: an editor saving on idle, on a machine you forgot about, generates conflicted copies by itself.

**Partial writes.** Not specified in the help article; a racing write produces a second file rather than a merge.

**Junk.** `name (conflicted copy).md` files beside your notes.

**Trust.** Dropbox can read the files unless you encrypt beneath it.

**The cost.** A third party with the notes and conflict files that look like notes. It is cross-platform, which iCloud Drive is not, and needs no peer online, which Syncthing does.

## Atomic writes, and why they matter to all four

If an app saves by truncating the file and streaming new content, there is a window where the file is empty or half-written, and a watcher that fires inside it ships the broken state everywhere; the other device may then "win" a conflict with the truncated version.

The fix is old: write a temporary file in the same directory, then `rename()` it over the original. An existing `newpath` is atomically replaced, no process ever observes the path missing, and if the rename fails an instance of `newpath` remains ([rename(2)](https://man7.org/linux/man-pages/man2/rename.2.html)). Syncthing writes this way on the receiving side; a notes tool has to do it on the writing side.

The other half is the reload: when a file changes on disk under an open buffer, the app must reload a clean buffer and must not reload a dirty one. Reloading a dirty buffer loses your typing; never reloading overwrites the other device's change on the next save.

## Encryption

Only git over SSH to a host you own keeps the transport from reading the notes. Otherwise, encrypt below the syncer and point the notes app at the decrypted view.

| Tool | Layer | Syncer sees | Filenames hidden | Platforms | Fit |
|---|---|---|---|---|---|
| FileVault | whole disk | plaintext (sync runs above the disk layer) | no | macOS | protects a lost laptop, not the cloud copy |
| encrypted `.dmg` | disk image, mounted | one image file | yes | macOS | sync the image; coarse, whole-image transfers |
| [gocryptfs](https://github.com/rfjakob/gocryptfs) | FUSE overlay, per file | one ciphertext file per note, 18-byte header plus 32 bytes per 4 KiB block | yes | Linux; macOS beta | best fit for Syncthing, Dropbox, iCloud |
| [fscrypt](https://github.com/google/fscrypt) | kernel, per directory on ext4/f2fs | plaintext (encryption is below the syncer) | contents and names encrypted; sizes, counts and timestamps not | Linux | protects the disk, not the cloud copy |
| [age](https://github.com/FiloSottile/age) | per file, explicit | one `.age` file per run | as good as what you tar | all | backups and archives, not live sync |

The table turns on where the syncer reads. FileVault and fscrypt encrypt the disk; a syncer running as your user reads decrypted files and uploads plaintext. gocryptfs and a `.dmg` give the syncer ciphertext while you work in the mounted view. gocryptfs is the practical one: a one-note edit syncs one small file, and its README lists a 2017 security audit and `apt`, `pacman` and MacPorts packages. age is explicit encrypt and decrypt with small keys and no configuration, for a `tar` of the vault on its way to a backup target.

```sh
# gocryptfs: ciphertext under the synced folder, plaintext mounted outside it
gocryptfs -init ~/Sync/notes.enc
gocryptfs ~/Sync/notes.enc ~/notes
```

```sh
# age: encrypted archive for a backup destination
tar cz ~/notes | age -r age1... > notes-$(date +%F).tar.gz.age
```

## How tnotes behaves underneath

From the README's sync section, and only that:

- A root can sit inside a folder synced by Syncthing, iCloud Drive or Dropbox, or be a git repository. Apart from each root's own `.Trash/`, nothing is written next to your notes.
- Changes from another device show up live: the watcher reloads clean tabs; dirty tabs keep their text.
- Saves are atomic, so a syncer never sees a half-written note.
- When the same note is edited on two devices, tnotes never overwrites the other side; it keeps its own text as `name (conflict <time>).md`. Syncthing's `name.sync-conflict-*.md` and Dropbox's `name (conflicted copy).md` are listed as ordinary notes, so nothing is hidden.
- Dot-prefixed entries are ignored: `.stfolder`, `.git`, `.stversions`, iCloud `.name.md.icloud` placeholders.
- For encryption, the README's advice is to encrypt the folder rather than the notes (FileVault or an encrypted `.dmg` on macOS, gocryptfs or fscrypt on Linux) and point `roots` at the mounted, decrypted path; links, search and the CLI keep working, and a syncer only sees ciphertext.

A tnotes conflict copy and a Syncthing conflict copy come from different races, and one note can end up with both; both show in the list. A git merge conflict is a note containing `<<<<<<<` until you fix it in an editor. Setup recipes are in [/docs/sync](/docs/sync).

## Picking one

- Two or three machines you own, any OS: Syncthing, with gocryptfs underneath if one is a laptop that travels.
- You want line-level history and already commit everything else: git with `zdiff3`; see [/for/sysadmins](/for/sysadmins) for the cron variant.
- Mac and iPhone only, minimum setup: iCloud Drive, and accept that conflicts resolve in a dialog.
- Cross-platform with a phone and no peer to keep online: Dropbox, with a `.dmg` or gocryptfs directory as the payload if the notes are private.

Whichever you pick, run the two-device test on day one: edit the same note on both while one is offline, reconnect, and look at what is on disk. Every transport above keeps the text; they differ in what they ask you to do next.
