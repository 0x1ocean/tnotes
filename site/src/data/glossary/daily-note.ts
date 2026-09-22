import type { GlossaryEntry } from "../types";

export const entry: GlossaryEntry = {
  slug: "daily-note",
  term: "Daily note",
  short:
    "A daily note is one note per calendar day, named by date (2026-09-22), used as a running log and inbox from which durable notes are linked out.",
  body: `A daily note is a note whose title is a date. There is one per day, it is the default place to write anything that has no better home, and it is expected to be messy. Durable ideas get moved or linked to their own notes; the daily note keeps the timeline.

## Origin

The pattern is a journal, a lab notebook, or a bullet journal's daily log, moved into a note tool. Outliner-style apps made today's date the landing page, and Obsidian ships it as a core plugin that "opens a note based on today's date, or creates it if it doesn't exist", named \`YYYY-MM-DD\` by default and optionally filled from a template ([Obsidian: Daily notes](https://help.obsidian.md/plugins/daily-notes)). Command-line journals predate all of these: jrnl, around since 2012, appends a timestamped entry from the shell with \`jrnl today at 3am: ...\` and files it chronologically ([jrnl docs](https://jrnl.sh/en/stable/usage/)).

## How it works

The tool needs three things: a date format, a folder, and a way to open today's note in one keystroke, creating it if missing. ISO 8601 (\`2026-09-22\`) is the sensible format because it sorts correctly as a filename and is unambiguous. Templates add a fixed skeleton (tasks, meetings, log). Links from the daily note to topic notes give each topic a dated backlink trail, which is where much of the value is: a project note's backlinks become its history.

## Pitfalls

Daily notes accumulate. A year is 365 files, and unless the log is periodically mined for durable notes it becomes write-only. A folder per year or month keeps the list manageable. Date formats with slashes or month names (\`22/09/2026\`, \`Sep 22\`) sort badly and vary by locale. Tools that link a date property to the daily note (Obsidian does) create implicit links that other tools cannot see. And per-day granularity is a choice: some people want a weekly note instead, and few tools make that switch easy.

## In other tools

Obsidian's daily note is a normal Markdown file that other tools can read, and a \`date\` property in any note becomes a link to that day's note. jrnl is a journal, not a note app: entries are filtered by date or \`@tag\` from the command line and the journal file can be encrypted. In nb, \`nb add --title "$(date +%F)"\` creates one as an ordinary note ([nb README](https://github.com/xwmx/nb)).`,
  inTnotes: `tnotes has no daily-note command, template variable or calendar. A daily note is an ordinary note whose title is the date, and the CLI is enough to make one from a shell alias or keybinding:

\`\`\`sh
tnotes new "$(date +%F)" --tag daily --folder daily
tnotes append "$(date +%F)" "- $(date +%H:%M) call with Ann"
\`\`\`

\`new\` refuses a title that already exists (unless \`--duplicate\`), so the second line is the one to run repeatedly; \`append\` addresses the note by its file stem, \`2026-09-22\`. The TUI picks the change up live. \`tnotes ls --tag daily\` lists the log newest first, and the \`daily\` folder can be filtered in the tree. Dated backlinks work as everywhere: write \`[[Project X]]\` in the day's note and the project's backlinks overlay shows the date. See [/docs/cli](/docs/cli) and [/for/sysadmins](/for/sysadmins).`,
  related: ["tag", "backlink", "plain-text-notes"],
};
