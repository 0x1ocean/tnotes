# vhs docs/demo/demo.tape  (run via docs/demo/record.sh)
Output docs/demo.gif

Set Shell "bash"
Set FontFamily "JetBrains Mono"
Set FontSize 15
Set Width 1200
Set Height 640
Set Theme "Nord"
Set Padding 12
Set TypingSpeed 40ms
Set Framerate 30

# --- TUI: capture a note, link it, tag it ---
Type "tnotes"
Enter
Sleep 1.2s

Ctrl+t
Sleep 400ms
Type "Weekly plan"
Enter
Enter
Type "#work #planning"
Enter
Enter
Type "- [ ] Ship tnotes 1.3 "
Type "[[tno"
Sleep 700ms
Tab
Sleep 500ms
Enter
Type "- [ ] Prep "
Type "[[Mee"
Sleep 700ms
Tab
Sleep 500ms
Enter
Type "- [ ] Book flights for Berlin #travel"
Sleep 1.5s

# follow the link, show backlinks come for free
Up
Up
Alt+Enter
Sleep 1.5s
Escape
Sleep 300ms
Ctrl+q
Sleep 600ms

# --- CLI: the same vault, as JSON, for scripts and agents ---
Type "clear"
Enter
Type "tnotes ls --json | jq '.[0] | {id, title, tags, links, backlinks}'"
Enter
Sleep 2.5s

Type "tnotes append weekly-plan '- [ ] call Ann about the venue'"
Enter
Sleep 800ms
Type "tnotes cat weekly-plan"
Enter
Sleep 2.5s

Type "tnotes search venue --json | jq -r '.[0].path'"
Enter
Sleep 2s
