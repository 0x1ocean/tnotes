---
title: Sync and encryption
description: Sync tnotes with Syncthing, iCloud Drive, Dropbox or git and encrypt the vault with FileVault, gocryptfs or fscrypt — how conflicts and atomic saves keep notes safe.
order: 5
---

Notes are plain files, so syncing and encryption are the file system's job; tnotes only needs to behave well underneath.

## Sync

Put a root inside a folder synced by Syncthing, iCloud Drive, Dropbox, or keep it in a git repository.

- **Live updates.** Changes from another device show up immediately: the watcher reloads clean tabs, dirty tabs keep their text. The cursor stays where it was.
- **Atomic saves.** A note is written to a temporary file and renamed into place, so a syncer never sees a half-written note.
- **Conflicts, not overwrites.** When the same note is edited on two devices, tnotes never overwrites the other side: it keeps its own text as `name (conflict <time>).md`. Syncthing's `name.sync-conflict-*.md` and Dropbox's `name (conflicted copy).md` are listed as ordinary notes so nothing is hidden from you.
- **Ignored entries.** Dot-prefixed files and directories — `.stfolder`, `.git`, `.stversions`, iCloud `.name.md.icloud` placeholders — are skipped.

### git

```sh
cd ~/Documents/notes
git init && git add . && git commit -m "notes"
```

Commit from a cron job or a shell alias; `.Trash/` can go in `.gitignore` or be kept for history.

## Encryption

Encrypt the folder, not the notes, and point `roots` at the mounted, decrypted path. Everything — links, search, the CLI — keeps working, and a syncer only ever sees ciphertext.

| Platform | Options |
|---|---|
| macOS | FileVault (whole disk), or an encrypted `.dmg` mounted at `~/Documents/notes` |
| Linux | `gocryptfs` (per-directory, works on top of a synced folder), `fscrypt`, LUKS |

```sh
# gocryptfs: cipher dir is synced, plain dir is what tnotes opens
gocryptfs -init ~/Sync/notes.enc
gocryptfs ~/Sync/notes.enc ~/notes
tnotes ~/notes
```

## How files are stored

- One `.md` file per note; the filename is a slug of the first line (the title), and it is renamed when the title changes.
- Folders in the tree are directories on disk.
- Every root has its own `.Trash/` that mirrors the folder layout, so trashed notes can be restored to where they came from.
- Nothing else is written next to your notes.
