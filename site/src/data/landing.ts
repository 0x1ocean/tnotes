import { RELEASES, REPO, RUST_VERSION, TAP, VERSION } from "@/lib/meta";

const tarball = (target: string) =>
  `$ V=${VERSION}; T=${target}\n$ curl -sL ${RELEASES}/download/v$V/tnotes-$V-$T.tar.gz | tar xz\n$ install tnotes-$V-$T/tnotes ~/.local/bin/`;

export type InstallMethod = { title: string; command: string; note: string; upgrade?: string };
export type InstallOs = {
  id: string;
  label: string;
  /** `navigator.platform` prefixes that select this tab; first OS is the no-JS default. */
  platforms: string[];
  recommended: InstallMethod;
  alternatives: InstallMethod[];
  caveat?: string;
};

/** Grouped by OS, one recommended path each, alternatives below it. */
export const INSTALL: InstallOs[] = [
  {
    id: "linux",
    label: "Linux",
    platforms: ["Linux"],
    recommended: {
      title: "Static binary, any distro",
      command: tarball("x86_64-unknown-linux-musl"),
      note: "musl build: no glibc version to match, works on Debian, Arch, Alpine, NixOS, a fresh container. On aarch64 (Raspberry Pi, Graviton): T=aarch64-unknown-linux-gnu.",
      upgrade: "re-run with the new version number",
    },
    alternatives: [
      { title: "Homebrew on Linux", command: `$ brew install ${TAP}`, note: "x86_64 and aarch64. Prebuilt, no Rust needed.", upgrade: "brew upgrade tnotes" },
      { title: "cargo", command: "$ cargo install tnotes", note: `Builds from crates.io. Rust ≥ ${RUST_VERSION}.`, upgrade: "cargo install tnotes --force" },
    ],
    caveat: "No AUR, deb or rpm packages yet; the static tarball is the portable answer until then.",
  },
  {
    id: "macos",
    label: "macOS",
    platforms: ["Mac"],
    recommended: {
      title: "Homebrew",
      command: `$ brew install ${TAP}`,
      note: "Apple Silicon and Intel. Prebuilt bottle, no Rust needed.",
      upgrade: "brew upgrade tnotes",
    },
    alternatives: [
      { title: "Prebuilt tarball, Apple Silicon", command: tarball("aarch64-apple-darwin"), note: "Intel: T=x86_64-apple-darwin. A .sha256 sits next to every tarball.", upgrade: "re-run with the new version number" },
      { title: "cargo", command: "$ cargo install tnotes", note: `Builds from crates.io. Rust ≥ ${RUST_VERSION}.`, upgrade: "cargo install tnotes --force" },
    ],
    caveat: "Binaries are not signed. Homebrew and curl downloads run as-is; a tarball saved through the browser is quarantined by Gatekeeper: xattr -d com.apple.quarantine tnotes clears it.",
  },
  {
    id: "source",
    label: "From source",
    platforms: [],
    recommended: {
      title: "cargo, from crates.io",
      command: "$ cargo install tnotes",
      note: `Any platform with Rust ≥ ${RUST_VERSION}. Windows is untested.`,
      upgrade: "cargo install tnotes --force",
    },
    alternatives: [
      { title: "Current main branch", command: `$ cargo install --git ${REPO}`, note: "Includes the vendored word-wrap fork of edtui. May be ahead of the changelog.", upgrade: "re-run" },
    ],
  },
];

export const PILLARS = [
  {
    title: "Files, not a database",
    body: "One <code>.md</code> per note, folders are directories, tags live in the text. Atomic saves, conflict copies instead of silent overwrites, live reload. Sync with anything.",
  },
  {
    title: "A real editor",
    body: "Live highlighting, list continuation, checkbox toggling, word wrap, link and tag completion, backlinks. Emacs or vim keys. Mouse works everywhere.",
  },
  {
    title: "Scriptable by design",
    body: "<code>tnotes ls --json</code> gives every note with tags, links, backlinks and timestamps. Agents and cron jobs write; your open TUI updates live.",
  },
];

