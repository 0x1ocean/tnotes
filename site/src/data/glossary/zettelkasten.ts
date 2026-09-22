import type { GlossaryEntry } from "../types";

export const entry: GlossaryEntry = {
  slug: "zettelkasten",
  term: "Zettelkasten",
  short:
    "A Zettelkasten is a note system of small, single-idea notes, each with a fixed address, connected by explicit links rather than filed under topics.",
  body: `Zettelkasten is German for "slip box": a collection of small notes, one idea each, that are linked to one another instead of being sorted into categories. The method is defined by three rules: notes are atomic, every note has a permanent address, and connections are written into the notes as references.

## Origin

Card files for research go back centuries; Wikipedia traces them through Conrad Gessner, Leibniz and Linnaeus ([Wikipedia: Zettelkasten](https://en.wikipedia.org/wiki/Zettelkasten)). The modern method comes from the sociologist Niklas Luhmann, who built a box of roughly 90,000 cards between the 1950s and his death in 1998 and credited it for some 50 books and 550 articles. His 1981 essay *Kommunikation mit Zettelkästen* describes the design decision that matters: notes are not ordered by topic but by fixed position. Each card gets a number that never changes; a new card can be inserted behind any existing one (57/12 is followed by 57/12a, then 57/12a1), and any card can cite any other by number. A keyword register is the entry point, since there is no table of contents. Luhmann argues that a note without links "will get lost in the Zettelkasten" ([English translation at zettelkasten.de](https://zettelkasten.de/communications-with-zettelkastens/)).

## How it works on a computer

Software makes the fixed address trivial: a filename or a timestamp id is permanent enough, and \`[[wikilinks]]\` replace hand-copied numbers. Backlinks give the reverse direction that Luhmann had to maintain by hand. The discipline that remains is the writing: one idea per note, in your own words, with the link and a sentence saying why it is relevant.

## Pitfalls

The method rewards linking and punishes filing. Deep folder trees recreate the topic hierarchy Luhmann rejected. Timestamp ids (\`202609221430\`) are unambiguous but unreadable as link text, which is why most Markdown tools resolve links by title instead. Collecting is not the work; a large box of unlinked quotes is a scrapbook, not a Zettelkasten.

## In other tools

Obsidian and Foam are general-purpose and fit the method through wikilinks and backlinks. Zettlr lists Zettelkasten support as a headline feature and has a dedicated link-management module ([Zettlr README](https://github.com/Zettlr/Zettlr)); The Archive, from the zettelkasten.de authors, is a plain-text macOS app built around it ([The Archive](https://zettelkasten.de/the-archive/)). Any folder of Markdown files with title-based links is enough; the software matters less than the two habits of splitting and linking.`,
  inTnotes: `tnotes does not generate ids and has no Zettelkasten mode; it provides the primitives. Each note is a Markdown file whose title is its address: \`[[Title]]\` links resolve by title, \`alt+enter\` follows one, and a missing target is created on the spot, so a link written while thinking becomes the next note. The backlinks overlay and the \`backlinks\` field in \`ls --json\` give the reverse direction. Renaming a note rewrites every link to it, so titles can change without breaking the box.

\`\`\`sh
tnotes new "Attention is a bottleneck" --tag zk
tnotes ls --tag zk --json | jq '.[] | {title, links, backlinks}'
\`\`\`

Folders are ordinary directories and optional. See [/for/zettelkasten](/for/zettelkasten) and [/compare/tnotes-vs-zettlr](/compare/tnotes-vs-zettlr).`,
  related: ["wikilink", "backlink", "plain-text-notes"],
};
