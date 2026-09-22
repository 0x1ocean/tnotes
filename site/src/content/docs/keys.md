---
title: Keyboard and mouse
description: Every tnotes keybinding for the list, tree and editor, in emacs and vim modes, plus mouse actions.
order: 2
---

The in-app `?` page (`F1`) is the source of truth; this table mirrors it for the current release.

## Global

| Action | Keys |
|---|---|
| new note | `ctrl+t` |
| preview / edit | `ctrl+l` |
| close tab | `ctrl+w` |
| next tab | `alt+.` · `ctrl+x` · `ctrl+pgdn` · `alt+→` |
| previous tab | `alt+,` · `ctrl+pgup` · `alt+←` |
| jump to tab | `1-9` |
| sidebar / panel | `ctrl+b` |
| next pane | `tab` |
| previous pane | `shift+tab` |
| settings | `F2` |
| help | `F1` |
| quit | `ctrl+q` · `ctrl+c` (list/tree: `q`) |

## List

| Action | Keys |
|---|---|
| move | `j/k` `↑/↓` |
| edit (pins the tab) | `enter` / `l` |
| search | `/` |
| trash (in trash: delete) | `d` |
| undo trash | `u` |
| restore (trash) | `r` |
| move to folder | `m` |
| sort | `s` |
| top / bottom | `gg` / `G` |
| help / settings | `?` `,` |

## Tree

| Action | Keys |
|---|---|
| filter | `enter` |
| fold | `space` |
| new / rename / delete folder | `N` / `R` / `D` |
| list ← · filter → list | `h` / `l` |

## Editor

| Action | Keys |
|---|---|
| back to list | `esc` |
| complete tag | `#…` `tab` |
| complete link | `[[…` `tab` |
| follow [[link]] · backlinks | `alt+enter` |
| continue list · empty item ends it | `enter` |
| toggle task (or click the box) | `alt+x` |
| nest / un-nest list item | `tab` / `shift+tab` |
| #tag → filter · [[link]] → open | `ctrl+click` |

### Emacs keys (default)

| Action | Keys |
|---|---|
| find in note | `ctrl+s` |
| undo / redo | `ctrl+u` / `ctrl+r` |
| word back / forward | `alt+b` / `alt+f` |

### Vim keys

Set `editor.keys = "vim"` in the [config](/docs/config) or on the settings page.

| Action | Keys |
|---|---|
| insert | `i` `a` `I` `A` `o` `O` |
| normal · then back to list | `esc` |
| undo / redo / repeat | `u` · `ctrl+r` · `.` |
| search | `/` `n` `N` |

## Mouse

| Action | Effect |
|---|---|
| click / double-click | select · pin |
| right-click | context menu |
| wheel / drag | scroll · select text (copied on release) |
| shift+drag | the terminal's own selection, bypassing tnotes |
| click a checkbox | toggle it |
| ctrl+click a `#tag` / `[[link]]` | filter by tag · open the note |
