import type { Tool } from "../types";

export const tool: Tool = {
  slug: "notion",
  name: "Notion",
  url: "https://www.notion.com",
  tagline:
    "Hosted workspace of pages and databases with real-time collaboration, a REST API and an official MCP server; content lives on Notion's servers, not in files.",
  category: "web app",
  platforms: "Web; desktop apps for macOS and Windows; iOS and Android",
  license: "Proprietary",
  pricing: "Free plan for individuals; Plus, Business and Enterprise per member per month",
  features: {
    storage: {
      v: "no",
      note: "cloud database; export to Markdown & CSV, HTML or PDF",
    },
    terminal: { v: "no" },
    wikilinks: {
      v: "partial",
      note: "typing [[ opens a page picker; the link is a page mention, not text",
    },
    backlinks: { v: "yes", note: "backlinks list above the page title, from @-mentions" },
    tags: {
      v: "partial",
      note: "Select / Multi-select database properties; no inline #tags",
    },
    cli: { v: "no", note: "REST API and hosted MCP server; no official CLI" },
    json: { v: "partial", note: "REST API (api.notion.com) speaks JSON; nothing local" },
    liveReload: { v: "no", note: "no files on disk; edits sync through the cloud" },
    vim: { v: "no" },
    mouse: { v: "yes", note: "block handles, drag and drop, slash menus" },
    sync: { v: "yes", note: "built in; per-minute server backups" },
    encryption: {
      v: "partial",
      note: "AES-256 at rest, TLS in transit; not end-to-end, staff can access for support",
    },
    mobile: { v: "yes", note: "iOS and Android" },
    plugins: {
      v: "partial",
      note: "integrations via the API and MCP; no in-app plugin system",
    },
    openSource: { v: "no", note: "SDKs are open source, the product is not" },
  },
  intro: `Notion is a hosted document and database product. A page is a tree of blocks stored on Notion's servers; a database is a table of pages with typed properties (Select, Multi-select, Date, Relation, Formula, up to 500 per database) that you can view as a table, board, calendar or timeline. Everything is collaborative by default: sharing, comments, permissions per page or teamspace, and audit logs on paid plans.

For programmatic access there is a REST API at \`api.notion.com\` with cursor pagination and JSON bodies, and since 2025 a hosted MCP server so Claude Code, Cursor or Codex can search, read and write a workspace after an OAuth grant. Enterprise workspaces can trigger a full export from an admin API. What Notion does not have is a local file: the desktop app is a wrapper over the same cloud data, and export is a zip of Markdown, CSV, HTML or PDF that you request and download.

tnotes sits on the other end. There is no server, no account and no collaboration; notes are \`.md\` files in a directory you own, and the CLI's JSON is a view of those files. Choosing between them is mostly choosing between a shared workspace with a data model and a folder of text.`,
  chooseTool: [
    "Several people edit the same pages at once and you need permissions, comments and mentions on top of the text.",
    "Databases with typed properties, relations, rollups and formulas are the actual work; a folder of Markdown cannot express them.",
    "You want an official, hosted integration surface (REST API, MCP, Slack/GitHub/Jira connections) maintained by the vendor.",
    "Mobile and web access from any device without setting up file sync.",
  ],
  chooseTnotes: [
    "Your notes must be readable and writable without a network, an account, or Notion's continued existence: plain `.md` files, `git` history, any editor.",
    "Agents and scripts work on a local folder: `tnotes ls --json`, `search`, `append` and `write` return in milliseconds with no token, rate limit or OAuth grant.",
    "You live in a terminal and want `[[links]]`, `#tags`, backlinks and search there, with vim or emacs keys.",
    "Encryption is under your control: FileVault, gocryptfs or an encrypted volume, not a vendor promise of encryption at rest.",
  ],
  together: `There is no live sync between Notion and tnotes. The workable path is a one-way export: Notion writes a zip of Markdown files, you unzip it into a tnotes root.

In Notion: \`Settings\` → \`General\` → \`Export all workspace content\`, format \`Markdown & CSV\`, subpages included. Large workspaces arrive by email link, up to 30 hours later. Per page, the \`•••\` → \`Export\` menu does the same for one subtree.

\`\`\`sh
mkdir -p ~/notes/notion && cd ~/notes/notion
unzip ~/Downloads/Export-*.zip
tnotes ~/notes/notion                 # add it as a root
tnotes ls --json | jq 'length'        # every exported page is a note
tnotes search "onboarding" --dir ~/notes/notion
\`\`\`

What to expect in the export:

- Each file is named \`<Page title> <32-hex page id>.md\`, and each subpage folder carries the same suffix. tnotes takes a note's title from its first line, so the list reads correctly as long as the exported file starts with the page title; the file names keep the suffix until renamed. Editing a title in tnotes renames the file to a slug of the new title and rewrites \`[[links]]\`, but the export's links are not wikilinks (next point), so rename in bulk with a script if you want clean stems.
- Links between pages are exported as Markdown links to the id-suffixed file names (\`[Roadmap](Roadmap%20a1b2….md)\`). tnotes indexes only \`[[Title]]\` links, so Notion's page graph does not appear in \`links\`/\`backlinks\` until you convert them.
- Databases become \`.csv\` files (one row per page) plus a Markdown file per row. tnotes ignores the CSV.
- Callout blocks are exported as HTML, not Markdown. Select and Multi-select values are database properties, not \`#tags\` in the text, so tnotes does not see them as tags.
- Attachments land in the same folders; tnotes only lists \`.md\`.

Treat the folder as a snapshot. If Notion stays the source of truth, re-export and replace; if tnotes does, stop editing in Notion.`,
  faq: [
    {
      q: "Can I export Notion to Markdown?",
      a: "Yes. Any page: `•••` → `Export` → `Markdown & CSV`, optionally with subpages. Whole workspace: `Settings` → `General` → `Export all workspace content` (desktop or web only; the download link expires after 7 days). Full-page databases export as CSV with one Markdown file per row; callouts export as HTML. Form views cannot be exported.",
    },
    {
      q: "Is there a Notion CLI?",
      a: "Not an official one. Notion exposes a REST API (bearer token, JSON, cursor pagination, 100 items per page) and a hosted MCP server that clients such as Claude Code connect to over OAuth. Community CLIs wrap the API. For a local folder of Markdown, `tnotes` gives `ls`, `search`, `cat`, `new`, `append` and `write` with `--json`, no token required.",
    },
    {
      q: "Is Notion end-to-end encrypted?",
      a: "No. Notion's security page states customer data is encrypted at rest with AES-256 and in transit with TLS 1.2+, hosted on AWS, and that employees may access data for troubleshooting or content recovery. There is no client-side key. If that matters, keep notes in files on an encrypted disk.",
    },
  ],
  verifiedOn: "2026-09-22",
  sources: [
    { label: "Notion Help: Export your content", url: "https://www.notion.com/help/export-your-content" },
    { label: "Notion Help: Links & backlinks", url: "https://www.notion.com/help/create-links-and-backlinks" },
    { label: "Notion Help: Security practices", url: "https://www.notion.com/help/security-and-privacy" },
    { label: "Notion API reference: Introduction, and MCP overview", url: "https://developers.notion.com/reference/intro" },
  ],
};
