import type { Tool } from "../types";

export const tool: Tool = {
  slug: "logseq",
  name: "Logseq",
  url: "https://logseq.com",
  tagline:
    "Outliner-style knowledge base with daily journals, block references and a plugin API; file graphs are Markdown or Org files, the newer DB graphs are SQLite.",
  category: "desktop app",
  platforms: "Linux, macOS, Windows (Electron); iOS, Android; web app for DB graphs",
  license: "AGPL-3.0",
  pricing: "Free. Logseq Sync (beta) requires an Open Collective contribution ($5 or $15/month).",
  features: {
    storage: {
      v: "partial",
      note: "file graphs: .md/.org in pages/ and journals/; DB graphs (beta): SQLite",
    },
    terminal: { v: "no", note: "Electron desktop app; the new Node CLI targets DB graphs only" },
    wikilinks: { v: "yes", note: "[[page]] with autocompletion; ((uuid)) block refs" },
    backlinks: { v: "yes", note: "Linked References and Unlinked References per page" },
    tags: { v: "yes", note: "#tag is a page reference; tags:: page property" },
    cli: {
      v: "partial",
      note: "logseq CLI (Node) for DB graphs: list, query, sync; nothing for file graphs",
    },
    json: { v: "partial", note: "CLI --output json, DB graphs only" },
    liveReload: {
      v: "partial",
      note: "desktop watches the graph; a conflicting external edit prompts to reload",
    },
    vim: {
      v: "partial",
      note: "custom :shortcuts in config.edn; community vim-shortcuts plugin",
    },
    mouse: { v: "yes", note: "GUI: drag blocks, sidebar, whiteboards" },
    sync: {
      v: "partial",
      note: "any file sync for file graphs; Logseq Sync (paid beta, E2EE); RTC alpha for DB graphs",
    },
    encryption: {
      v: "partial",
      note: "Logseq Sync encrypts with age; local graph is plain files",
    },
    mobile: { v: "yes", note: "iOS and Android; DB-graph mobile app in alpha" },
    plugins: { v: "yes", note: "plugin API, marketplace, themes" },
    openSource: { v: "yes", note: "AGPL-3.0, ClojureScript" },
  },
  intro: `Logseq is an outliner: every line is a block with a bullet, blocks nest, and any block can be referenced from anywhere by its \`((uuid))\`. Pages are Markdown or Org files in a graph directory (\`pages/\`, \`journals/\`, \`assets/\`, \`logseq/config.edn\`), and the app rebuilds a Datascript database from them on load. Linked References under each page, \`#tag\` as a page reference, \`key:: value\` properties on any block, and Datalog queries are what people stay for.

The project is mid-transition. The "DB version" replaces file graphs with a SQLite database, adds a real-time collaboration sync (RTC, alpha), a new mobile app, and a Node CLI (\`logseq graph list\`, \`logseq list page --output json\`, \`logseq sync start\`). The README calls the DB version beta and warns that data loss is possible. File graphs still work in the stable release, and the file-graph documentation at docs.logseq.com is the part that matters for a plain-text workflow.

tnotes is the opposite shape: no outliner, no block identities, no queries. A note is a Markdown file whose first line is its title; links resolve by title; the TUI and the \`tnotes\` CLI are the whole interface. If your notes are Logseq-shaped (bullets everywhere, block refs, properties) tnotes will show them but not understand them. If they are documents with headings and \`[[links]]\`, both tools read the same files.`,
  chooseTool: [
    "You think in outlines and block references, not documents. Nothing in tnotes addresses a block.",
    "Daily journals, `TODO`/`DONE` task states, scheduled and deadline dates, and Datalog queries over properties are the workflow.",
    "You want a plugin ecosystem, PDF annotation, whiteboards, and mobile apps that read the same graph.",
    "Logseq Sync's end-to-end encryption (file names included) fits your threat model better than syncing a folder yourself.",
  ],
  chooseTnotes: [
    "You want notes that any Markdown tool reads without an outliner's `- ` on every line and `id::` lines under referenced blocks.",
    "You need a headless interface today: `tnotes ls --json`, `search`, `cat`, `new`, `append`, `write` work on a plain folder, not a DB graph, and a running TUI picks the changes up.",
    "You work over SSH or in a terminal multiplexer and want the notes app in the same window as the shell.",
    "A single static binary (Homebrew, cargo, or a tarball) instead of an Electron app; no beta storage format to migrate through.",
  ],
  together: `A Logseq **file graph** is a directory of Markdown, so it can be a tnotes root. \`pages/\` and \`journals/\` show up as folders; \`assets/\` has no \`.md\` files and is empty in the tree.

\`\`\`sh
tnotes ~/graphs/work                  # add the graph as a root
tnotes ls --folder work/journals --json | jq '.[0]'
tnotes search "retro" --dir ~/graphs/work
\`\`\`

What tnotes does with Logseq's conventions:

- Block properties (\`key:: value\`), the \`id:: <uuid>\` line Logseq writes under any block that is referenced, \`collapsed:: true\`, and \`((uuid))\` block references are plain text in tnotes. They are highlighted as list items and left alone.
- Logseq pages rarely start with a heading; the first line is usually \`- \` or a property. tnotes uses the first line as the title and **renames the file when that line changes**. Do not edit the first line of a Logseq page in tnotes unless you want the file renamed. Reading, searching and appending are safe.
- \`[[Page Title]]\` resolves in both tools when the file stem matches the title. Namespaced pages (\`[[term/backlink]]\` stored as \`term___backlink.md\`) and journal links (\`[[Sep 22nd, 2026]]\` for \`2026_09_22.md\`) resolve in Logseq only.
- \`#tag\` is a page reference in Logseq and a filter in tnotes; nested \`#work/project\` is a namespace in Logseq and a nested tag in tnotes. Both read the same characters.
- Logseq's \`logseq/\` directory is not dot-prefixed, so any \`.md\` it contains (it keeps backups under \`logseq/bak/\`) is listed as a note. \`.git\`, \`.stfolder\` and other dot-entries are ignored.
- Logseq detects external writes; when it has unsaved in-memory changes to the same file it prompts about the file being modified on disk. Save in Logseq before appending with \`tnotes append\`, or keep one writer per file.

DB graphs are SQLite and have no \`.md\` files on disk; there is nothing for tnotes to open. Use \`logseq graph export --type edn\` for those, or keep a file graph.`,
  faq: [
    {
      q: "Can I open a Logseq graph in a terminal?",
      a: "Logseq has no TUI. Its Node CLI (`logseq`) works only against DB graphs (SQLite). For a file graph, point a terminal Markdown tool at the directory: `tnotes ~/graphs/work` lists `pages/` and `journals/` as folders and searches their text. Outliner semantics (block refs, properties, queries) are not interpreted.",
    },
    {
      q: "Does Logseq store notes as Markdown files?",
      a: "File graphs do: one `.md` (or `.org`) per page under `pages/`, one per day under `journals/`, plus `logseq/config.edn`. Every block is a `- ` bullet and Logseq adds `id::` and other `key:: value` property lines as needed. DB graphs, the beta default in the DB version, store the graph in SQLite; export to EDN or SQLite with the CLI.",
    },
    {
      q: "Is Logseq Sync end-to-end encrypted?",
      a: "Yes. Per the Logseq docs, each synced graph is encrypted with a password using age; file contents and file names are encrypted before they reach AWS. It is a beta feature available to active Open Collective contributors, and Logseq advises against combining it with iCloud, Syncthing or Dropbox on the same graph.",
    },
  ],
  verifiedOn: "2026-09-22",
  sources: [
    { label: "logseq/logseq README (DB version, plugin API, CLI)", url: "https://github.com/logseq/logseq" },
    { label: "Logseq CLI (Node) docs", url: "https://github.com/logseq/logseq/blob/master/docs/cli/logseq-cli.md" },
    { label: "Logseq docs: Logseq Sync", url: "https://github.com/logseq/docs/blob/master/pages/Logseq%20Sync.md" },
    { label: "Logseq docs: Built-in Properties (id, collapsed)", url: "https://github.com/logseq/docs/blob/master/pages/Built-in%20Properties.md" },
  ],
};
