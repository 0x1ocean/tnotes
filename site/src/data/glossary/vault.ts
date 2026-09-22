import type { GlossaryEntry } from "../types";

export const entry: GlossaryEntry = {
  slug: "vault",
  term: "Vault",
  short:
    "A vault is the folder on disk that holds a set of notes; the tool indexes it, links resolve inside it, and its subfolders are the note hierarchy.",
  body: `A vault is the root folder of a note collection. Everything under it is the tool's universe: files are notes, subdirectories are folders, and a \`[[link]]\` is looked up among the titles inside that root and nowhere else.

## Origin

The word is Obsidian's. Its help defines a vault as "a folder on your local file system where Obsidian stores your notes", possibly with subfolders, and says you can keep one vault or several ([Obsidian: Create a vault](https://help.obsidian.md/vault)). The concept is older and goes by other names: a vimwiki *wiki* is a path in \`g:vimwiki_list\` ([vimwiki README](https://github.com/vimwiki/vimwiki)); nb calls it a *notebook*; Dendron says *vault* too, "a git backed folder for your notes" ([Dendron README](https://github.com/dendronhq/dendron)); Joplin's notebooks are database records, not directories. What they share is a boundary: an index is built over one tree, and identity (titles, links, tags) is unique within it.

## How it works

The tool walks the tree at startup, indexes titles, links and tags, and watches for changes. Anything it needs to remember about the collection that is not a note goes in a hidden folder at the root; Obsidian writes \`.obsidian/\` with per-vault hotkeys, themes and plugins, and its docs recommend excluding \`workspace.json\` from git because it changes on every open ([Obsidian: How Obsidian stores data](https://help.obsidian.md/data-storage)). Multiple vaults are separate indexes: switching is a context switch, and a link cannot cross the boundary.

## Pitfalls

Nesting a vault inside another vault confuses both indexes; Obsidian warns against it because links may not update correctly. The hidden config folder is the part that does not travel: sync it and you carry plugin state between machines, ignore it and each machine has its own settings. Sync clients treat the whole tree as data, so a tool that writes caches or lock files next to notes creates conflicts that have nothing to do with your writing.

## In other tools

Obsidian: one folder, \`.obsidian/\` inside it, and as many vaults as you like, each with its own settings. Logseq: a *graph*; the file-based version is a folder, and the newer DB version keeps the graph in SQLite ([Logseq README](https://github.com/logseq/logseq)). nb: notebooks are git repositories under \`~/.nb\` by default, or any folder initialised as a local notebook, switched with \`nb use\` ([nb README](https://github.com/xwmx/nb)). Joplin: notebooks live in SQLite, with the folder-like hierarchy stored as records rather than directories ([Joplin FAQ](https://joplinapp.org/help/faq)).`,
  inTnotes: `tnotes calls a vault a **root**. \`roots\` in \`~/.config/tnotes/config.toml\` is a list, so several folders can be open at once, and a note's id, \`root/sub/stem\`, names which one it belongs to.

\`\`\`sh
tnotes ~/work/notes      # add a root to the config and open it
tnotes --dir ~/tmp/notes # this folder only, config untouched
\`\`\`

The first launch asks where notes live (default \`~/Documents/notes\`). tnotes writes nothing inside a root except notes and a per-root \`.Trash/\` that mirrors the folder layout; config and session state live under \`~/.config/tnotes\` and \`~/.local/state/tnotes\`. Dot-prefixed entries such as \`.git\` and \`.obsidian\` are ignored, so an Obsidian vault can be added as a root and its config folder stays out of the way. See [/docs/config](/docs/config) and [/for/obsidian-users](/for/obsidian-users).`,
  related: ["plain-text-notes", "wikilink", "backlink"],
};
