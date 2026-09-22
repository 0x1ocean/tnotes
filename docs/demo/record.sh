#!/usr/bin/env sh
# Records docs/demo.gif with vhs (https://github.com/charmbracelet/vhs).
# Uses a throwaway vault and XDG dirs so your real config/state are untouched.
set -eu
cd "$(dirname "$0")/../.."
cargo build --release --quiet

DEMO=$(mktemp -d)
export DEMO
export XDG_CONFIG_HOME="$DEMO/config" XDG_STATE_HOME="$DEMO/state"
export PATH="$PWD/target/release:$PATH"
mkdir -p "$DEMO/vault/projects" "$XDG_CONFIG_HOME/tnotes"

cat > "$XDG_CONFIG_HOME/tnotes/config.toml" <<EOF
roots = ["$DEMO/vault"]
[appearance]
sidebar_width = 28
EOF

cat > "$DEMO/vault/tnotes-roadmap.md" <<'EOF'
# tnotes roadmap

#tnotes #work/tnotes

- [x] headless CLI with --json
- [x] [[Meeting notes]] backlinks overlay
- [ ] homebrew-core
EOF

cat > "$DEMO/vault/meeting-notes.md" <<'EOF'
# Meeting notes

#work

- [ ] send the slides
- [ ] follow up on [[tnotes roadmap]]
EOF

cat > "$DEMO/vault/projects/reading-list.md" <<'EOF'
# Reading list

#books

- Designing Data-Intensive Applications
- The Pragmatic Programmer
EOF

vhs docs/demo/demo.tape
rm -rf "$DEMO"
