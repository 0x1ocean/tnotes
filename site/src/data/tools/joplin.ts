import type { Tool } from "../types";

export const tool: Tool = {
  slug: "joplin",
  name: "Joplin",
  url: "https://joplinapp.org",
  tagline:
    "Open-source Evernote-style notes and to-dos with end-to-end encrypted sync, desktop, mobile and terminal clients.",
  category: "desktop app",
  platforms:
    "Windows, macOS, Linux, Android, iOS; terminal app on macOS, Linux and WSL (Node.js)",
  license: "AGPL-3.0-or-later (server package licensed separately)",
  pricing:
    "Free. Joplin Cloud sync from 2.40 €/month (Basic, billed yearly) to 7.99 €/month (Pro 100 GB); self-hosted Joplin Server also available.",
  features: {
    storage: {
      v: "no",
      note: "notes live in a SQLite database in the profile directory; sync targets hold Joplin's own item format, not editable files",
    },
    terminal: {
      v: "yes",
      note: "`joplin` terminal app: three-pane TUI with Vim-like normal and `:` command modes",
    },
    wikilinks: {
      v: "no",
      note: "links are `[title](:/noteid)`; a community Wikilinks plugin adds [[ ]]",
    },
    backlinks: {
      v: "partial",
      note: "not built in; community plugins (Automatic Backlinks, Backlinks Navigator, Note Link System)",
    },
    tags: {
      v: "yes",
      note: "tags are metadata attached to notes (`tag add`), not `#text`",
    },
    cli: {
      v: "yes",
      note: "shell mode: `joplin mknote`, `ls`, `cat`, `set`, `export`, `sync`, `tag`…",
    },
    json: {
      v: "partial",
      note: "`ls -f json`; Data API (REST on port 41184) speaks JSON when the app runs",
    },
    liveReload: {
      v: "no",
      note: "the database is the source of truth; edits go through the app, CLI or API",
    },
    vim: {
      v: "yes",
      note: "desktop: Keyboard Mode → Vim; terminal app is Vim-like and edits in `$EDITOR`",
    },
    mouse: { v: "yes", note: "desktop and mobile apps" },
    sync: {
      v: "yes",
      note: "Joplin Cloud, Nextcloud, WebDAV, Dropbox, OneDrive, S3, local folder; `joplin sync` from cron",
    },
    encryption: {
      v: "yes",
      note: "end-to-end encryption of synced data on every client (master key + password)",
    },
    mobile: { v: "yes", note: "Android, iOS" },
    plugins: {
      v: "yes",
      note: "desktop and Android; iOS limited to recommended plugins",
    },
    openSource: { v: "yes", note: "AGPL-3.0-or-later, TypeScript" },
  },
  intro: `Joplin is the open-source answer to Evernote: notebooks, notes and to-dos with attachments, tags and full-text search, imported from ENEX files with their metadata intact. It is offline first, with every client holding a full copy of the data in a local SQLite database and reconciling through a sync target. Sync targets are deliberately pluggable and vendor-neutral: Nextcloud, any WebDAV server, Dropbox, OneDrive, S3, a local folder, or the paid Joplin Cloud, all optionally end-to-end encrypted with a master key you hold.

Notes are Markdown, rendered by a CommonMark pipeline with optional extensions (KaTeX, Mermaid, footnotes, ABC notation). Links between notes use \`[title](:/noteid)\`, an id-based form that survives retitling; \`[[wikilinks]]\` and backlinks come from community plugins, not the core. Tags are objects attached to notes rather than words in the text. The desktop editor has a Keyboard Mode setting with Default, Emacs and Vim; the terminal app is a Node.js three-pane TUI with Vim-like normal mode, a \`:\` command line, and external-editor editing, and its commands (\`mknote\`, \`ls\`, \`cat\`, \`set\`, \`export\`, \`sync\`) also run directly from a shell.

What Joplin is not is a folder of files. The FAQ is explicit that sync directories are "basically just a database" and not meant to be user-editable. To get Markdown files out you export (\`--format md\` or \`md_frontmatter\`); to get them in you import. That is the line that separates Joplin from tnotes.`,
  chooseTool: [
    "You need phones and tablets as first-class clients with the same encrypted data, including attachments and to-dos with alarms.",
    "You want end-to-end encryption built into sync rather than an encrypted filesystem, and a choice of sync backends including your own Nextcloud.",
    "You are migrating from Evernote; the ENEX importer keeps notebooks, tags, resources and metadata, and OneNote import is supported as well.",
    "Attachments, images, PDFs and a rendered preview matter as much as the text.",
    "You want a plugin ecosystem on desktop and a REST Data API for integrations.",
  ],
  chooseTnotes: [
    "Your notes must be plain `.md` files on disk that git, grep, rsync and other editors can touch directly. Joplin keeps them in SQLite.",
    "You want the terminal to be the primary editor, with live Markdown highlighting, `[[link]]` completion and a backlinks overlay built in.",
    "Agents and scripts should read and write notes without a running app or an API token: `tnotes ls --json`, `tnotes write --stdin`, and the TUI reloads.",
    "You want `[[Title]]` links and `#tags` in the text itself, portable to Obsidian or any Markdown tool.",
    "You want a single binary with no Node.js runtime, database or profile directory to manage.",
  ],
  together: `There is no live overlap: Joplin's notes are rows in SQLite and tnotes reads files, so the two never see the same data at the same time. The bridge is export and import.

\`\`\`sh
# Joplin → Markdown files → a tnotes root
joplin export --format md ~/joplin-md      # Markdown files, one directory per export
tnotes ~/joplin-md                          # open the export as a tnotes root

# tnotes → Joplin
joplin import --format md ~/Documents/notes "Imported"
\`\`\`

Use \`--format md_frontmatter\` if you want Joplin's metadata preserved as YAML; tnotes will then show \`---\` as the first line of each note, because its title is the first line and it documents no frontmatter handling. Joplin's internal links (\`[title](:/id)\`) have no meaning outside Joplin; rewrite the ones you care about as \`[[Title]]\`. Joplin tags are not \`#tags\` in the text, so add \`#tag\` words for anything you want to filter on in tnotes.

Going the other way, \`joplin import --format md\` takes a file or a directory and maps the folder structure onto notebooks. The realistic split is to keep Joplin for attachment-heavy or mobile-first notebooks and tnotes for a text vault you script against. See [/docs/cli](/docs/cli) and [/guides/plain-text-notes-sync](/guides/plain-text-notes-sync).`,
  faq: [
    {
      q: "Does Joplin store notes as Markdown files?",
      a: "No. Notes are Markdown text, but they are stored in a SQLite database in the profile directory, and the files Joplin writes to a sync target are its own item format. Joplin's FAQ says a sync directory is 'basically just a database'. Use `joplin export --format md` to get files; tnotes works on files directly.",
    },
    {
      q: "Can I use Joplin from the terminal?",
      a: "Yes. `npm install -g joplin` gives a terminal app with a three-pane TUI, Vim-like keys and a `:` command line. The same commands work in shell mode (`joplin mknote \"Title\"`, `joplin ls -f json`, `joplin sync`) for scripts and cron. Editing a note opens your external editor. tnotes edits inline in its TUI and also has a headless CLI.",
    },
    {
      q: "Does Joplin support wikilinks and backlinks?",
      a: "Not in the core. Joplin links are `[title](:/noteid)`, created by dragging a note or 'Copy Markdown link'. Community plugins add `[[wikilinks]]` (Wikilinks) and backlink panels (Automatic Backlinks to note, Backlinks Navigator, Note Link System). tnotes has both built in and exposes `links` and `backlinks` in `ls --json`.",
    },
  ],
  verifiedOn: "2026-09-22",
  sources: [
    {
      label: "Joplin Terminal Application (shell mode, sync, commands)",
      url: "https://joplinapp.org/help/apps/terminal",
    },
    {
      label: "Joplin FAQ (SQLite database, sync directory)",
      url: "https://joplinapp.org/help/faq",
    },
    {
      label: "Joplin Markdown Guide (note links)",
      url: "https://joplinapp.org/help/apps/markdown",
    },
    {
      label: "Joplin Data API",
      url: "https://joplinapp.org/help/api/references/rest_api",
    },
    {
      label: "Joplin End-To-End Encryption",
      url: "https://joplinapp.org/help/apps/sync/e2ee",
    },
    {
      label: "Joplin plugins repository (Wikilinks, backlink plugins)",
      url: "https://github.com/joplin/plugins",
    },
    { label: "Joplin Cloud plans", url: "https://joplinapp.org/plans/" },
    {
      label: "Joplin LICENSE (AGPL-3.0-or-later)",
      url: "https://github.com/laurent22/joplin/blob/dev/LICENSE",
    },
  ],
};
