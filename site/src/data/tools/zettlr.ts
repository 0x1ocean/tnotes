import type { Tool } from "../types";

export const tool: Tool = {
  slug: "zettlr",
  name: "Zettlr",
  url: "https://www.zettlr.com/",
  tagline:
    "Electron Markdown editor for academic writing: citations, Pandoc export and a Zettelkasten layer on plain files.",
  category: "desktop app",
  platforms: "macOS, Windows, Linux (Electron; x64 and ARM builds)",
  license: "GPL-3.0",
  pricing: "Free, donation-funded",
  features: {
    storage: { v: "yes", note: "plain .md files in folders opened as workspaces" },
    terminal: { v: "no", note: "GUI only; its command-line switches configure the app, they do not replace it" },
    wikilinks: { v: "yes", note: "[[ID]] or [[filename]], optional [[target|title]]; autocomplete after [[" },
    backlinks: { v: "yes", note: "inbound links listed in the Related Files sidebar tab" },
    tags: { v: "yes", note: "#tag in text or YAML keywords; tag cloud and tag manager" },
    cli: { v: "no", note: "no headless subcommands" },
    json: { v: "no" },
    liveReload: { v: "yes", note: "\"Always load remote changes\" setting; off by default it asks first" },
    vim: { v: "yes", note: "input mode: Normal, Vim or Emacs" },
    mouse: { v: "yes" },
    sync: { v: "partial", note: "none built in; files sync with any file sync" },
    encryption: { v: "no", note: "not built in; encrypt the folder at the OS level" },
    mobile: { v: "no" },
    plugins: { v: "no", note: "no plugin API; custom CSS, snippets, custom export commands" },
    openSource: { v: "yes", note: "GPL-3.0, TypeScript on Electron" },
  },
  intro: `Zettlr calls itself a publication workbench, and the feature list follows from that: a bundled Pandoc, export profiles for LaTeX and Word templates, CSL-styled citations from a Zotero or JabRef export, LanguageTool integration, a readability mode. The Markdown editor is CodeMirror 6 with a preview-style rendering mode, a table editor and a linter for style issues. Version 4.8.0 shipped on 18 September 2026 and the project is still actively developed.

The Zettelkasten layer sits on top of ordinary files. Zettlr generates a timestamp ID (\`YYYYMMDDHHMMSS\` by default) and recognises it in either the filename or the file body; \`[[…]]\` links can target that ID or the filename, and \`Cmd/Ctrl+L\` inserts a fresh ID at the cursor. There is no separate backlinks pane: the Related Files tab in the sidebar lists inbound, outbound and bidirectional links above notes that merely share a tag, and a graph view draws the same relations.

Zettlr is a desktop application. It has command-line switches such as \`--data-dir\` and \`--clear-cache\`, but nothing that reads or writes notes without opening the window. If your workflow is a terminal, a script or an agent, Zettlr has no surface for it.`,
  chooseTool: [
    "You export: Pandoc is bundled, and there are profiles for PDF via LaTeX, Word templates and Textbundle.",
    "You cite: CSL JSON or BibTeX databases, CSL styles, and citation autocomplete in the editor.",
    "You want a rendered editing view with a table editor, spellcheck and LanguageTool, not raw Markdown in a terminal.",
    "You link by stable ID so that notes can be retitled and renamed without breaking links.",
  ],
  chooseTnotes: [
    "You live in a terminal or over SSH: tnotes is a TUI plus `tnotes ls | search | cat | new | append | write`, all with `--json`.",
    "Scripts or AI agents need to read and write the same vault; Zettlr has no headless mode.",
    "You want a small Rust binary with no Electron runtime.",
    "You link by title and want links rewritten automatically when a note is renamed.",
  ],
  together: `Both tools read a folder of \`.md\` files, so you can point tnotes at a Zettlr workspace:

\`\`\`sh
tnotes ~/Zettelkasten          # adds the folder to your roots and opens it
tnotes search "luhmann" --json  # read-only: ls, search and cat never touch a file
\`\`\`

What carries over and what breaks:

- \`[[20240101120000]]\` resolves in tnotes only when \`20240101120000.md\` is the file name: tnotes matches a link against titles first, then against file stems. IDs that live only inside the body are not link targets in tnotes.
- \`[[20240101120000|Reading notes]]\` does not resolve at all. tnotes has no \`|\` alias syntax; the whole string between the brackets is the target.
- tnotes takes a note's title from its first non-empty line and, when it saves, renames the file to a slug of that title. Editing \`20240101120000.md\` that starts with \`# Reading notes\` in the tnotes TUI, or with \`tnotes append\` / \`tnotes write\`, produces \`reading-notes.md\`. Zettlr still finds the note if the ID also appears in the body; if the ID was only the file name, Zettlr's links to it are dead.
- tnotes rewrites \`[[Old title]]\` to \`[[New title]]\` on rename. It does not know about ID links, so it leaves them alone.
- \`#tags\` written in the body work in both. Keywords in YAML front matter are Zettlr-only; tnotes shows the front matter as text.

Practical split: keep the ID in the body (Zettlr's \`Cmd/Ctrl+L\` puts it there), link by title where you can, and use tnotes for search and reading from the terminal. If you edit in both, expect files to be renamed by tnotes.`,
  faq: [
    {
      q: "Does Zettlr have backlinks?",
      a: "Yes, under a different name. The Related Files tab in the right sidebar lists every note that links to the current one (inbound), every note it links to (outbound), mutual links, and then notes that share a tag. Bidirectional links sort to the top.",
    },
    {
      q: "Does Zettlr use IDs or titles for wiki links?",
      a: "Either. Zettlr recognises an ID (a timestamp by default) in the file name or the body, and a preference decides whether autocomplete inserts the ID or the file name as the link target. Links can carry a title after a pipe, and another preference sets which side of the pipe is the target.",
    },
    {
      q: "Can I use Zettlr from the terminal?",
      a: "No. Zettlr is an Electron app; its command-line switches (`--data-dir`, `--clear-cache`, `--launch-minimized`) only affect how the GUI starts. For a terminal or scripted workflow on the same Markdown files, use a tool with a headless CLI such as tnotes or nb.",
    },
  ],
  verifiedOn: "2026-09-22",
  sources: [
    { label: "Zettlr README (features, platforms, command-line switches, license)", url: "https://github.com/Zettlr/Zettlr" },
    { label: "Zettlr docs: settings reference (input mode, remote changes, Zettelkasten links)", url: "https://docs.zettlr.com/en/reference/settings.html" },
    { label: "Zettlr docs: the Zettelkasten method (IDs, links, tags)", url: "https://docs.zettlr.com/en/pkms/zkn-method.html" },
    { label: "Zettlr docs: Related Files sidebar (inbound and outbound links)", url: "https://docs.zettlr.com/en/sidebar/related-files.html" },
  ],
};
