import type { GlossaryEntry } from "../types";

export const entry: GlossaryEntry = {
  slug: "markdown",
  term: "Markdown",
  short:
    "Markdown is a plain-text formatting syntax (# headings, - lists, **bold**) designed to be readable as-is and convertible to HTML.",
  body: `Markdown is a way of writing formatted text in plain text: \`#\` for a heading, \`-\` for a list item, \`**bold**\`, \`[text](url)\`, fenced code between three backticks. The file reads fine without a renderer, which is the point.

## Origin

John Gruber published Markdown in 2004 as a Perl script and a syntax, with Aaron Swartz credited for feedback on the design. Gruber's stated goal was readability: a Markdown document "should be publishable as-is, as plain text, without looking like it's been marked up with tags", with plain-text email as the main influence ([daringfireball.net/projects/markdown](https://daringfireball.net/projects/markdown/)). The original spec left edge cases undefined, and implementations diverged. CommonMark, started in 2014, is a precise specification with a test suite; the current version is 0.31.2 from January 2024 ([spec.commonmark.org](https://spec.commonmark.org/)). GitHub Flavored Markdown extends CommonMark with tables, task lists (\`- [ ]\`), strikethrough and autolinks, and most note tools follow GFM.

## How it works

A renderer parses block structure first (paragraphs, headings, lists, code blocks, quotes), then inline structure within blocks (emphasis, links, code spans). Editors that "highlight" Markdown do not render it; they colour the source so the structure is visible while the syntax stays editable. Editors that render it show formatted output and hide or dim the markers.

## Pitfalls

Nesting is whitespace-sensitive: a sub-list needs enough indentation to sit inside its parent's content, and a code block inside a list item needs more. Two spaces at the end of a line are a hard line break and invisible. Note-app extensions (\`[[wikilinks]]\`, \`#tags\`, \`==highlight==\`, block ids, YAML front matter) are not Markdown; they are conventions layered on top, and each one is a place where a second tool will show raw text. The safe core is CommonMark plus GFM task lists.

## In other tools

Obsidian generates wikilinks by default and can be switched to standard Markdown links for interoperability ([Obsidian: Internal links](https://help.obsidian.md/links)). Joplin follows CommonMark and can show the source and the rendered document side by side ([Joplin Markdown guide](https://joplinapp.org/help/apps/markdown)). Vimwiki ships its own default markup plus a Markdown mode ([vimwiki README](https://github.com/vimwiki/vimwiki)). Org-mode is a different syntax with the same plain-text idea. Terminal tools mostly highlight rather than render, because a terminal has one font and no inline images.`,
  inTnotes: `tnotes highlights Markdown in place while you type: headings, lists, checkboxes, code, links, emphasis, quotes and \`#tags\`. There is no HTML rendering. \`editor.highlight = "color"\` uses the accent, code and tag colours from the \`[theme]\` section; \`"mono"\` uses only bold, italic and underline, for monochrome terminals.

The editor helps with structure: Enter continues a list and an empty item ends it, \`tab\` / \`shift+tab\` nest and un-nest an item, \`alt+x\` or a click toggles a \`- [ ]\` checkbox. \`editor.template\` sets the text of a new note (default \`# \`), and the first line is the note's title.

\`\`\`toml
[editor]
highlight = "mono"
template = "# "
width = 72
\`\`\`

See [/docs/config](/docs/config) and [/docs/keys](/docs/keys).`,
  related: ["plain-text-notes", "wikilink", "frontmatter"],
};
