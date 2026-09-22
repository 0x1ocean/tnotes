import type { Tool } from "../types";

export const tool: Tool = {
  slug: "org-mode",
  name: "Org mode",
  url: "https://orgmode.org/",
  tagline:
    "Emacs major mode for outlines, TODOs, agendas and literate documents in its own .org plain-text format.",
  category: "editor plugin",
  platforms: "Wherever Emacs runs: Linux, macOS, Windows, Android; terminal via emacs -nw",
  license: "GPL-3.0-or-later (ships with Emacs)",
  pricing: "Free",
  features: {
    storage: { v: "yes", note: "plain .org files; not Markdown" },
    terminal: { v: "yes", note: "`emacs -nw` runs the full mode in a terminal" },
    wikilinks: { v: "partial", note: "[[*Heading]], [[file:notes.org::*Heading]], [[id:UUID]]; not title-based [[Note]]" },
    backlinks: { v: "partial", note: "not in Org itself; org-roam adds a backlinks buffer" },
    tags: { v: "yes", note: ":work:urgent: on headlines, inherited down the outline, searchable in the agenda" },
    cli: { v: "partial", note: "`emacs --batch -l script.el`; you write the commands in Elisp" },
    json: { v: "partial", note: "nothing built in; external parsers (orgparse, orgize, hop) emit JSON" },
    liveReload: { v: "yes", note: "auto-revert-mode / global-auto-revert-mode" },
    vim: { v: "partial", note: "via the evil package; not part of Org" },
    mouse: { v: "yes", note: "Emacs mouse support, including in the terminal with xterm-mouse-mode" },
    sync: { v: "partial", note: "files; git, Syncthing, or any file sync" },
    encryption: { v: "yes", note: "org-crypt encrypts entries tagged :crypt: with GnuPG" },
    mobile: { v: "partial", note: "third-party: Orgzly Revived, Orgro, MobileOrg, organice, Emacs on Android" },
    plugins: { v: "yes", note: "Emacs packages: org-roam, org-contrib, hundreds more on MELPA" },
    openSource: { v: "yes", note: "GPL-3.0-or-later, Emacs Lisp" },
  },
  intro: `Org mode is an outliner first. A file is a tree of headlines; each headline can carry a TODO state, tags, a schedule, a deadline, a property drawer and a clock. The agenda collects those across every file you register and shows a day, a week or a custom query. Babel executes code blocks inside the document and captures the results, and the exporter produces HTML, LaTeX, ODT, Markdown and more from the same source. Org 9.8 is the current release, and the mode ships with Emacs, so there is nothing to install.

Linking in Org is precise rather than loose. \`[[*Heading]]\` targets a headline in the current file, \`[[file:projects.org::*Heading]]\` one in another file, \`[[id:UUID]]\` a headline by its \`ID\` property. Nothing in core Org matches a bare \`[[Note title]]\` against a folder of files, and nothing in core Org lists who links here. That is the gap org-roam fills: it indexes every node into an auxiliary database and shows backlinks in a side buffer.

The format is the decision. \`.org\` is not Markdown; the outline, the timestamps and the property drawers have no Markdown equivalent, which is why Org files stay in Emacs or in the Org-specific mobile apps. Pandoc reads and writes Org, so one-way conversion is a command, but round-tripping loses the parts that make Org worth using.`,
  chooseTool: [
    "You want the agenda: scheduled and deadline items across files, clocking, and TODO state workflows in one view.",
    "You write documents with executable code blocks (Babel) and export them to LaTeX, HTML or ODT.",
    "You already use Emacs; Org is built in and every Emacs package composes with it.",
    "You want entry-level encryption inside a file: `org-crypt` encrypts a headline's body when you save.",
    "Your notes need structure beyond headings: properties, columns, tables with formulas.",
  ],
  chooseTnotes: [
    "You want Markdown files that every other editor, phone app and static site generator reads.",
    "You need `[[Title]]` links with completion, backlinks and rename rewriting without configuring org-roam and a database.",
    "You want a CLI with JSON output for scripts and agents instead of writing Elisp for `emacs --batch`.",
    "You want a mouse-driven TUI that opens in under a second, not an Emacs configuration.",
  ],
  together: `There is no shared file format: tnotes lists only \`.md\` files and ignores everything else, and Org does not read Markdown. Two workable arrangements.

**Keep Org for the agenda, tnotes for notes.** Tasks with dates, clocks and TODO states stay in \`.org\` files; reference notes, meeting notes and anything you want to grep from a script live in a tnotes root. Both are plain text in the same git repository. Org's \`file:\` links can point into the Markdown folder (\`[[file:~/notes/weekly-plan.md]]\`), and tnotes ignores the \`.org\` files next to it.

**One-way export with Pandoc.** Pandoc reads \`org\` and writes GitHub-flavoured Markdown; run it into a tnotes root:

\`\`\`sh
mkdir -p ~/notes/from-org
for f in ~/org/*.org; do
  pandoc -f org -t gfm "$f" -o ~/notes/from-org/"$(basename "$f" .org)".md
done
tnotes ~/notes
\`\`\`

The Markdown is a snapshot: edits made in tnotes do not flow back, \`[[*Heading]]\` links become ordinary text or Markdown links (tnotes only indexes \`[[…]]\`), and \`:tag:\` headline tags are not \`#tags\`. Treat the export as read-only material for search and for agents that need \`tnotes ls --json\`, and keep editing the \`.org\` originals in Emacs. If you want both directions, keep the two sets of notes separate rather than converting back and forth.`,
  faq: [
    {
      q: "Does Org mode support backlinks?",
      a: "Not on its own. Org has forward links (`[[*Heading]]`, `[[file:…]]`, `[[id:…]]`) and no index of incoming links. org-roam adds that: it stores every node and link in an auxiliary database and shows the backlinks of the current node in the Org-roam buffer.",
    },
    {
      q: "Can I use Org mode from the command line without opening Emacs?",
      a: "Yes, with `emacs --batch -l script.el`: batch mode runs Elisp from a script, prints to stdout and exits. You write the script yourself; Org has no `list` or `search` subcommands. External parsers such as orgparse (Python), orgize (Rust) or hop (Babashka) read .org files without Emacs and can emit JSON.",
    },
    {
      q: "Can I convert Org files to Markdown?",
      a: "Pandoc reads and writes Org (`pandoc -f org -t gfm notes.org`) and Org's own exporter has a Markdown backend (`ox-md`). Headlines, lists, tables and code blocks convert well; timestamps, property drawers, TODO keywords and Babel results have no Markdown equivalent and come through as text or are dropped.",
    },
  ],
  verifiedOn: "2026-09-22",
  sources: [
    { label: "Org manual: External Links (file:, id: link types)", url: "https://orgmode.org/manual/External-Links.html" },
    { label: "Org tools page (mobile apps, Pandoc, external parsers)", url: "https://orgmode.org/tools.html" },
    { label: "Org-roam manual (backlinks buffer, database)", url: "https://www.orgroam.com/manual.html" },
    { label: "GNU Emacs manual: Initial Options (--batch, -nw)", url: "https://www.gnu.org/software/emacs/manual/html_node/emacs/Initial-Options.html" },
  ],
};
