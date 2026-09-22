import type { Cell, FeatureKey } from "./types";

/** tnotes' own column in every comparison. Facts from README.md / CHANGELOG.md. */
export const TNOTES: Record<FeatureKey, Cell> = {
  storage: { v: "yes", note: "plain .md files, folders are directories" },
  terminal: { v: "yes", note: "TUI (ratatui) plus a headless CLI" },
  wikilinks: { v: "yes", note: "[[Title]] with completion; rewritten on rename" },
  backlinks: { v: "yes", note: "overlay + `backlinks` in JSON" },
  tags: { v: "yes", note: "#tag and nested #work/project, parsed from text" },
  cli: { v: "yes", note: "ls, search, cat, new, append, write, trash, restore" },
  json: { v: "yes", note: "--json on every command" },
  liveReload: { v: "yes", note: "file watcher; dirty tabs keep their text" },
  vim: { v: "yes", note: "vim or emacs keys (edtui)" },
  mouse: { v: "yes", note: "click, double-click, right-click menus, drag" },
  sync: { v: "partial", note: "any file sync: git, Syncthing, iCloud, Dropbox" },
  encryption: { v: "partial", note: "encrypt the folder: FileVault, gocryptfs" },
  mobile: { v: "no", note: "files sync to any mobile Markdown editor" },
  plugins: { v: "no" },
  openSource: { v: "yes", note: "MIT, Rust" },
};
