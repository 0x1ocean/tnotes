# Changelog

All notable changes to tnotes. The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/); the section for a version is used verbatim as its GitHub release notes.

## [Unreleased]

### Added

- `ctrl+c` quits, alongside `ctrl+q`.

### Fixed

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

[Unreleased]: https://github.com/0x1ocean/tnotes/compare/v1.1.1...HEAD
[1.1.1]: https://github.com/0x1ocean/tnotes/compare/v1.1.0...v1.1.1
[1.1.0]: https://github.com/0x1ocean/tnotes/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/0x1ocean/tnotes/releases/tag/v1.0.0
