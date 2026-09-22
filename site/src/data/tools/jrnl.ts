import type { Tool } from "../types";

export const tool: Tool = {
  slug: "jrnl",
  name: "jrnl",
  url: "https://jrnl.sh",
  tagline:
    "Python command-line journal: timestamped entries in one plain-text file, natural-language dates, optional AES encryption.",
  category: "CLI",
  platforms: "macOS, Linux, Windows (Python)",
  license: "GPL-3.0",
  pricing: "Free",
  features: {
    storage: {
      v: "yes",
      note: "one text file per journal (`[2020-06-28 18:22] Title` blocks), or a `YYYY/MM/DD.txt` folder journal",
    },
    terminal: { v: "yes", note: "CLI only; composing and filtering by arguments" },
    wikilinks: { v: "no" },
    backlinks: { v: "no" },
    tags: {
      v: "yes",
      note: "inline `@tag` (symbol configurable); no nesting",
    },
    cli: {
      v: "yes",
      note: "`jrnl text` writes; `-on`, `-from`, `-contains`, `@tag` filter; `--edit`, `--delete`",
    },
    json: {
      v: "yes",
      note: "`--format json`, plus md, txt, xml, yaml; `--tags` counts",
    },
    liveReload: {
      v: "no",
      note: "no long-running process; each command reads the file",
    },
    vim: {
      v: "partial",
      note: "`--edit` opens entries in the configured external editor",
    },
    mouse: { v: "no" },
    sync: {
      v: "partial",
      note: "journal files can live in Dropbox or any synced folder",
    },
    encryption: {
      v: "yes",
      note: "AES via Fernet; v3 file format with a random per-write salt; single-file journals only",
    },
    mobile: { v: "no" },
    plugins: {
      v: "partial",
      note: "extra export formats can be added through plugins",
    },
    openSource: { v: "yes", note: "GPL-3.0, Python" },
  },
  intro: `jrnl is a journal, not a wiki. You type \`jrnl yesterday: Called in sick. Used the time to clean.\` and it parses the natural-language timestamp, takes the first sentence as the title, and appends an entry to a single text file. Filtering works the same way in reverse: \`jrnl -from "last year" -to march @work\` selects entries by date and tag, and \`--format json\`, \`md\`, \`txt\`, \`xml\` or \`yaml\` reshapes the output for other programs. There is no concept of a note that changes over time; an entry is a dated block of text.

Storage is deliberately minimal. A journal is one file with \`[YYYY-MM-DD HH:MM] Title\` headers, small enough that thousands of entries fit in under a megabyte, and jrnl docs show how to decrypt it manually if the tool disappears. A folder journal (\`YYYY/MM/DD.txt\`) is available for people who want one file per day, at the cost of encryption. Multiple journals (work, personal) are named in the config file and selected by name on the command line.

Encryption has gone through three formats: v1 AES-CBC, v2 Fernet with a static salt, and the current v3 with a random 16-byte salt per write (v1 and v2 files are still readable; see advisory GHSA-rhx6-37mm-5q9r). Editing goes through \`--edit\`, which opens the selected entries in your external editor and writes back only those entries. Tags use \`@\` rather than \`#\` because \`#\` is reserved by most shells.`,
  chooseTool: [
    "You want a dated log, not a graph of notes: one line to write, natural-language dates, chronological output.",
    "Encryption of the whole journal with a password is a hard requirement; jrnl encrypts the file itself and can store the key in the system keychain.",
    "You already think in filters: `-on yesterday`, `-n 10`, `-starred`, `@tag -and @other`.",
    "You want exports in JSON, XML, YAML or Markdown grouped by year and month from the same command.",
    "You keep separate journals per area of life and switch by name (`jrnl work ...`).",
  ],
  chooseTnotes: [
    "You want notes with titles, folders and `[[links]]` that you revisit and rewrite, with backlinks to see what points where.",
    "You want to edit in place with live Markdown highlighting rather than round-tripping through an external editor.",
    "You want one `.md` file per note so any editor, grep or sync tool sees individual documents rather than one growing journal file.",
    "Agents and scripts need to read, search and write individual notes by id with `--json`, and see the result live in a UI.",
    "You want the same tool to be both the interface and the automation layer; tnotes' TUI and CLI share every file.",
  ],
  together: `The two do not compete for the same file. Keep jrnl for the dated log and tnotes for durable notes, and move text across when a journal entry turns into something you will maintain.

\`\`\`sh
# pull yesterday's entries into a new tnotes note
jrnl -on yesterday --format md | tnotes new "Journal 2026-09-21" --tag journal --stdin

# archive a whole tag as one note
jrnl @project-x --format md | tnotes new "Project X journal" --stdin

# the other direction: turn a note into a dated entry (first sentence becomes the title)
tnotes cat weekly-plan | jrnl
\`\`\`

\`jrnl --format md\` groups entries under year and month headings and bumps any existing \`#\` headings down a level, so the output nests cleanly under the title tnotes writes on the first line. \`tnotes new --stdin\` takes the body from stdin and prints the new id; add \`--folder journal\` to keep imports in their own directory. jrnl's \`@tags\` are not tnotes \`#tags\`; a \`sed 's/@\\([a-z]\\)/#\\1/g'\` in the pipe converts them if you want them filterable in tnotes. See [/docs/cli](/docs/cli) and [/glossary/daily-note](/glossary/daily-note).`,
  faq: [
    {
      q: "Is jrnl a note-taking app?",
      a: "It is a journal: append-only dated entries in one file, searched by date and tag. It has no note titles as files, no folders, no links between entries. Use it for a log; use tnotes when notes need names, links and editing over time.",
    },
    {
      q: "Can I export jrnl to Markdown files?",
      a: "`jrnl --format md` prints all (or filtered) entries as one Markdown document grouped by year and month; `--file` writes it to a path, and `--format yaml --file dir/` writes one file per entry. Piping the Markdown into `tnotes new --stdin` creates a note from it.",
    },
    {
      q: "Does jrnl support Vim or a TUI?",
      a: "No TUI. `jrnl` without arguments prompts for an entry on the command line, and `--edit` opens matching entries in whatever editor is set in the config, Vim included. tnotes has a built-in editor with Vim or Emacs keys.",
    },
  ],
  verifiedOn: "2026-09-22",
  sources: [
    { label: "jrnl docs: Overview", url: "https://jrnl.sh/en/stable/overview/" },
    { label: "jrnl docs: Basic Usage", url: "https://jrnl.sh/en/stable/usage/" },
    { label: "jrnl docs: Formats", url: "https://jrnl.sh/en/stable/formats/" },
    {
      label: "jrnl docs: Journal Types",
      url: "https://jrnl.sh/en/stable/journal-types/",
    },
    {
      label: "jrnl docs: Encryption",
      url: "https://jrnl.sh/en/stable/encryption/",
    },
    { label: "jrnl README (GPL-3.0)", url: "https://github.com/jrnl-org/jrnl" },
  ],
};
