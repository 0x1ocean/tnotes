import type { GlossaryEntry } from "../types";

export const entry: GlossaryEntry = {
  slug: "backlink",
  term: "Backlink",
  short:
    "A backlink is an inbound link: the list of notes that link to the note you are reading, computed by the tool rather than written by hand.",
  body: `A backlink is a link seen from the target's side. If note A contains \`[[B]]\`, then B has a backlink from A. Nobody writes backlinks; the tool scans every note for links and inverts the index, so each note can show who refers to it.

## Origin

The idea is as old as hypertext, but wikis made it a standard feature. MediaWiki, the software behind Wikipedia, exposes it as "What links here" and keeps a dedicated \`pagelinks\` table so that the inverse lookup is cheap ([MediaWiki Help:Links](https://www.mediawiki.org/wiki/Help:Links)). Personal note tools adopted the same view once \`[[wikilinks]]\` became common in Markdown vaults; Obsidian's backlinks plugin defines the term the same way and adds a second section for *unlinked mentions*, plain-text occurrences of the note's name ([Obsidian: Backlinks](https://help.obsidian.md/plugins/backlinks)).

## How it works

The tool keeps an index: for each note, the set of link targets it contains. Backlinks for note B are every note whose set contains B. The index must be rebuilt when a file changes, and title resolution happens first, so an ambiguous or misspelled link either lands on the wrong note or on none. A naive implementation compares every note against every other note; that is quadratic and shows up as a multi-second stall once a vault passes a few hundred files.

## Pitfalls

Backlinks are only as good as link resolution. Unlinked mentions are a separate, more expensive feature, and few terminal tools offer it. Renaming a note without rewriting the links that point at it silently empties its backlink list. A backlink panel that lists titles without context tells you that a connection exists but not why, so tools differ in whether they show the linking paragraph.

## In other tools

Obsidian shows linked and unlinked mentions in a sidebar tab, with optional surrounding context, and can render the list at the bottom of the note. Vimwiki has \`:VimwikiBacklinks\`, which searches all files of the current wiki for links to the current page ([vimwiki docs](https://github.com/vimwiki/vimwiki/blob/master/doc/vimwiki.txt)). MediaWiki's "What links here" is a special page rather than part of the article view.`,
  inTnotes: `tnotes builds the backlink index from the \`[[links]]\` in your files. In the TUI, \`alt+enter\` on a line without a link opens the backlinks overlay; the status bar shows \`↩ N\` with the count, and the editor's right-click menu has the same entry. Rows in the overlay are clickable.

Because renaming a note rewrites \`[[old title]]\` to \`[[new title]]\` everywhere, the list survives reorganising. The headless CLI returns the same data:

\`\`\`sh
tnotes ls --json | jq '.[] | {id, backlinks}'
\`\`\`

Version 1.3.0 replaced a quadratic backlink resolver in \`ls --json\` with a linear one (2 s down to 40 ms for 500 notes, per the [changelog](/changelog)). Unlinked mentions are not indexed. See [/docs/cli](/docs/cli) and [/for/ai-agents](/for/ai-agents).`,
  related: ["wikilink", "zettelkasten", "vault"],
};
