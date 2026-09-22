import type { GlossaryEntry } from "../types";

export const entry: GlossaryEntry = {
  slug: "plain-text-notes",
  term: "Plain-text notes",
  short:
    "Plain-text notes are notes stored as ordinary text files on disk, readable and editable by any program, with no database or proprietary container.",
  body: `Plain-text notes are notes kept as ordinary files, one readable text file per note, in a folder you control. The format is the file itself: no database, no export step, no application needed to read it back.

## Origin

The practice predates note apps; it is how Unix users have always kept \`TODO\` and \`notes.txt\`. The recent argument for it was made most compactly by Steph Ango, CEO of Obsidian, in *File over app* (2023): "if you want to create digital artifacts that last, they must be files you can control, in formats that are easy to retrieve and read", with the corollary that notes readable on a computer from the 1960s will still be readable in the 2060s ([stephango.com/file-over-app](https://stephango.com/file-over-app)). Markdown gave the practice a shared dialect for headings and lists without leaving text.

## How it works

The filesystem is the database. Search is \`grep\` or \`rg\`; backup is \`cp\` or git; sync is whatever syncs files; encryption is an encrypted volume. Structure lives in three places: the folder tree, the text itself (\`#tags\`, \`[[links]]\`), and optionally a metadata header. A tool adds an index over the files for fast search and link resolution, and must rebuild that index when a file changes underneath it.

## Pitfalls

Two programs writing the same file is the classic failure. A sync client and an editor can each save a stale version over the other's, and a tool that writes with a plain \`open()\` can leave a half-written file if it is killed mid-save. Look for atomic saves (write to a temporary file, then rename) and for conflict copies instead of silent overwrites. Attachments are the other weak point: images and PDFs are files too, but a link to them is a path, and paths break when files move. Finally, "plain text" is a spectrum. A Markdown file with YAML front matter, block ids and embed syntax reads fine, but only one application renders it as intended.

## In other tools

Obsidian, Logseq, Zettlr, Foam, vimwiki and nb all store Markdown files in a folder. Joplin keeps notes in a SQLite database and exports Markdown on request; Apple Notes and Notion hold notes in a proprietary store with an export function. The distinction that matters is whether a second program can open the file the first one wrote, right now, without asking.`,
  inTnotes: `tnotes stores one \`.md\` file per note. The filename is a slug of the first line (the title) and is renamed when the title changes. Folders in the sidebar are directories; every root has a \`.Trash/\` that mirrors the layout. Nothing else is written next to your notes; session state and config live under \`~/.config/tnotes\` and \`~/.local/state/tnotes\`. Dot-prefixed entries are ignored, so \`.git\` and \`.stfolder\` do not show up as notes.

Saves are atomic. When a file changes on disk, a clean tab reloads; a tab with unsaved edits keeps them and a conflict copy, \`name (conflict <time>).md\`, is written instead of overwriting.

\`\`\`sh
rg -l '#work' ~/Documents/notes   # any tool works on the files
tnotes search work --json         # or ask tnotes
\`\`\`

See [/docs/sync](/docs/sync) and [/guides/plain-text-notes-sync](/guides/plain-text-notes-sync).`,
  related: ["markdown", "vault", "frontmatter"],
};
