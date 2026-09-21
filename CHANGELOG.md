# Changelog

All notable changes to tnotes. The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/); the section for a version is used verbatim as its GitHub release notes.

## [Unreleased]

## [1.3.0] - 2026-09-21

### Added

- CLI `write <note> --stdin` (replace the text; a changed title renames the file and updates `[[links]]`; a concurrent edit becomes a conflict copy and an error; empty input is refused) and `append <note> [text|--stdin]`.
- CLI `restore <note>`, `--limit N` for `ls`/`search`, `text` in `cat --json`; `new` refuses a title that already exists unless `--duplicate` is given.
- Prebuilt release tarballs for Linux x86_64 (glibc, musl), Linux aarch64, and macOS (Apple Silicon, Intel), with SHA-256 sums.
- A note renamed on disk (`mv`, or `tnotes write` changing the title) keeps its tab in a running TUI; unsaved edits in that tab become a conflict copy next to the new file.
- Quitting with a note that cannot be saved (disk full, permissions) writes its text under the state directory (`…/tnotes/unsaved/`) and reports the path instead of losing it; trash, folder delete and reload refuse to proceed after a failed save.

### Fixed

- `ls --json` on a large vault resolved backlinks quadratically (2 s for 500 notes); now linear (40 ms).
- CLI: `--tag '#work'` works for `new`, `ls` and `search` (leading `#` stripped); a blank title is rejected instead of creating `untitled.md`; `append "- item"` no longer trips the argument parser.
- Clicking a row in a scrolled overlay (folder browser, backlinks, move picker) hit the wrong entry.
- Pasting while the settings page was open went into the hidden editor.
- With emacs keys, `ctrl+c`/`ctrl+q` now also quit from the settings page.
- External updates of an open note keep the cursor position instead of jumping to the top.
- `editor.width` from the config is clamped to 40..=160 like the settings page does.

## [1.2.0] - 2026-09-21

### Added

- `ctrl+c` quits, alongside `ctrl+q`.
- A mouse selection is copied to the clipboard when the button is released, like in a terminal; copy/cut confirm in the status bar.

### Fixed

- Clipboard on Wayland: copies went to the X11 selection via XWayland and never reached the Wayland clipboard; arboard now uses the `wayland-data-control` backend.
- Context-menu `cut` on a mouse selection did nothing (the copy step cleared the selection before the delete).
- With emacs keys, a mouse selection left the editor in edtui's Visual/Normal mode where no key did anything. The editor now stays in Insert; Backspace/Delete remove the selection, typing replaces it, any other key drops it.
- Sidebar text no longer touches the separator line: one blank column is kept before it.

## [1.1.1] - 2026-09-21

### Added

- `CHANGELOG.md` and a `Release` workflow: pushing a `v*` tag builds, tests, creates the GitHub release with the changelog section, and publishes to crates.io.
- README: `cargo install tnotes`, a "Use with AI agents" section, and an updated screenshot with `[[links]]` and backlinks.

## [1.1.0] - 2026-09-21

### Added

- Headless CLI: `tnotes ls | search | cat | new | trash`, all with `--json` and `--dir`. Notes are addressed by id (`root/sub/stem`), path, or a unique file stem. A running TUI picks up CLI changes live.
- `[[wikilinks]]`: resolved by title (case-insensitive), then by file stem. Completion popup after `[[`; `alt+enter` / `ctrl+click` follows a link and creates a missing target next to the current note.
- Backlinks overlay (`alt+enter` on a line without a link, `↩ N` in the status bar, editor context menu).
- Renaming a note rewrites `[[old title]]` → `[[new title]]` in every other note, including open unsaved tabs.
- `links` and `backlinks` fields in `--json` output.

### Changed

- New notes are named after their title (`# Groceries` → `groceries.md`); the default template still yields `untitled.md`.
- The vendored word-wrap fork of edtui is published as `edtui-tnotes`, so `cargo install tnotes` works.

## [1.0.0] - 2026-09-20

Initial release: folders, `#tags`, tabs with a preview tab, live Markdown highlighting, emacs/vim keys, mouse-first UI, session restore, in-app settings, atomic saves with conflict copies, live reload.

[Unreleased]: https://github.com/0x1ocean/tnotes/compare/v1.3.0...HEAD
[1.3.0]: https://github.com/0x1ocean/tnotes/compare/v1.2.0...v1.3.0
[1.2.0]: https://github.com/0x1ocean/tnotes/compare/v1.1.1...v1.2.0
[1.1.1]: https://github.com/0x1ocean/tnotes/compare/v1.1.0...v1.1.1
[1.1.0]: https://github.com/0x1ocean/tnotes/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/0x1ocean/tnotes/releases/tag/v1.0.0
