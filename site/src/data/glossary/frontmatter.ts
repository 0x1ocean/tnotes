import type { GlossaryEntry } from "../types";

export const entry: GlossaryEntry = {
  slug: "frontmatter",
  term: "Front matter",
  short:
    "Front matter is a metadata block at the top of a Markdown file, usually YAML between two --- lines, holding fields like title, date and tags.",
  body: `Front matter is a block of key-value metadata at the very start of a text file, separated from the body by delimiter lines. The common form is YAML between two lines of \`---\`:

\`\`\`md
---
title: Weekly plan
tags: [work, planning]
date: 2026-09-22
---

# Weekly plan
\`\`\`

The body is ordinary Markdown; the header is structured data the tool can read without parsing prose.

## Origin

The convention comes from static site generators. Jekyll (2008) processes any file that begins with a YAML block between triple-dashed lines and exposes the fields to templates; the block "must be the first thing in the file" ([Jekyll: Front Matter](https://jekyllrb.com/docs/front-matter/)). Hugo accepts the same YAML form plus TOML between \`+++\` lines and JSON in braces, and picks the parser from the delimiter ([Hugo: Front matter](https://gohugo.io/content-management/front-matter/)). Note-taking tools inherited the YAML variant because so many notes end up published through one of these generators.

## How it works

A parser checks whether line 1 is exactly \`---\`, finds the next \`---\`, and hands the text between them to a YAML parser. Everything after is the document. A reader that does not know the convention sees a horizontal rule, some \`key: value\` lines, and a second rule; the content is still readable, just cluttered.

## Pitfalls

YAML is stricter than it looks. A colon in an unquoted value (\`title: Note: draft\`), a leading \`#\` (which starts a comment), or an \`@\` will make the block fail to parse, and tools differ on whether they then show an error or silently treat the whole file as body. Field names are not standardised: one tool reads \`tags\`, another \`keywords\`, and a plain \`tags: work\` string versus a \`tags: [work]\` list is a common mismatch. Front matter is also invisible to tools that only index the body, so a tag that exists only in the header may not be searchable everywhere. Obsidian, for instance, requires tags in YAML to be written as a list.

## In other tools

Obsidian calls the block *properties*, gives it a form-style editor, and types each field (text, list, number, date, checkbox, tags); it also accepts JSON in the block but saves it back as YAML ([Obsidian: Properties](https://help.obsidian.md/properties)). nb reads a title from either a Markdown \`# h1\` or front matter ([nb README](https://github.com/xwmx/nb)). Jekyll and Hugo treat it as the page's variables and never show it to readers.`,
  inTnotes: `tnotes has no front matter support. It does not parse a YAML block, and a note that starts with \`---\` is shown as text like any other line. The first line of the file is the title, so a front matter block would make \`---\` the title and the filename slug.

Metadata lives in the body instead: the title is the first line (the default template is \`# \`), and tags are inline \`#tags\`, including nested ones like \`#work/project\`, parsed from the text. A \`tags:\` field in a header is not a tag to tnotes.

\`\`\`sh
tnotes new "Weekly plan" --tag work
tnotes ls --json | jq '.[0] | {title, tags, created, modified}'
\`\`\`

\`created\` and \`modified\` timestamps are reported per note by \`ls --json\` without any header in the file. See [/docs/config](/docs/config) and [/glossary/tag](/glossary/tag).`,
  related: ["markdown", "tag", "plain-text-notes"],
};
