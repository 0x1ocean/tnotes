import type { GlossaryEntry } from "../types";

export const entry: GlossaryEntry = {
  slug: "wikilink",
  term: "Wikilink",
  short:
    "A wikilink is a link written as [[Page title]] that points to another note by its title instead of a URL or file path.",
  body: `A wikilink is a link to another page written as the target's title inside double square brackets: \`[[Page title]]\`. The reader sees the title; the software resolves it to a page at render time or when the link is followed.

## Origin

The first wiki, Ward Cunningham's [WikiWikiWeb](https://en.wikipedia.org/wiki/WikiWikiWeb) (1995), made links by running capitalised words together: \`SoftwareDesignPatterns\` was both a page name and a link to it. The bracket syntax came later with wiki engines that wanted links containing spaces and lowercase words. MediaWiki, the software behind Wikipedia, documents \`[[like this]]\` as its internal link form and calls such links "wikilinks"; a link to a page that does not exist is rendered red and following it opens the page for creation ([MediaWiki Help:Links](https://www.mediawiki.org/wiki/Help:Links)). Note-taking tools borrowed the syntax and the red-link behaviour: an unresolved link is an invitation to write the note.

## How it works

Resolution is by name, not location. The tool keeps an index from titles (usually also filenames) to files, and looks the bracketed text up in it. Most tools add a pipe for display text (\`[[Target|shown text]]\`) and a hash for a heading (\`[[Target#Section]]\`). Obsidian also allows a folder prefix and block references (\`[[Note#^id]]\`), which its own documentation marks as non-portable ([Obsidian: Internal links](https://help.obsidian.md/links)).

## Pitfalls

Wikilinks are not CommonMark. A plain Markdown renderer prints the brackets literally, and \`[text](target.md)\` is the portable alternative. Case sensitivity varies: MediaWiki ignores the case of the first letter only, while tnotes matches whole titles case-insensitively. Duplicate titles in different folders make a bare link ambiguous, and each tool picks a winner by its own rule. Renaming a note breaks every link to it unless the tool rewrites them, so check whether yours does before reorganising.

## In other tools

Obsidian generates wikilinks by default and can be switched to Markdown links. Vimwiki turns a word into \`[[word]]\` with Enter and supports \`[[link|description]]\` ([vimwiki README](https://github.com/vimwiki/vimwiki)). Foam, a VS Code extension, uses the same syntax and can emit Markdown reference definitions so the links resolve on GitHub ([Foam README](https://github.com/foambubble/foam)). Joplin links by internal id, \`[text](:/0b0d62d1…)\`, so a title change never breaks a link but the source text is not readable on its own ([Joplin Markdown guide](https://joplinapp.org/help/apps/markdown)).`,
  inTnotes: `tnotes links notes with \`[[Note title]]\`. Typing \`[[\` opens a completion popup; \`tab\` accepts a title. A link resolves by title, case-insensitively, then by file stem. \`alt+enter\` or \`ctrl+click\` follows it, and a missing target is created next to the current note.

Renaming a note, in the TUI or with \`tnotes write\`, rewrites \`[[old title]]\` to \`[[new title]]\` in every other note, including open unsaved tabs. The headless CLI exposes the graph:

\`\`\`sh
tnotes ls --json   # each note lists its links and backlinks
\`\`\`

Only the bare \`[[Title]]\` form is documented; there is no pipe or heading syntax in the README. See [/docs/keys](/docs/keys) and [/guides/markdown-wikilinks-terminal](/guides/markdown-wikilinks-terminal).`,
  related: ["backlink", "markdown", "zettelkasten"],
};
