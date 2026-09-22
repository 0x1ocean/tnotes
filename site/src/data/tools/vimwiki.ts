import type { Tool } from "../types";

export const tool: Tool = {
  slug: "vimwiki",
  name: "Vimwiki",
  url: "https://github.com/vimwiki/vimwiki",
  tagline:
    "A personal wiki inside Vim: linked text files with their own syntax, a diary, todo lists and HTML export, for people who already live in Vim.",
  category: "editor plugin",
  platforms: "Anywhere Vim 7.3+ or Neovim runs (Linux, macOS, Windows, BSD)",
  license: "MIT",
  pricing: "Free",
  features: {
    storage: {
      v: "yes",
      note: ".wiki files by default; .md with syntax = 'markdown', ext = '.md'",
    },
    terminal: { v: "yes", note: "it is Vim" },
    wikilinks: { v: "yes", note: "[[link]], [[link|description]], [[dir/page]], [[page#anchor]]" },
    backlinks: { v: "yes", note: ":VimwikiBacklinks searches the wiki for links to the current page" },
    tags: {
      v: "partial",
      note: ":tag: syntax, not #tag; needs a .tags metadata file (:VimwikiRebuildTags)",
    },
    cli: { v: "no", note: "Vim commands only; scriptable with vim -c or a headless nvim" },
    json: { v: "no" },
    liveReload: { v: "no", note: "Vim's own autoread / :checktime" },
    vim: { v: "yes" },
    mouse: { v: "partial", note: "whatever Vim gives you with set mouse=a" },
    sync: { v: "partial", note: "plain files: git, Syncthing, anything" },
    encryption: { v: "partial", note: "encrypt the directory yourself" },
    mobile: { v: "no", note: "files sync to any mobile editor" },
    plugins: { v: "partial", note: "no plugin API; it is itself a Vim plugin, extend with Vimscript" },
    openSource: { v: "yes", note: "MIT, Vimscript" },
  },
  intro: `Vimwiki has been the Vim answer to a personal wiki since 2008. You register one or more wikis in \`g:vimwiki_list\`, press \`<Leader>ww\` to open the index, put the cursor on a word and press Enter to turn it into a \`[[link]]\`, press Enter again to create and open the page, Backspace to come back. Pages are plain text with one of three syntaxes (its own, Markdown, MediaWiki); the native syntax gets the full feature set including \`:Vimwiki2HTML\` export, Markdown is second-best supported and has no built-in HTML converter.

Beyond links it has a diary (\`<Leader>w<Leader>w\` for today), checkbox lists with partial-completion states (\`[.]\`, \`[o]\`, \`[X]\`), tables that align themselves, folding, \`:VimwikiBacklinks\`, \`:VimwikiRenameFile\` that rewrites links, and \`:tag-one:tag-two:\` tags with omni-completion and \`:VimwikiSearchTags\`. Development is slow and maintainers few, which the README says openly; the plugin is stable rather than moving.

tnotes covers the link-and-tag core with a different interface: a two-pane TUI with mouse support, \`#tags\` parsed from the text, link completion after \`[[\`, backlinks in an overlay, and a CLI that returns JSON. It edits with edtui's vim keys, which is a subset of Vim, not Vim. Vimwiki users who try it usually keep both on the same directory.`,
  chooseTool: [
    "You want the full Vim editing model, your own mappings, and every Vim plugin you already use; tnotes' vim keys are edtui's subset (insert, normal, `u`, `ctrl+r`, `.`, `/`), not Vim.",
    "Native-syntax features matter: HTML export, self-aligning tables, `[.]`/`[o]` partial checkboxes, header-level folding.",
    "A diary (`<Leader>w<Leader>w` for today, `<Leader>w<Leader>i` to regenerate the index) and tag search across the wiki, all from Vim's command line.",
    "You need it on Windows, BSD, or a server where you can install a Vim plugin but not a new binary.",
  ],
  chooseTnotes: [
    "You want a headless interface for scripts and agents: `tnotes search --json`, `cat`, `new`, `append`, `write` have no Vimwiki counterpart.",
    "You prefer a sidebar, tabs, mouse clicks and right-click menus over `:VimwikiUISelect` and the location list.",
    "You want `#tags` in the text (including nested `#work/project`) and `[[Title]]` links resolved by title, without a `.tags` index to rebuild.",
    "Live reload when another device or a script changes a file, with conflict copies instead of Vim's swap-file prompt.",
  ],
  together: `Both tools edit plain Markdown, so they can share a directory. Configure a Markdown wiki with the \`.md\` extension and restrict Vimwiki to registered paths so it does not claim every Markdown file on the system:

\`\`\`vim
let g:vimwiki_list = [{'path': '~/Documents/notes/', 'syntax': 'markdown', 'ext': '.md'}]
let g:vimwiki_global_ext = 0
\`\`\`

\`\`\`sh
tnotes ~/Documents/notes          # same directory as a tnotes root
vim -c VimwikiIndex               # or <Leader>ww inside Vim
\`\`\`

How the two overlap on the same files:

- \`[[link]]\` is the same characters in both. Vimwiki resolves \`[[Weekly plan]]\` to the file \`Weekly plan.md\` relative to the current page; tnotes resolves by note title (first line, case-insensitive), then by file stem, and names new files as a slug of the title (\`# Weekly plan\` becomes \`weekly-plan.md\`). Links that both tools follow are the ones written as the file stem: \`[[weekly-plan]]\`. Links by title work in tnotes only, and following one in Vimwiki creates a second file.
- \`[[link|description]]\`: Vimwiki follows \`link\` and shows \`description\`. tnotes documents \`[[Note title]]\` only; whether it strips the \`|description\` part when resolving is not documented, so treat aliased links as Vimwiki-only until you have tested it.
- Renames: \`:VimwikiRenameFile\` rewrites links across the wiki; tnotes rewrites \`[[old title]]\` when a note's first line changes. Each tool's rename is visible to the other as ordinary file changes.
- Tags: Vimwiki's \`:tag-one:tag-two:\` and tnotes' \`#tag\` do not see each other. The \`.tags\` metadata file Vimwiki writes is dot-prefixed, so tnotes ignores it.
- \`diary/\` shows up as a folder in tnotes. tnotes keeps trashed notes under \`.Trash/\` in the root; Vimwiki's search and backlink commands may still scan that directory.
- tnotes reloads a clean tab when Vim writes the file; Vim needs \`set autoread\` or \`:e\` to pick up a tnotes write or a \`tnotes append\`. Vim's swap file will prompt if both edit the same note at once.`,
  faq: [
    {
      q: "Can Vimwiki use Markdown files?",
      a: "Yes: set `'syntax': 'markdown', 'ext': '.md'` on the wiki in `g:vimwiki_list`. `[[wikilinks]]`, the diary, lists and tags work; `:Vimwiki2HTML` does not, since only the native syntax ships a converter (see `:h vimwiki-option-custom_wiki2html`). Set `g:vimwiki_global_ext = 0` or Vimwiki treats every `.md` on the machine as a wiki page.",
    },
    {
      q: "Does Vimwiki have backlinks?",
      a: "`:VimwikiBacklinks` (also `:VWB`) searches all files of the current wiki for links to the current page and lists them; it does not maintain an index. tnotes keeps backlinks live in an overlay (`alt+enter` on a line without a link) and in the `backlinks` field of `tnotes ls --json`.",
    },
    {
      q: "Is there a CLI for Vimwiki notes?",
      a: "No. Vimwiki is Vim commands and mappings; the closest is `vim -c` or a headless Neovim running Vimscript. Because the notes are plain files, a terminal tool that understands `[[links]]` can index the same directory: `tnotes ls --json --dir ~/vimwiki` returns id, title, tags, links and backlinks per note.",
    },
  ],
  verifiedOn: "2026-09-22",
  sources: [
    { label: "vimwiki README (syntaxes, markdown setup, license)", url: "https://github.com/vimwiki/vimwiki/blob/dev/README.md" },
    { label: "vimwiki help: links, tags, backlinks, commands (doc/vimwiki.txt)", url: "https://github.com/vimwiki/vimwiki/blob/dev/doc/vimwiki.txt" },
  ],
};
