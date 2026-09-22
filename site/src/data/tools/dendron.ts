import type { Tool } from "../types";

export const tool: Tool = {
  slug: "dendron",
  name: "Dendron",
  url: "https://wiki.dendron.so/",
  tagline:
    "VS Code extension for hierarchical Markdown notes named a.b.c.md, in maintenance mode since February 2023.",
  category: "editor plugin",
  platforms: "VS Code or VSCodium on Windows, macOS, Linux; CLI via Node.js",
  license: "Apache-2.0 (relicensed from GPL-3.0 at the maintenance announcement)",
  pricing: "Free",
  features: {
    storage: { v: "yes", note: "plain .md files, one flat vault folder; hierarchy is in the file name, YAML front matter required" },
    terminal: { v: "no", note: "editor is VS Code; the CLI is headless but not a TUI" },
    wikilinks: { v: "yes", note: "[[dotted.note.name]], plus block anchors and cross-vault links" },
    backlinks: { v: "yes", note: "Backlinks panel in the sidebar" },
    tags: { v: "yes", note: "#tag in text or `tags:` in front matter; each tag is a note under tags.*" },
    cli: { v: "yes", note: "`dendron note get | find | lookup | write | delete | move`" },
    json: { v: "yes", note: "`--output json` on the note commands" },
    liveReload: { v: "partial", note: "VS Code reloads clean editors; Reload Index re-reads schemas" },
    vim: { v: "partial", note: "through the VSCodeVim extension" },
    mouse: { v: "yes" },
    sync: { v: "partial", note: "vaults are folders meant for git; no sync service" },
    encryption: { v: "no" },
    mobile: { v: "no" },
    plugins: { v: "partial", note: "pods for import/export and hooks; the rest is the VS Code ecosystem" },
    openSource: { v: "yes", note: "Apache-2.0, TypeScript; maintenance mode, last commit June 2025" },
  },
  intro: `Dendron's idea is that a knowledge base should be refactored like code. Notes are named by their place in a hierarchy, so \`project1.tasks.task1.md\` is the child of \`project1.tasks.md\`, and all of them sit in one flat vault folder. Lookup (\`Ctrl+L\`) finds or creates notes by that dotted path, schemas type the hierarchy and apply templates, and Rename Note or Refactor Hierarchy move whole subtrees while rewriting every wikilink that pointed into them. Every note carries YAML front matter with an \`id\`, \`title\`, \`created\` and \`updated\`; Dendron warns when it is missing.

The extension is the product, but it is not the only entry point. \`@dendronhq/dendron-cli\` runs \`dendron note lookup --query hello --output json\` and \`dendron note write --fname mytest --body "..."\` against a workspace without VS Code, and the same engine publishes a vault as a Next.js site.

The state of the project matters more than any feature. In February 2023 the founder announced on Discord that the company had pivoted, that "Dendron, the extension, will no longer be actively developed" and that the project would be in maintenance mode; the README links to the GitHub discussion that reproduces the announcement. Pull requests are reviewed on a best-effort basis, the last commit is from June 2025, and the extension still installs and works, but nothing new is coming. Anyone choosing Dendron in 2026 is choosing a finished tool.`,
  chooseTool: [
    "You want hierarchy as the primary structure: dotted names, stubs for missing parents, schemas that enforce shape and templates.",
    "You refactor: renaming a note or a whole hierarchy rewrites every wikilink across the vault.",
    "You already work in VS Code and want notes in the same window as code.",
    "You publish: the built-in Next.js site generator handles per-vault, per-hierarchy and per-note publishing permissions.",
    "You are comfortable with a project in maintenance mode because it is local-first and the files are plain Markdown.",
  ],
  chooseTnotes: [
    "You want an actively developed tool: tnotes 1.3.0 shipped on 21 September 2026 with `write`, `append` and `restore` in the CLI.",
    "You want a terminal UI, not an editor extension: tnotes runs over SSH and in a tmux pane.",
    "You want folders on disk, `[[Title]]` links and `#tags` with no required front matter.",
    "Scripts and agents need `ls`, `search`, `cat`, `new`, `append`, `write` with `--json` and no Node.js runtime.",
  ],
  together: `A Dendron vault is a folder of \`.md\` files, so tnotes can open it, but two Dendron conventions collide with tnotes' rules:

- tnotes takes the title from the first non-empty line. Every Dendron note starts with the \`---\` front-matter fence, so \`tnotes ls\` shows \`---\` as the title of every note and the YAML shows as text in the editor.
- tnotes renames a file to a slug of its title whenever it saves. The slug of \`---\` is \`untitled\`, so editing \`project1.tasks.task1.md\` in the tnotes TUI, or with \`tnotes append\` / \`tnotes write\`, renames it to \`untitled.md\` (then \`untitled-2.md\`, and so on). That breaks Dendron's hierarchy and its links.
- \`[[project1.tasks.task1]]\` does not resolve: tnotes matches links against titles and against slugged file stems, and the slug of a dotted name has dashes, not dots.
- \`#example.my-example\` is read by tnotes as \`#example\`; tags stop at the dot.

So use tnotes read-only on a live Dendron vault (\`ls\`, \`search\`, \`cat\` never write), or convert once. This turns the dotted hierarchy into folders and the front-matter \`title\` into the first line:

\`\`\`sh
src=~/Dendron/vault; dst=~/notes
for f in "$src"/*.md; do
  stem=$(basename "$f" .md)
  out="$dst/$(printf '%s' "$stem" | tr . /).md"
  mkdir -p "$(dirname "$out")"
  awk 'NR==1 && $0=="---" {fm=1; next}
       fm && $0=="---"      {fm=0; print "# " title; next}
       fm && sub(/^title: */, "") {gsub(/^"|"$/, ""); title=$0; next}
       fm {next}
       {print}' "$f" > "$out"
done
tnotes "$dst"
\`\`\`

\`project1.tasks.task1.md\` becomes \`project1/tasks/task1.md\` titled from its front matter. Wikilinks still contain dotted names after this; rewrite them to \`[[Title]]\` with a second pass or leave them and let \`alt+enter\` create the missing targets as you go. Front-matter \`tags:\` are dropped; add \`#tags\` in the body where you want them.`,
  faq: [
    {
      q: "Is Dendron still maintained?",
      a: "It is in maintenance mode. The founder announced in February 2023 that active development had stopped after the company pivoted; the README states it plainly, and the last commit to the repository is from June 2025. The extension still works because everything is local files, and pull requests are reviewed on a best-effort basis.",
    },
    {
      q: "How do I get my notes out of Dendron?",
      a: "They are already Markdown files in the vault folder, named `a.b.c.md` with YAML front matter. Any Markdown tool opens them. To move to a folder-based tool, split the dotted names into directories and move the front-matter `title` into a heading; the recipe above does that for tnotes.",
    },
    {
      q: "Does Dendron have a CLI?",
      a: "Yes. `npm install -g @dendronhq/dendron-cli` gives `dendron note get | find | lookup | write | delete | move`, with `--output json`, `md_gfm` or `md_dendron`, plus publishing and pod commands. It needs a workspace with `dendron.yml` and starts its own engine unless you attach to a running one.",
    },
  ],
  verifiedOn: "2026-09-22",
  sources: [
    { label: "Dendron README (maintenance notice, features, Apache-2.0 license)", url: "https://github.com/dendronhq/dendron" },
    { label: "GitHub discussion #3890 reproducing the February 2023 announcement", url: "https://github.com/dendronhq/dendron/discussions/3890" },
    { label: "Dendron wiki: note CLI (dendron note …, --output json)", url: "https://wiki.dendron.so/notes/wti0omzx9zzfsfg67vc1kj0/" },
    { label: "Dendron wiki: front matter, tags and hierarchies", url: "https://wiki.dendron.so/notes/ffec2853-c0e0-4165-a368-339db12c8e4b/" },
  ],
};
