import type { Tool } from "../types";

export const tool: Tool = {
  slug: "obsidian",
  name: "Obsidian",
  url: "https://obsidian.md",
  tagline:
    "Desktop and mobile Markdown knowledge base built around [[links]], a graph and a large plugin ecosystem.",
  category: "desktop app",
  platforms: "Windows, macOS, Linux, iOS, iPadOS, Android",
  license: "Proprietary (free for personal and commercial use)",
  pricing:
    "Free. Optional add-ons: Sync from $4/user/month billed annually, Publish from $8/site/month, Catalyst $25 one-time, Commercial $50/user/year.",
  features: {
    storage: {
      v: "yes",
      note: "Markdown files in a vault folder; per-vault settings in `.obsidian/`",
    },
    terminal: {
      v: "partial",
      note: "Obsidian CLI (installer 1.12.7+) drives the running desktop app; no terminal editor",
    },
    wikilinks: {
      v: "yes",
      note: "[[Note]], [[Note#heading]], [[Note|alias]], [[Note#^block]]; updated on rename",
    },
    backlinks: {
      v: "yes",
      note: "Backlinks core plugin: linked and unlinked mentions",
    },
    tags: {
      v: "yes",
      note: "#tag and nested #inbox/to-read in text, or a `tags` property",
    },
    cli: {
      v: "partial",
      note: "`obsidian` CLI needs the app running; `ob` (Headless) only syncs and publishes",
    },
    json: {
      v: "partial",
      note: "`format=json` on some CLI commands (backlinks, links, tags, bases)",
    },
    liveReload: {
      v: "yes",
      note: "refreshes the vault when files change on disk",
    },
    vim: { v: "yes", note: "Settings → Editor → Vim key bindings" },
    mouse: { v: "yes" },
    sync: {
      v: "yes",
      note: "Obsidian Sync add-on, or any file sync (Dropbox, iCloud, git)",
    },
    encryption: {
      v: "partial",
      note: "Sync is end-to-end encrypted (AES-256-GCM); the local vault is not",
    },
    mobile: { v: "yes", note: "iOS, iPadOS, Android" },
    plugins: {
      v: "yes",
      note: "core plugins, community plugins, themes, CSS snippets",
    },
    openSource: {
      v: "no",
      note: "closed source; JSON Canvas and Knap formats are MIT",
    },
  },
  intro: `Obsidian is a Markdown editor and knowledge base that runs on a local folder it calls a vault. Every note is a \`.md\` file, subfolders are folders, and vault-specific settings live in a hidden \`.obsidian/\` directory at the vault root. Because the data is plain files, Obsidian's own docs list Dropbox, iCloud, OneDrive and git as valid ways to sync a vault, alongside the paid Obsidian Sync service.

The model is links first. \`[[Note]]\` links resolve by filename, can target headings (\`[[Note#Heading]]\`) and blocks (\`[[Note#^id]]\`), and are rewritten when a file is renamed. The Backlinks core plugin shows linked and unlinked mentions for the active note, Graph view draws the whole vault, and Bases turns frontmatter properties into database-style tables. Community plugins, themes and CSS snippets extend all of that.

Two recent additions matter for people who script their notes. Obsidian CLI (requires the 1.12.7 installer) exposes app commands from a shell, with \`format=json\` on some of them, but it controls a running desktop app rather than reading files on its own. Obsidian Headless (\`ob\`, open beta, needs a Sync subscription) syncs and publishes vaults from a server without the app.`,
  chooseTool: [
    "You want a GUI: graph view, canvas, Bases tables, PDF and image attachments, rendered previews.",
    "You need a phone app; Obsidian ships native iOS and Android clients that open the same vault.",
    "You depend on community plugins; the directory covers queries, templates, drawing and much more, and tnotes has no plugin system.",
    "You want a hosted, end-to-end encrypted sync service with version history and shared vaults, and are fine paying for it.",
    "You use heading, block and alias links; tnotes documents only `[[Title]]`.",
  ],
  chooseTnotes: [
    "You live in a terminal or over SSH. tnotes is a single Rust binary with a mouse-capable TUI; Obsidian is an Electron app.",
    "Scripts and agents need a headless interface that works without a GUI process: `tnotes ls --json`, `search`, `cat`, `new`, `append`, `write` operate directly on files.",
    "You want nothing written next to your notes. tnotes keeps its config in `~/.config/tnotes/` and only creates a `.Trash/` per root; Obsidian adds `.obsidian/` inside the vault.",
    "You want the source: tnotes is MIT-licensed, Obsidian is closed.",
    "Your notes should survive a two-way edit: a CLI `write` while the TUI has unsaved changes yields a conflict copy, never a silent overwrite.",
  ],
  together: `Point a tnotes root at an existing vault. Both tools read the same \`.md\` files, and tnotes ignores dot-prefixed entries, so \`.obsidian/\` never appears in its tree.

\`\`\`sh
tnotes ~/Vaults/main            # adds the vault to roots in config.toml and opens it
tnotes ls --json --dir ~/Vaults/main | jq '.[] | {title, backlinks}'
\`\`\`

Or set it permanently in \`~/.config/tnotes/config.toml\`:

\`\`\`toml
roots = ["~/Vaults/main"]
\`\`\`

What carries over unchanged: \`[[Title]]\` links, \`#tags\` including nested \`#work/project\`, folders, checkboxes. Obsidian resolves links by filename; tnotes resolves by note title (case-insensitive) and then by file stem, so a plain \`[[Note]]\` link that resolves in Obsidian also resolves in tnotes through the stem. Obsidian's \`[[Note#heading]]\`, \`[[Note|alias]]\`, \`[[folder/Note]]\` and block references are not documented in tnotes.

Two things to know before editing. tnotes takes the note title from the first line and renames the file to a slug of it when that line changes, rewriting \`[[links]]\` in the other notes it manages. And tnotes has no documented frontmatter handling: a note that starts with a YAML block has \`---\` as its first line. Keep the Obsidian app open while you work in tnotes; both watch the folder and pick up each other's saves. See [/for/obsidian-users](/for/obsidian-users) and [/docs/sync](/docs/sync).`,
  faq: [
    {
      q: "Can tnotes open an Obsidian vault?",
      a: "Yes. A vault is a folder of Markdown files; run `tnotes ~/path/to/vault` or add it to `roots` in the config. `.obsidian/` is ignored because tnotes skips dot-prefixed entries. Links and tags use the same syntax in both.",
    },
    {
      q: "Does Obsidian have a command-line interface?",
      a: "Since the 1.12.7 installer, Obsidian CLI lets you run app commands such as `obsidian search query=...` or `obsidian daily:append content=...` from a shell, with `format=json` on some commands. It requires the desktop app to be running. Obsidian Headless (`ob`) is a separate npm package that syncs and publishes without the app but does not read or search notes. tnotes' CLI works on the files directly with no app process.",
    },
    {
      q: "Is Obsidian free and open source?",
      a: "Obsidian is free for personal and commercial use since February 2025, but the app is closed source; only side projects like JSON Canvas and Knap are MIT. Sync ($4/user/month billed annually) and Publish are paid add-ons. tnotes is MIT.",
    },
  ],
  verifiedOn: "2026-09-22",
  sources: [
    {
      label: "Obsidian Help: How Obsidian stores data",
      url: "https://help.obsidian.md/data-storage",
    },
    {
      label: "Obsidian Help: Internal links",
      url: "https://help.obsidian.md/links",
    },
    {
      label: "Obsidian Help: Obsidian CLI",
      url: "https://help.obsidian.md/cli",
    },
    {
      label: "Obsidian Help: Headless Sync",
      url: "https://help.obsidian.md/sync/headless",
    },
    {
      label: "Obsidian Help: Sync security and privacy",
      url: "https://help.obsidian.md/sync/security",
    },
    {
      label: "Obsidian pricing and license overview",
      url: "https://obsidian.md/pricing",
    },
    {
      label: "Obsidian license summary",
      url: "https://obsidian.md/license",
    },
    {
      label: "Obsidian Help: Settings (Vim key bindings)",
      url: "https://help.obsidian.md/settings",
    },
  ],
};
