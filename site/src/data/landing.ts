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

/** Legend for the annotated TUI; numbers match `a(n, …)` calls in FakeTui.astro. */
export const ANNOTATIONS = [
  { n: 1, title: "Folders are directories", body: "Every root and subfolder in the tree is a real directory on disk." },
  { n: 2, title: "#tags, parsed from the text", body: "Nested ones like <code>#work/project</code> too. Click one to filter." },
  { n: 3, title: "Fuzzy search", body: "<kbd>/</kbd> searches titles and bodies (nucleo), combined with tag and folder filters." },
  { n: 4, title: "Tabs with a preview tab", body: "Selecting a note previews it; typing or <kbd>enter</kbd> pins it. <kbd>1–9</kbd> jumps." },
  { n: 5, title: "Tags live in the note", body: "No frontmatter, no sidecar file: the tag line is the tag." },
  { n: 6, title: "Lists that behave", body: "<kbd>enter</kbd> continues, an empty item ends, <kbd>tab</kbd> nests, <kbd>alt+x</kbd> or a click toggles the box." },
  { n: 7, title: "[[Links]] with completion", body: "Popup after <code>[[</code>, <kbd>alt+enter</kbd> follows, a missing target is created, links are rewritten on rename." },
  { n: 8, title: "Live Markdown highlighting", body: "Headings, emphasis, code, quotes, links and tags styled as you type; colour or mono." },
  { n: 9, title: "Autosave, atomic, never lost", body: "Written after 500 ms idle to a temp file then renamed; a conflict copy instead of an overwrite; a failed save keeps the text under the state dir." },
  { n: 10, title: "Backlinks", body: "Count in the status bar, overlay on <kbd>alt+enter</kbd>, and a <code>backlinks</code> field in the CLI's JSON." },
  { n: 11, title: "Emacs or vim keys", body: "edtui underneath. Switch on the settings page (<kbd>F2</kbd>)." },
];

/** man-page style feature list: SECTION → flag → description (trusted HTML). */
export const MAN = [
  {
    section: "FILES",
    items: [
      { flag: ".md on disk", body: "One file per note, named after its title. Atomic saves, <code>name (conflict &lt;time&gt;).md</code> instead of a silent overwrite, live reload when a file changes outside the app." },
      { flag: "folders", body: "Real directories. Every root has its own <code>.Trash/</code> mirroring the layout, so <kbd>u</kbd> undoes and <kbd>r</kbd> restores." },
      { flag: "#tags", body: "Parsed from the text, nested <code>#work/project</code> included. Nothing else is written next to your notes." },
      { flag: "sync", body: "Any file sync: git, Syncthing, iCloud, Dropbox. Syncer conflict files are listed as ordinary notes." },
    ],
  },
  {
    section: "EDITOR",
    items: [
      { flag: "[[links]]", body: "Completion popup after <code>[[</code>, <kbd>alt+enter</kbd> or <kbd>ctrl+click</kbd> to follow, backlinks overlay, rewritten when a title changes." },
      { flag: "- [ ] lists", body: "<kbd>enter</kbd> continues, empty item ends, <kbd>tab</kbd> / <kbd>shift+tab</kbd> nest, <kbd>alt+x</kbd> toggles a checkbox." },
      { flag: "highlighting", body: "Headings, lists, checkboxes, code, links, emphasis, quotes, tags as you type. Colour or monochrome; the theme follows your terminal palette." },
      { flag: "emacs | vim", body: "edtui keys. Vim has normal and insert modes, <code>.</code> repeat and <code>/</code> search." },
      { flag: "wrap, width", body: "Word wrap at a configurable column, left or centred, drawn or terminal cursor." },
    ],
  },
  {
    section: "NAVIGATION",
    items: [
      { flag: "tabs", body: "One preview tab; editing pins it. <kbd>1–9</kbd> jumps, <kbd>alt+.</kbd> / <kbd>alt+,</kbd> cycle, <kbd>ctrl+w</kbd> closes." },
      { flag: "/ search", body: "Fuzzy over titles and bodies, combined with the tag and folder filter. <kbd>s</kbd> changes the sort." },
      { flag: "mouse", body: "Click selects, double-click pins, right-click opens a context menu, wheel scrolls, drag selects text and copies on release." },
      { flag: "session", body: "Open tabs, filter, sort, focus and folded tree sections come back on the next launch." },
      { flag: "narrow", body: "A single-panel layout on small terminals; <kbd>ctrl+b</kbd> toggles the sidebar." },
      { flag: "F2", body: "An in-app settings page that writes <code>config.toml</code> for you." },
    ],
  },
  {
    section: "CLI",
    items: [
      { flag: "tnotes ls --json", body: "Every note with id, path, title, folder, tags, links, backlinks, created, modified, preview. <code>--tag</code>, <code>--folder</code>, <code>--limit</code>." },
      { flag: "search · cat", body: "Fuzzy search, best first; <code>cat</code> prints a note, <code>--json</code> adds the text." },
      { flag: "new · append · write", body: "Create, add a line, replace the text; all take <code>--stdin</code>. A changed title renames the file and updates links." },
      { flag: "trash · restore", body: "To the root's <code>.Trash/</code> and back. Nothing is deleted outright." },
      { flag: "live", body: "A running TUI picks CLI changes up; a tab with unsaved edits is never overwritten. No daemon, no API: files and JSON, which is all an agent needs." },
    ],
  },
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
