import type { Tool } from "../types";

export const tool: Tool = {
  slug: "neorg",
  name: "Neorg",
  url: "https://github.com/nvim-neorg/neorg",
  tagline:
    "Neovim plugin for structured notes, tasks and journals in its own .norg format, parsed by tree-sitter.",
  category: "editor plugin",
  platforms: "Neovim 0.10+ on Linux, macOS, Windows; needs luarocks and a tree-sitter build",
  license: "GPL-3.0",
  pricing: "Free",
  features: {
    storage: { v: "yes", note: "plain .norg files in workspaces managed by core.dirman; not Markdown" },
    terminal: { v: "yes", note: "it is Neovim" },
    wikilinks: { v: "partial", note: "{:file:} and {:file:* Heading} links, {* Heading} in-file; no [[Title]] syntax" },
    backlinks: { v: "no", note: "planned as part of a Zettelkasten library on the roadmap" },
    tags: { v: "partial", note: "categories in @document.meta; no inline #tag index" },
    cli: { v: "no", note: "commands are :Neorg … inside Neovim" },
    json: { v: "no" },
    liveReload: { v: "partial", note: "Neovim's own autoread / checktime" },
    vim: { v: "yes", note: "native" },
    mouse: { v: "yes", note: "Neovim's mouse support" },
    sync: { v: "partial", note: "files; git or any file sync" },
    encryption: { v: "no" },
    mobile: { v: "no", note: "a mobile app is on the roadmap, not built" },
    plugins: { v: "yes", note: "everything is a module; core.*, external.* community modules" },
    openSource: { v: "yes", note: "GPL-3.0, Lua" },
  },
  intro: `Neorg is not a Markdown tool and does not try to be one. It defines its own file format, \`.norg\`, with a tree-sitter grammar, and builds every feature on that: headings that nest by star count, TODO items with a state cycle, a journal module, a table of contents module, a presenter, a code-block tangler. The README calls the project young and warns of occasional breaking changes; version 9.0.0 changed the installation path to luarocks, and 9.6.4 (April 2026) is the latest tag.

Links are Norg objects, not titles. \`{* Heading}\` points at a heading in the current file, \`{:notes/index:* Heading}\` at a heading in another file of the workspace, \`{# anything}\` at any object with that name; \`core.esupports.hop\` follows them. There is no title-based link and no backlinks index. The roadmap lists a Zettelkasten backend, to be written in Rust, whose main job would be tracking backlinks; it is unchecked.

Getting out is a Neovim command. \`:Neorg export to-file out.md\` converts the current buffer through \`core.export.markdown\`, and \`:Neorg export directory ~/notes markdown\` converts a workspace. Pandoc does not read \`.norg\`; the roadmap marks the Pandoc integration as stalled.`,
  chooseTool: [
    "You want Neovim to be the entire environment: Norg files, journal, TODO states, calendar and presenter without leaving the editor.",
    "You prefer a format designed for a parser: Norg's grammar is stricter and more regular than Markdown, and tree-sitter powers folding, concealing and text objects.",
    "You extend by writing Lua modules: Neorg's module system is the product, and community `external.*` modules plug into it.",
    "You are fine with a young project and pinning versions.",
  ],
  chooseTnotes: [
    "You want Markdown, readable by every other tool, phone app and language model.",
    "You want `[[Title]]` links, completion after `[[`, backlinks and rename rewriting today, not on a roadmap.",
    "You want a CLI with `--json` so scripts and agents can read and write the vault.",
    "You want vim keys without owning a Neovim configuration: tnotes ships with an edtui vim mode and installs as one binary.",
  ],
  together: `There is no overlap on disk: Neorg writes \`.norg\`, tnotes lists only \`.md\`, and Pandoc has no Norg reader. The one bridge is Neorg's exporter, which is one-way. Inside Neovim, export a workspace into a folder under a tnotes root:

\`\`\`
:Neorg export directory ~/norg markdown ~/notes/from-norg
\`\`\`

Then open the root:

\`\`\`sh
tnotes ~/notes
\`\`\`

The result is a snapshot. tnotes indexes only \`[[…]]\` links, so whatever the exporter emits for \`{:file:}\` links is text or a Markdown link to tnotes, not an edge in \`links\` / \`backlinks\`; \`@document.meta\` categories are not \`#tags\`. Edits in tnotes do not go back to \`.norg\`.

If you want both, split by kind rather than convert: Norg for the journal and task workflow inside Neovim, a tnotes root for reference notes that scripts and agents read with \`tnotes ls --json\`. Both are plain text in one git repository, and tnotes ignores the \`.norg\` files next to its Markdown.`,
  faq: [
    {
      q: "Does Neorg support backlinks?",
      a: "No. Links are one-directional Norg objects (`{* Heading}`, `{:file:* Heading}`). A Zettelkasten library that would track backlinks is listed on the roadmap as unstarted work.",
    },
    {
      q: "Can I convert Neorg files to Markdown?",
      a: "Yes, from inside Neovim: `:Neorg export to-file note.md` converts the current buffer through `core.export.markdown`, and `:Neorg export directory <dir> markdown [out-dir]` converts a whole workspace. Pandoc cannot read `.norg`; the roadmap marks a Pandoc parser as stalled.",
    },
    {
      q: "Does Neorg work with Markdown or Org files?",
      a: "No. Neorg only understands `.norg`. The roadmap mentions upgraders from `.org` to `.norg` as a goal, and Markdown export exists, but there is no import path and no Markdown mode.",
    },
  ],
  verifiedOn: "2026-09-22",
  sources: [
    { label: "Neorg README (Neovim 0.10+, luarocks install, GPL-3.0, 9.0.0 breaking changes)", url: "https://github.com/nvim-neorg/neorg" },
    { label: "Neorg roadmap (Zettelkasten/backlinks, Pandoc, mobile app: unchecked)", url: "https://github.com/nvim-neorg/neorg/blob/main/ROADMAP.md" },
    { label: "Neorg cheatsheet (link syntax)", url: "https://github.com/nvim-neorg/neorg/blob/main/doc/cheatsheet.norg" },
    { label: "Neorg wiki: core.export (:Neorg export to-file / directory)", url: "https://github.com/nvim-neorg/neorg/wiki/Exporting-Files" },
  ],
};
