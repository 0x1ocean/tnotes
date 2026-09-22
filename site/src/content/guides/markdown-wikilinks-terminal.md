---
title: "Markdown wikilinks in the terminal: resolution, backlinks, renames"
description: "How [[wikilinks]] resolve by filename, title or id in Obsidian, vimwiki, Logseq, Zettlr, Dendron and tnotes; backlinks, rename handling, and ripgrep recipes."
published: 2026-09-22
keywords: ["wikilinks", "markdown links", "backlinks", "ripgrep notes", "note linking"]
order: 2
---

A [wikilink](/glossary/wikilink) is two brackets around a name: `[[Weekly plan]]`. Most tools also accept `[[Target|shown text]]` for a display label and `[[Target#Heading]]` for an anchor. It is not part of CommonMark or GitHub Flavored Markdown, so a plain Markdown renderer prints the brackets literally. The syntax is shared; what the name inside the brackets refers to is not. That is where tools disagree, and it is what breaks when you move a vault from one tool to another.

This guide covers the three resolution strategies in use, how backlinks are computed, what happens when the target is renamed, and how to answer link questions in a bare folder with `rg` when no tool is running.

## What the name resolves to

There are three answers, and a vault written for one is subtly wrong under another.

### By filename

The link text is a file path without the extension. `[[Weekly plan]]` means `Weekly plan.md`, somewhere in the vault.

