import type { Tool } from "../types";

export const tool: Tool = {
  slug: "nb",
  name: "nb",
  url: "https://xwmx.github.io/nb/",
  tagline:
    "Single-script Bash CLI for notes, bookmarks and archives with git versioning, [[links]], tags and encryption.",
  category: "CLI",
  platforms: "Linux, macOS, any Unix; Windows via WSL, MSYS or Cygwin",
  license: "AGPL-3.0",
  pricing: "Free",
  features: {
    storage: {
      v: "yes",
      note: "plain files (Markdown, Org, LaTeX, AsciiDoc, anything) in git-backed notebooks; a `.index` per folder maps ids to filenames",
    },
    terminal: {
      v: "yes",
      note: "pure CLI; `nb browse` serves a local web app for terminal or GUI browsers",
    },
    wikilinks: {
      v: "yes",
      note: "[[123]], [[Title]], [[notebook:folder/Title]], `|` display text",
    },
    backlinks: {
      v: "partial",
      note: "`backlink` plugin appends a Backlinks section to linked files",
    },
    tags: { v: "yes", note: "#tag and nested #project/design/ui" },
    cli: {
      v: "yes",
      note: "add, edit, list, search, move, delete, pin, todo, bookmark, import, export…",
    },
    json: {
      v: "no",
      note: "`--paths`, `--filenames`, `--no-id` for plain output; no JSON",
    },
    liveReload: {
      v: "partial",
      note: "no long-running process; `.index` is reconciled with files on disk",
    },
    vim: { v: "partial", note: "editing happens in your `$EDITOR`" },
    mouse: {
      v: "partial",
      note: "keyboard CLI; `nb browse --gui` opens a web browser",
    },
    sync: {
      v: "yes",
      note: "automatic git push/pull per notebook; also works under Dropbox or Syncthing",
    },
    encryption: {
      v: "yes",
      note: "per-item AES-256 via OpenSSL or GPG with `--encrypt`",
    },
    mobile: {
      v: "no",
      note: "the git remote or synced folder is reachable from any mobile editor",
    },
    plugins: {
      v: "yes",
      note: "`.nb-plugin` subcommands and `.nb-theme` colour themes in Bash",
    },
    openSource: { v: "yes", note: "AGPL-3.0, Bash" },
  },
  intro: `nb is a note-taking, bookmarking and archiving tool written as one portable Bash script. It needs Bash, git and a text editor; optional dependencies such as Pandoc, w3m or pdftotext unlock extra features when present. Notes live in notebooks, which are git repositories under \`~/.nb\` by default, or any folder you initialize with \`nb notebooks init\`. Every command commits, and a notebook with a remote pushes and pulls automatically.

Items are addressed by numeric id, filename or title, with a \`notebook:folder/id\` selector syntax that reaches across notebooks. Ids come from a \`.index\` file per folder where the line number is the id, so ids survive a clone. Linking uses \`[[wiki-style]]\` syntax that accepts ids, titles and paths, tags are \`#hashtags\` including nested ones, and \`nb browse\` renders it all as a local website you can open in w3m or a GUI browser. nb also does things tnotes does not attempt: bookmarks with cached page text and Wayback lookups, per-item encryption, todos with tasks, Pandoc import and export, and plugins written in shell.

nb's strength is breadth in a scriptable shell interface. Its weakness for interactive use is that reading and editing are delegated: you view through a pager or browser and edit in \`$EDITOR\`, with no live editor of its own.`,
  chooseTool: [
    "You want git history for every change without thinking about it; nb commits on each command and syncs notebooks to a remote.",
    "You store more than notes: bookmarks with cached page content, PDFs, images, encrypted items, todos.",
    "Your editor is the point. nb hands files to Vim, Emacs, VS Code or whatever `$EDITOR` says, so you keep your full editing setup.",
    "You need Org, LaTeX or AsciiDoc alongside Markdown, or Pandoc conversion on export.",
    "You want a shell-scriptable tool with plugins written in the same language.",
  ],
  chooseTnotes: [
    "You want an editor in the terminal: live Markdown highlighting, `[[` and `#` completion, backlinks overlay, tabs, mouse selection, all inside one binary.",
    "Scripts and agents need structured output; `tnotes ls --json` returns id, path, title, folder, tags, links, backlinks and timestamps, while nb prints text lists.",
    "Retitling a note should rewrite `[[links]]` in other notes; tnotes does that, and nb's docs describe id and title links but no rewriting on rename.",
    "You want a running UI that picks up external changes live; nb has no long-running process and tnotes reloads clean tabs when files change.",
    "You prefer plain files with no index: tnotes derives everything from the `.md` files and the directory tree, so there is nothing to reconcile.",
  ],
  together: `Both tools work on plain files, so a directory can be an nb notebook and a tnotes root at the same time. nb needs a \`.git\` directory and a \`.index\` file; tnotes ignores dot-prefixed entries, so neither shows up in its tree.

\`\`\`sh
nb notebooks init ~/notes         # git repo + .index in an existing folder
tnotes ~/notes                    # add it as a tnotes root
cd ~/notes                        # inside the folder, nb uses it as the "local" notebook
nb add --title "Standup" --content "- [ ] slides"   # nb writes standup.md, tnotes shows it live
tnotes new "Retro" --stdin < retro.md                # tnotes writes retro.md, nb indexes it
nb list                                              # .index reconciled with the new file
\`\`\`

Rules that keep the two consistent. Create nb notes with \`--title\`: it writes a \`# Title\` first line and names the file after it, whereas a bare \`nb add\` names the file by datetime and tnotes would show the first line of content as the title. Use \`[[Title]]\` links rather than \`[[123]]\`; both resolve titles, only nb resolves ids. Avoid changing a title in tnotes: tnotes renames the file, nb's \`.index\` reconciles that as a delete plus an add, and the note gets a new id. Both tools read the same \`#tags\`, including nested ones. If nb has a remote configured, run any nb command after editing in tnotes to commit and push; tnotes never touches git. See [/docs/cli](/docs/cli) and [/for/sysadmins](/for/sysadmins).`,
  faq: [
    {
      q: "Is nb a TUI or a CLI?",
      a: "A CLI. nb has no full-screen interface of its own; it prints lists to the terminal, opens notes in `$EDITOR`, and `nb browse` serves a local web app you can read in a terminal browser such as w3m or in a GUI browser. tnotes is both: a TUI editor and a headless CLI on the same files.",
    },
    {
      q: "Does nb output JSON?",
      a: "No. `nb list` and `nb ls` have flags for paths, filenames and excerpts, and `nb search` prints matches, but there is no JSON format. Scripts parse text. tnotes adds `--json` to every command and includes links and backlinks in the output.",
    },
    {
      q: "Can nb and tnotes share a notebook?",
      a: "Yes, if the notebook is a plain folder of Markdown files. Initialize the folder with `nb notebooks init` and add it to tnotes' `roots`. nb's `.git` and `.index` are hidden from tnotes, and nb reconciles its index when tnotes adds or removes files. Prefer `[[Title]]` links and avoid retitling notes in tnotes, since the rename changes the note's nb id.",
    },
  ],
  verifiedOn: "2026-09-22",
  sources: [
    { label: "nb README (GitHub)", url: "https://github.com/xwmx/nb" },
    {
      label: "nb README: Linking",
      url: "https://github.com/xwmx/nb#-linking",
    },
    {
      label: "nb README: Git Sync",
      url: "https://github.com/xwmx/nb#-git-sync",
    },
    {
      label: "nb README: .index files",
      url: "https://github.com/xwmx/nb#index-files",
    },
  ],
};
