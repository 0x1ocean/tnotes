import type { GlossaryEntry } from "../types";

export const entry: GlossaryEntry = {
  slug: "tag",
  term: "Tag (#tag)",
  short:
    "A tag is a keyword attached to a note, usually written inline as #word, that groups notes across folders and can be filtered on.",
  body: `A tag is a label on a note. Unlike a folder, a note can carry many tags, and a tag can span every folder. In Markdown notes the usual form is an inline hashtag, \`#meeting\`, written anywhere in the text; some tools also or instead take a list in front matter.

## Origin

Tagging as user-assigned keywords spread through the 2000s web (del.icio.us bookmarks, Flickr photos) as an alternative to fixed categories. The \`#\` prefix comes from IRC channel names by way of Twitter: Chris Messina proposed \`#barcamp\`-style groups in an August 2007 tweet, and Twitter started hyperlinking hashtags to search in 2009 ([Wikipedia: Hashtag](https://en.wikipedia.org/wiki/Hashtag)). Note tools adopted the inline form because it needs no UI: the tag is part of the text, survives copy and paste, and is visible in any editor.

## How it works

The tool scans each note for \`#\` followed by a word, builds an index from tag to notes, and offers a filter or search operator. Nested tags extend the word with slashes, \`#work/project\`, so that filtering on \`#work\` also matches its children; Obsidian documents this exact behaviour and rule set: letters, numbers, \`_\`, \`-\` and \`/\`, at least one non-digit, case-insensitive ([Obsidian: Tags](https://help.obsidian.md/tags)). Completion after typing \`#\` keeps spelling consistent, which matters because \`#meeting\` and \`#meetings\` are two tags.

## Pitfalls

\`#\` is overloaded in Markdown. \`# Heading\` at the start of a line is a heading, \`#1984\` may be an issue number, and a \`#\` inside a code span is code. Parsers differ on which of these they treat as tags. Tags do not carry meaning beyond their name, so a taxonomy drifts: \`#todo\`, \`#task\` and \`#tasks\` accumulate unless there is completion or a tag list to check against. Front-matter tags and inline tags are two systems, and a tool that reads only one will miss the other. Shell users hit one more: \`#\` starts a comment, so \`--tag '#work'\` needs quotes.

## In other tools

Obsidian reads inline \`#tags\` and a \`tags\` list in properties, with a tags view for browsing. jrnl uses \`@\` as its tag symbol precisely because \`#\` is reserved in shells, and warns when a new tag looks like a typo of an existing one ([jrnl docs](https://jrnl.sh/en/stable/usage/)). nb accepts \`--tags tag1,tag2\` on \`nb add\` and \`#tagging\` in the text ([nb README](https://github.com/xwmx/nb)).`,
  inTnotes: `tnotes parses \`#tags\` from the note text, including nested ones like \`#work/project\`. There is no tag field; the text is the source of truth. In the editor, typing \`#\` then \`tab\` completes from existing tags, and \`ctrl+click\` on a tag filters the list to it. Tags are highlighted in the \`theme.tag\` colour (\`green\` by default) when \`editor.highlight = "color"\`.

The CLI filters and creates by tag; a leading \`#\` is stripped so quoting is harmless:

\`\`\`sh
tnotes ls --tag work
tnotes search "invoice" --tag '#work/acme' --json
tnotes new "Standup" --tag work
\`\`\`

\`ls --json\` includes a \`tags\` array per note. A \`tags:\` line in a header is not parsed; write the tag inline instead. See [/docs/cli](/docs/cli) and [/glossary/frontmatter](/glossary/frontmatter).`,
  related: ["frontmatter", "daily-note", "vault"],
};