Obsidian works this way: `[[Three laws of motion]]` and `[[Three laws of motion.md]]` are equivalent, a folder prefix such as `[[Projects/Three laws of motion]]` is a path from the vault root, and a link to a missing note creates the file at that path ([Obsidian help: Internal links](https://raw.githubusercontent.com/obsidianmd/obsidian-help/master/en/Linking%20notes%20and%20files/Internal%20links.md)). The note's first heading is irrelevant; the filename is the identity.

vimwiki is the same model with Vim path rules: `[[projects/Important Project 1]]` is relative to the current file's wiki, `[[/index]]` is absolute to the wiki root, `[[../index]]` walks up, and `wiki1:` or `wn.Name:` prefixes address another registered wiki ([vimwiki documentation](https://raw.githubusercontent.com/vimwiki/vimwiki/dev/doc/vimwiki.txt), section 5.2). Default extension is `.wiki`; `.md` is a per-wiki setting.

Dendron is filename-based with a twist: dots in the filename form a hierarchy, so `project1.designs.promotion.md` is the note `promotion` under `project1.designs`, and the link is `[[project1.designs.promotion]]` ([Dendron: Concepts](https://wiki.dendron.so/notes/c6fd6bc4-7f75-4cbb-8f34-f7b99bfe2d50/)). Folders are not used for structure at all.

Consequence: the filename must be human-readable, since it is what you type inside the brackets, and spaces in filenames are normal. Anything that slugifies filenames (`weekly-plan.md`) is incompatible with links written for this model.

### By title

The link text is the note's title, a value inside the file, and the filename is derived from it or independent of it.

Logseq resolves `[[Brackets]]` and `#Hashtags` to a page by name, and states the two forms function the same; a link to a page that does not exist creates it ([Logseq docs: How to create pages](https://raw.githubusercontent.com/logseq/docs/master/pages/How%20to%20create%20pages%20in%20Logseq.md)). Page names are matched without regard to case in `[[...]]` references, while the `(page "…")` query form is case-sensitive ([logseq/logseq#4004](https://github.com/logseq/logseq/issues/4004)).

nb accepts either a title or an id inside the brackets: `[[Example Title]]`, `[[123]]`, `[[Sample Folder/Demo Title]]`, and across notebooks `[[demo:Example Title]]` or `[[sample:Example File.md]]` ([xwmx/nb: Linking](https://github.com/xwmx/nb#-linking)).

tnotes resolves by title first, case-insensitively, then falls back to the file stem; details in the tnotes section below.

Consequence: the title is inside the file, so the tool has to index every file's first line (or frontmatter) before it can resolve anything. In exchange the filename can be a slug, and case in the link does not matter.

### By id

The link text is an opaque identifier that never changes. The note's title and filename are free to change around it.

Zettlr's default id is a timestamp in the form `YYYYMMDDHHMMSS`, and the manual's argument is exactly the rename problem: "the title of the note can change, but the ID remains the same." A setting, "Use the file ID as link target if possible", makes autocomplete insert `[[20250922141500|Weekly plan]]` rather than the filename ([Zettlr manual: Zettelkasten methods](https://docs.zettlr.com/en/pkms/zkn-method/)). Zettlr can also link by filename, so a single vault can mix both.

nb's numeric ids (`[[123]]`) are the same idea without the timestamp.

Consequence: links never break on rename, and never read well in a plain text editor. Ids are the right choice for a vault that will outlive several tools, at the cost of every link needing a label to be legible. See [Zettelkasten](/glossary/zettelkasten) for the method this comes from.

## Backlinks

A [backlink](/glossary/backlink) is the reverse edge: every note that contains a link to the current one. No Markdown syntax expresses it; it is computed.

Two designs exist. Most tools keep an in-memory or on-disk index and show backlinks in a pane or overlay; nothing is written into the note. Obsidian, Logseq, Dendron and tnotes do this. tnotes additionally exposes the result as data: `tnotes ls --json` includes `links` and `backlinks` arrays for every note, so a script can walk the graph without the TUI ([tnotes README](https://github.com/0x1ocean/tnotes#use-with-ai-agents)).

The other design writes backlinks into the file. nb's `backlink.nb-plugin` crawls a notebook and appends a "Backlinks" section to each linked note listing the passages that reference it ([nb plugin source](https://github.com/xwmx/nb/blob/master/plugins/backlink.nb-plugin)). vimwiki's `:VimwikiBacklinks` populates a search list instead of editing files.

Written-in backlinks survive tool changes and show up in any editor; they also drift if you forget to rerun the crawler, and they make every note contain text you did not write. Indexed backlinks are always current and never touch the file, and vanish when you leave the tool.

## What happens on rename

This is the question to ask before committing to a tool. The answers:

| Tool | Resolution | On rename |
|---|---|---|
| Obsidian | filename | updates internal links automatically; can be switched to a prompt under Settings, Files and links ([source](https://raw.githubusercontent.com/obsidianmd/obsidian-help/master/en/Linking%20notes%20and%20files/Internal%20links.md)) |
| vimwiki | filename | `:VimwikiRenameFile` renames the page and rewrites links to it, including diary pages; a plain `mv` does not ([source](https://raw.githubusercontent.com/vimwiki/vimwiki/dev/doc/vimwiki.txt)) |
| Dendron | dot-hierarchy name | Rename Note updates all backlinks; Refactor Hierarchy does the same over a regex-matched set with a preview ([source](https://wiki.dendron.so/notes/9zwkp44wnlaa8p8dpt4w8tq/)) |
| Logseq | page name | references are updated; open issues track gaps for org-format aliases and `{{query}}` blocks ([logseq/logseq#7519](https://github.com/logseq/logseq/issues/7519)) |
| Zettlr | id or filename | id links are unaffected by rename; the manual recommends the id setting precisely because filename links break when a file is renamed ([source](https://docs.zettlr.com/en/pkms/zkn-method/)) |
| tnotes | title, then stem | changing a title rewrites `[[old title]]` to `[[new title]]` in every other note, including open unsaved tabs; the file is renamed to the new slug ([source](https://github.com/0x1ocean/tnotes/blob/main/CHANGELOG.md)) |

Two things the table hides. First, "rename" means "through the tool". Rename a file with `mv` or in Finder and every filename-based tool now has a dangling link. Second, rewriting links is a mass edit across the vault, which is exactly what a syncer running on another machine may also be touching; a rename while another device has unsynced edits is the most reliable way to produce a conflict copy. See [Syncing plain-text notes](/guides/plain-text-notes-sync).

## Grepping links in a plain folder

None of the above requires a tool to be running. With `rg` you can answer the common questions over any Markdown folder.

Every note that links to a given title:

```sh
rg -l --fixed-strings '[[Weekly plan' ~/notes
```

The `--fixed-strings` avoids escaping the brackets. Dropping the closing `]]` also catches `[[Weekly plan|label]]` and `[[Weekly plan#Heading]]`.

Every link target in the vault, deduplicated with counts:

```sh
rg -o --no-filename '\[\[([^\]|#]+)' -r '$1' ~/notes | sort | uniq -c | sort -rn
```

Broken links, for a filename-resolved vault where the target must exist as `<name>.md`:

```sh
cd ~/notes
rg -o --no-filename '\[\[([^\]|#]+)' -r '$1' . | sort -u > /tmp/targets
fd -e md . | sed 's|^\./||; s|\.md$||' | sort -u > /tmp/files
comm -23 /tmp/targets /tmp/files
```

Notes nobody links to (orphans), same assumption:

```sh
comm -13 /tmp/targets /tmp/files
```

For a title-resolved vault, replace the `fd` line with one that extracts the first heading: `rg -m1 --no-filename '^# (.*)' -r '$1' .`, and compare case-insensitively with `sort -uf` and `comm -i`.

Backlinks for the note you have open in any editor, as a one-liner to bind to a key:

```sh
rg -n --fixed-strings "[[$(head -1 current.md | sed 's/^# //')" ~/notes
```

These are approximate. They do not know about links inside fenced code blocks, escaped brackets, or aliases. For a few thousand notes they are fast enough that precision is rarely worth the tooling.

## How tnotes resolves links

From the README and changelog, and only that:

- `[[Note title]]` is resolved by title, case-insensitively, then by file stem. The title is the note's first line; the filename is a slug of it and is renamed when the title changes.
- Typing `[[` opens a completion popup over existing titles; `alt+enter` or `ctrl+click` follows the link. A missing target is created next to the current note.
- `alt+enter` on a line without a link opens the backlinks overlay; the status bar shows `↩ N` for the count.
- Changing a title rewrites the link in every other note, including tabs with unsaved edits. The same happens through the CLI: `tnotes write <note> --stdin` with a new first line renames the file and updates `[[links]]`.
- `tnotes ls --json` prints `links` and `backlinks` for every note; `tnotes cat` prints the text.

The stem fallback means a link written for a filename-resolved vault (`[[weekly-plan]]`) still works if the slug matches, and a link written for a title-resolved vault (`[[Weekly Plan]]`) works regardless of case. Nested folder paths inside the brackets and `#heading` anchors are not mentioned in the README, so do not assume them. Keys are listed in [/docs/keys](/docs/keys); the CLI fields in [/docs/cli](/docs/cli).

## Writing links that survive a change of tool

If the vault may ever be opened by a different tool, a few habits keep the links resolvable:

- Make the filename and the title agree. A file `Weekly plan.md` whose first line is `# Weekly plan` resolves under both filename and title models. A slugged filename with a titled heading resolves under title and stem models only.
- Do not rely on case. Obsidian on a case-sensitive Linux filesystem and tnotes disagree about whether `[[weekly plan]]` finds `Weekly plan`.
- Avoid the characters Obsidian lists as unsafe in link text: `# | ^ : %%`. They have meaning in at least one tool.
- Keep one note per file with one title. Block references (`#^id`) are Obsidian-specific and do not travel.
- Rename through the tool, never with `mv`, unless you are about to run the broken-links check above.

For side-by-side detail on specific tools, see [tnotes vs Obsidian](/compare/tnotes-vs-obsidian), [tnotes vs Zettlr](/compare/tnotes-vs-zettlr), and [tnotes vs Dendron](/compare/tnotes-vs-dendron).