/** `icon` is 1–3 lines of monospace art; `body` is trusted HTML (kbd/code). */
export const FEATURES = [
  { title: "Plain Markdown on disk", icon: "┌─┐\n│▒│.md\n└─┘", body: "Atomic saves, a <code>name (conflict &lt;time&gt;).md</code> copy instead of an overwrite, and live reload when a file changes outside the app." },
  { title: "Folders and #tags", icon: "▾ work\n  └ #project", body: "Folders are real directories. Tags are parsed from the text, nested ones like <code>#work/project</code> included. Click a tag to filter." },
  { title: "[[Links]] and backlinks", icon: "[[note]] ←→ ↩ 3", body: "Completion popup after <code>[[</code>, <kbd>alt+enter</kbd> or <kbd>ctrl+click</kbd> to follow (missing targets are created), a backlinks overlay, and links rewritten on rename." },
  { title: "Tabs with a preview tab", icon: "1 roadmap  2 plan ×", body: "Selecting a note previews it; typing or <kbd>enter</kbd> pins it. Jump with <kbd>1–9</kbd>, cycle with <kbd>alt+.</kbd> / <kbd>alt+,</kbd>." },
  { title: "Live Markdown highlighting", icon: "# ** _ ` > - [x]", body: "Headings, lists, checkboxes, code, links, emphasis, quotes and tags styled as you type. Colour or monochrome." },
  { title: "Lists that behave", icon: "- [ ] ⏎\n  - [ ] ⇥", body: "<kbd>enter</kbd> continues a list, an empty item ends it, <kbd>tab</kbd> / <kbd>shift+tab</kbd> nest, <kbd>alt+x</kbd> or a click toggles a checkbox." },
  { title: "Emacs or vim keys", icon: "C-s  M-f  :wq", body: "Powered by edtui. Emacs by default; vim with normal/insert modes, <code>.</code> repeat, <code>/</code> search. Switch in settings." },
  { title: "Mouse-first", icon: "click ·· drag\n   right-click ▸", body: "Click to select, double-click to pin, right-click context menus, wheel scrolling, drag to select text (copied on release)." },
  { title: "Fuzzy search", icon: "/ milk_ ▸ 3", body: "<kbd>/</kbd> searches titles and bodies with a fuzzy matcher (nucleo), combined with tag and folder filters." },
  { title: "Session restore", icon: "↻ tabs · filter · sort", body: "Open tabs, filter, sort, focus and folded tree sections come back exactly as you left them." },
  { title: "In-app settings", icon: "F2 ▸ [x] wrap", body: "<kbd>F2</kbd> or <kbd>,</kbd> opens a settings page that writes <code>config.toml</code> for you. Theme follows your terminal palette." },
  { title: "Headless JSON CLI", icon: "$ tnotes ls --json", body: "<code>ls · search · cat · new · append · write · trash · restore</code>. Same ids everywhere. A running TUI picks changes up live." },
  { title: "Small terminals", icon: "[▪]  ◂  [▪▪]", body: "A single-panel layout kicks in on narrow windows; <kbd>ctrl+b</kbd> toggles the sidebar." },
  { title: "Trash, not delete", icon: ".Trash/ ↩ restore", body: "Every root has its own <code>.Trash/</code> mirroring the folder layout, so <kbd>u</kbd> undoes and <kbd>r</kbd> restores to where a note came from." },
  { title: "Never loses text", icon: "● saved  ▲ unsaved/", body: "If a save fails (disk full, permissions) the text is written under the state dir and the path is shown instead of losing it." },
];

export const WORKFLOWS = [
  {
    title: "Capture",
    body: "<kbd>ctrl+t</kbd>, type a title, keep typing. Autosave after 500 ms of idle.",
    code: "# Groceries\n\n#home\n\n- [ ] milk\n- [ ] coffee⏎\n- [ ] ▌",
  },
  {
    title: "Link",
    body: "Type <code>[[</code>, pick from the popup, <kbd>alt+enter</kbd> to jump. Rename a note and every link follows.",
    code: "- [ ] Prep [[Meet▌\n         ┌────────────────┐\n         │ Meeting notes  │\n         │ Meetup ideas   │\n         └────────────────┘",
  },
  {
    title: "Automate",
    body: "Pipe text in from anywhere. The open TUI shows it the moment the file lands.",
    code: '$ echo "- [ ] call Ann" \\\n  | tnotes append weekly-plan --stdin\n$ tnotes search "berlin" --json \\\n  | jq \'.[0].path\'\n"vault/weekly-plan.md"',
  },
];

export const AGENT_JSON = `$ tnotes ls --json | jq '.[0]'
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
}`;

export const CONFIG_SAMPLE = `~/.config/tnotes/config.toml

roots = ["~/Sync/notes", "~/work/wiki"]

[editor]
keys = "vim"
autosave_ms = 500
width = 72

[theme]
accent = "cyan"
tag = "green"`;

export const EXPLORE = [
  { href: "/compare", title: "Compare", description: "tnotes vs Obsidian, nb, jrnl, Logseq, vimwiki, org-mode and more, with recipes for using both." },
  { href: "/for", title: "Use cases", description: "Setups for vim users, Obsidian users, sysadmins over SSH, dotfiles people and AI agents." },
  { href: "/guides", title: "Guides", description: "Terminal note-taking, wikilinks, plain-text sync and encryption, notes memory for Claude Code." },
  { href: "/glossary", title: "Glossary", description: "Wikilink, backlink, TUI, Zettelkasten, vault, frontmatter, and how each works here." },
];

export const FAQ = [
  {
    q: "Where are my notes stored?",
    a: "As plain .md files in the folders you choose (default ~/Documents/notes). One file per note, named after its title. Nothing else is written next to them, so the folder is safe to sync with git, Syncthing, iCloud Drive or Dropbox.",
  },
  {
    q: "Does tnotes work with Obsidian or other Markdown apps?",
    a: "Yes. Notes are ordinary Markdown with [[wikilinks]] and #tags in the text, so the same folder can be opened by any Markdown editor. tnotes reloads live when a file changes outside the app.",
  },
  {
    q: "Can I use vim keys?",
    a: 'Yes. Set editor.keys = "vim" in the config or in the in-app settings page (F2). The default is emacs-style keys.',
  },
  {
    q: "How do AI agents use tnotes?",
    a: "Through the headless CLI: tnotes ls, search, cat, new, append, write, trash and restore, all with --json. No daemon, no API, just files. A running TUI picks the changes up live.",
  },
  { q: "Is it free?", a: "Yes. tnotes is open source under the MIT license." },
  {
    q: "Does it run on Windows?",
    a: "Untested. Prebuilt binaries ship for macOS and Linux; cargo install may work on Windows but is not supported.",
  },
];
