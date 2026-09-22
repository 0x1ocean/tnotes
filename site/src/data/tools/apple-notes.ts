import type { Tool } from "../types";

export const tool: Tool = {
  slug: "apple-notes",
  name: "Apple Notes",
  url: "https://support.apple.com/guide/notes/welcome/mac",
  tagline:
    "Apple's built-in rich-text notes app, synced through iCloud across iPhone, iPad, Mac and the web; zero setup, no files you can point a tool at.",
  category: "desktop app",
  platforms: "macOS, iOS, iPadOS, visionOS; icloud.com in a browser. No Linux or Windows app",
  license: "Proprietary, bundled with Apple devices",
  pricing: "Free with the device; iCloud storage beyond 5 GB is paid",
  features: {
    storage: {
      v: "no",
      note: "private database; per-note Markdown/PDF export on macOS 26+",
    },
    terminal: { v: "no" },
    wikilinks: {
      v: "partial",
      note: "type >> to link another note; stored as a rich link, no [[ ]] text",
    },
    backlinks: { v: "no", note: "note links are one-way; no backlinks list in the guide" },
    tags: { v: "yes", note: "#tag anywhere in the text; single word, hyphens and underscores" },
    cli: { v: "no", note: "AppleScript and Shortcuts only" },
    json: { v: "no" },
    liveReload: { v: "no", note: "no files to watch" },
    vim: { v: "no" },
    mouse: { v: "yes", note: "native macOS app; Apple Pencil and drawing on iPad" },
    sync: { v: "yes", note: "iCloud; also IMAP-based notes from other accounts" },
    encryption: {
      v: "yes",
      note: "locked notes are E2EE (AES-GCM); Advanced Data Protection covers all notes",
    },
    mobile: { v: "yes", note: "iPhone and iPad; Quick Note, scanning, audio transcription" },
    plugins: { v: "no" },
    openSource: { v: "no" },
  },
  intro: `Apple Notes is the app that is already on the device. It stores rich text with attachments, checklists, tables, scanned documents, drawings, and since recent releases audio recordings with transcripts and inline math. Notes sync through iCloud without configuration, a note can be locked with a password or Touch ID/Face ID, and with Advanced Data Protection turned on the whole Notes dataset is end-to-end encrypted so Apple cannot read it either.

Organisation is folders, smart folders, and \`#tags\` typed anywhere in the text. In current versions you can link one note to another by typing \`>>\` and picking a title; the link keeps following the note if it is renamed. macOS Tahoe 26 added \`File > Export To > Markdown\` and \`File > Import Markdown\`, so text can leave and enter the app without going through PDF. There is no file you can open in another editor: the data lives in a database inside the app's container and iCloud.

tnotes is the inverse: only text, only \`.md\` files, only on macOS and Linux, in a terminal. It has \`[[links]]\` with backlinks, \`#tags\` including nested ones, and a CLI that scripts and agents drive. It has no attachments, no drawings, no phone app. People who use both keep Apple Notes for capture on the phone and move durable text into a folder.`,
  chooseTool: [
    "You capture on an iPhone or iPad: camera, scans, Apple Pencil, Siri, share sheet, lock screen Quick Note.",
    "Attachments, tables, checklists and drawings belong inside the note, not next to it.",
    "You want end-to-end encryption that is on by default for locked notes and one switch (Advanced Data Protection) for everything else.",
    "Zero setup and shared notes with other Apple users matter more than portability.",
  ],
  chooseTnotes: [
    "You want your notes as files: `git log`, `grep`, `rsync`, any editor, and no export step to leave.",
    "You need a headless interface: `tnotes search --json`, `cat`, `new`, `append` for scripts and AI agents. Apple Notes offers AppleScript on macOS only.",
    "You work on Linux or over SSH, where Apple Notes does not exist.",
    "You want `[[links]]` with backlinks and nested `#work/project` tags in plain text rather than rich links and flat tags.",
  ],
  together: `tnotes cannot read Apple Notes' database, and Apple Notes cannot watch a folder. What works is a one-way export into a tnotes root.

**macOS 26 (Tahoe) or later.** Select a note and choose \`File > Export To > Markdown\`; save into your notes folder. The guide documents one note at a time; multi-selection has been reported to work in recent builds but is not documented, so check before relying on it.

\`\`\`sh
mkdir -p ~/Documents/notes/apple
# export from Notes.app into that folder, then
tnotes ~/Documents/notes/apple
tnotes ls --folder apple --json | jq '.[].title'
\`\`\`

**Older macOS, or bulk.** Notes has no bulk Markdown export before Tahoe, but its AppleScript dictionary exposes each note's \`body\` as HTML. Open-source exporters such as \`apple-notes-exporter\` and Mac App Store apps (\`Exporter\`, \`Notes Exporter\`) walk the accounts and write HTML or Markdown files, with attachments beside them. From HTML, \`pandoc\` finishes the job:

\`\`\`sh
brew install pandoc
for f in ~/Documents/notes/apple/*.html; do
  pandoc -f html -t gfm --wrap=none "$f" -o "\${f%.html}.md" && rm "$f"
done
\`\`\`

Limits to know:

- One way only. Editing the \`.md\` in tnotes does not change the note in Apple Notes; re-exporting overwrites your edits unless you export to a separate folder.
- \`#tags\` are inline text in Apple Notes and survive as text, so tnotes picks them up. \`>>\` note links do not become \`[[links]]\`; they export as plain text or a Markdown link, depending on the exporter.
- Expect locked notes to need unlocking before an exporter can read them; they are encrypted with your passphrase, not the account key.
- tnotes takes the title from the first line and names the file after it; an export whose first line is the note title yields sensible file names.
- Images, scans, audio and drawings are files next to the Markdown at best; tnotes lists only \`.md\`.

The reverse direction is also one way: \`File > Import Markdown\` on macOS 26 imports a folder of \`.md\` files (with an option to preserve the folder structure) into an \`Imported Notes\` folder.`,
  faq: [
    {
      q: "Can I export all my Apple Notes to Markdown?",
      a: "On macOS 26 Tahoe, `File > Export To > Markdown` exports the selected note; Apple's guide describes it per note. For everything at once, use an AppleScript-based exporter (they read each note's `body` as HTML) or a Mac App Store exporter, then convert with `pandoc` if you got HTML. Earlier macOS versions can only export PDF natively.",
    },
    {
      q: "Where does Apple Notes store its files on Mac?",
      a: "Not as documents. Notes keeps a private database inside its app container and syncs it through iCloud; there is no folder of note files to open in another editor. Get text out with `File > Export To`, `Edit > Copy as Markdown`, or AppleScript, and keep it in a directory that tools like tnotes can read.",
    },
    {
      q: "Are Apple Notes end-to-end encrypted?",
      a: "Locked notes are: a key derived from your passphrase with PBKDF2 encrypts the note and its attachments with AES-GCM, and you unlock with the passphrase or Touch ID/Face ID. Unlocked notes in iCloud are encrypted at rest but Apple holds the keys, unless you turn on Advanced Data Protection, which makes Notes end-to-end encrypted as a whole.",
    },
  ],
  verifiedOn: "2026-09-22",
  sources: [
    { label: "Apple Notes User Guide: Import, export, and print notes on Mac", url: "https://support.apple.com/guide/notes/import-export-and-print-notes-not201900c07/mac" },
    { label: "Apple Notes User Guide: Add links in Notes on Mac", url: "https://support.apple.com/guide/notes/add-links-apde615d29c2/mac" },
    { label: "Apple Notes User Guide: Use tags in Notes on Mac", url: "https://support.apple.com/guide/notes/use-tags-apdc88ed7f1d/mac" },
    { label: "Apple Platform Security: Secure features in the Notes app", url: "https://support.apple.com/guide/security/secure-features-in-the-notes-app-sec1782bcab1/web" },
  ],
};
