# tnotes launch plan

Working doc. Not shipped anywhere. Delete after launch.

## Readiness gate (SLC)

- Simple: one thing, Markdown notes in a terminal. Yes.
- Lovable: 1–5 people besides the author use it; mouse-first TUI + `[[links]]` + JSON CLI is a real reason to choose it over `nb`/`jrnl`. Yes.
- Complete: install (brew/cargo/tarball), TUI, CLI, docs site, changelog, tests, CI. No stubs. Yes.

Verdict: ship. Nothing on the roadmap should block the launch. Do not add features before it.

## Situation

- Owned: repo + tnotes.app (45 pages, sitemap, llms.txt). No email list, no blog audience. Fine for OSS: the repo *is* the owned channel; stars and watchers are the list.
- Rented: no HN/Reddit/Mastodon accounts with history. This is the main constraint: a day-old account posting a Show HN is fine (Show HN allows it), a day-old account in r/rust gets auto-filtered.
- Borrowed: this is where the plan leans. awesome-lists, ratatui showcase, This Week in Rust, terminaltrove, awesome-claude-code. All accept PRs/submissions from anyone; each gives a permanent backlink and a burst of qualified traffic.

## Two-week calendar

### Week 0 (now)

| Day | Action | Output |
|---|---|---|
| D0 | Create accounts: HN, Reddit, Mastodon (fosstodon.org), lobste.rs (ask for an invite in #lobsters IRC or from a user). Fill profiles: name, GitHub link, one line about tnotes. | accounts exist |
| D0 | Record demo GIF: `docs/demo/record.sh`. Put it at the top of README under the screenshot. Push. | `docs/demo.gif` |
| D0 | `homepage = "https://tnotes.app"` in Cargo.toml (done), site links in README (done). Add repo topics on GitHub: `tui`, `notes`, `markdown`, `note-taking`, `ratatui`, `cli`, `wikilinks`, `obsidian`, `ai-agents`, `rust`. Set the repo website field to tnotes.app. | GitHub metadata |
| D1 | Search Console + Bing Webmaster: verify tnotes.app, submit `sitemap-index.xml`. | indexing starts |
| D1–D3 | Spend 15 min/day on HN and r/rust *commenting* on others' posts (real comments, not tnotes). Karma is not the goal; account age with activity is. | accounts warm |
| D2 | Borrowed-channel PRs (below). Spread over 2 days so they don't look like a bot. | 5–7 PRs open |
| D3 | Submit to terminaltrove.com and to ratatui showcase (`ratatui/ratatui` discussions or their showcase repo). | listed |
| D4 | Tag `v1.4.0` if there is anything in `[Unreleased]` worth a release note. A fresh release on launch day reads better than "last release 2 weeks ago". If nothing: skip. | release |

### Week 1

| Day | Action |
|---|---|
| Tue or Wed, 08:00–09:30 ET | **Show HN**. Post from a warmed account. Post the *repo* URL, not the site (HN readers prefer it; the README now links to the site). First comment ready (below). Stay at the keyboard for 6 hours. Answer every comment in the first 2 hours; that decides whether it stays on the front page. |
| same day, +2h | Mastodon post with the GIF, tag `@ratatui_rs@fosstodon.org` if they exist there, `#rustlang #tui #markdown`. |
| same day, +4h | r/rust "project" flair post. Different text from HN: r/rust wants the Rust angle (ratatui, edtui fork, musl static, why Rust). |
| +1 day | r/commandline, r/PKMS. Not r/ObsidianMD yet. |
| +2 days | This Week in Rust PR to `rust-lang/this-week-in-rust`: one line under "Project Updates" or "Newsletters". Deadline is usually Tuesday; check the issue template. |
| +3 days | lobste.rs (if invited). Tag `rust`, `programming`. lobste.rs punishes self-promo tone; write it as "I built this, here is what was hard". |

### Week 2

| Day | Action |
|---|---|
| +7 days | r/ObsidianMD: "I made a terminal companion for Obsidian vaults" with the `/compare/tnotes-vs-obsidian` and `/for/obsidian-users` pages. Only if week-1 feedback confirms the vault-compat claims hold. |
| +8 days | dev.to / Hashnode cross-post of `/guides/notes-memory-for-claude-code` with canonical URL pointing to tnotes.app. |
| +10 days | Reply to every issue opened during launch. Label 3–5 `good first issue`. |
| +14 days | Retro: stars, site sessions (Vercel Analytics), which channel sent the installs. Decide the next "launch moment" (next release with a headline feature). |

## Borrowed-channel PRs (D2–D3)

Format each PR title as `Add tnotes` and the line in the list's own style. Check the CONTRIBUTING file first; several require alphabetical order and a specific description length.

| List | Repo | Section | Line |
|---|---|---|---|
| awesome-tuis | `rothgar/awesome-tuis` | Productivity or Notes | `- [tnotes](https://github.com/0x1ocean/tnotes) - Markdown notes with folders, #tags, [[links]], a mouse-first TUI and a JSON CLI for scripts and agents.` |
| awesome-ratatui | `ratatui/awesome-ratatui` | Apps → Productivity | same line |
| awesome-cli-apps | `agarrharr/awesome-cli-apps` | Productivity → Notes | `- [tnotes](https://github.com/0x1ocean/tnotes) - Terminal Markdown notes with tags, wikilinks and a JSON CLI.` |
| awesome-rust | `rust-unofficial/awesome-rust` | Applications → Text editors / Productivity | `* [tnotes](https://github.com/0x1ocean/tnotes) - Terminal Markdown notes: TUI + headless JSON CLI.` |
| awesome-markdown | `BubuAnabelas/awesome-markdown` | Editors / Tools | `- [tnotes](https://github.com/0x1ocean/tnotes) - Terminal Markdown notes with `[[wikilinks]]`, `#tags` and a CLI.` |
| awesome-obsidian | `kmaasrud/awesome-obsidian` | Tools / Companions | `- [tnotes](https://github.com/0x1ocean/tnotes) - Open an Obsidian vault from the terminal: same `[[links]]` and `#tags`, plus a JSON CLI.` |
| awesome-claude-code | `hesreallyhim/awesome-claude-code` | Tools / Memory | `- [tnotes](https://github.com/0x1ocean/tnotes) - Give Claude Code a Markdown notes folder as memory via a JSON CLI; no MCP server needed.` |
| terminaltrove | terminaltrove.com/submit | — | form |
| ratatui showcase | `ratatui/ratatui` README "Apps using Ratatui" or awesome-ratatui | — | as above |

Also: AUR `tnotes-bin` (write the PKGBUILD from the release tarball; 30 min) and a `nixpkgs` PR are worth it *after* launch if there are Linux users asking.

## Drafts

### Show HN title (choose one; 80 chars max)

1. `Show HN: tnotes – Markdown notes in the terminal with [[links]], #tags and a JSON CLI`
2. `Show HN: tnotes – a TUI for Markdown notes that AI agents can also drive from the CLI`
3. `Show HN: tnotes – terminal notes app that's also a CLI for scripts and coding agents`

Pick 1. It is the most literal; HN downvotes "for AI agents" in titles when the product is not AI.

### Show HN first comment

> Hi HN, author here.
>
> tnotes is a Rust TUI for Markdown notes: folders are directories, `#tags` and `[[links]]` are parsed from the text, and there's a headless CLI (`tnotes ls | search | cat | new | append | write`, all with `--json`) that works on the same files while the TUI is open.
>
> I built it because I wanted Obsidian's linking model but in a terminal, over SSH, and scriptable. `nb` and `jrnl` are great CLIs but not editors; vimwiki and org-mode are editors but tie you to one editor. I wanted a note app that behaves like a good TUI (mouse works, tabs, live highlighting) and like a good CLI (JSON out, stdin in, stable ids).
>
> Things that were harder than expected:
>
> - Word wrap in the editor. ratatui's edtui wraps by character; I vendored it and added word wrap with cursor mapping. It's published as `edtui-tnotes` so `cargo install` works.
> - Never losing text. Atomic saves, conflict copies instead of overwrites when a file changes under you, and if a save fails (disk full) the text goes to the state dir and you get the path.
> - CLI and TUI on the same vault at once. A clean open tab follows a CLI `append` live; a dirty tab keeps its edits and the CLI write becomes a conflict copy.
>
> The "AI agents" angle is just: the CLI prints JSON with links and backlinks, so `tnotes ls --json | jq` is enough for an agent to walk the graph. No MCP server, no daemon.
>
> Install: `brew install 0x1ocean/tnotes/tnotes` or `cargo install tnotes`. macOS and Linux; Windows untested.
>
> Happy to answer anything about ratatui, the edtui fork, or the file-watching/conflict logic.

Rules for the thread: answer within 10 minutes for the first 2 hours; concede valid criticism immediately ("yes, no frontmatter support, it's on the list"); never argue about Obsidian.

### r/rust post

Title: `tnotes: terminal Markdown notes (ratatui) with a JSON CLI – lessons from vendoring edtui for word wrap`

> I've been building tnotes, a TUI + CLI for Markdown notes, for a few months and just shipped 1.3. Repo: https://github.com/0x1ocean/tnotes
>
> Rust-specific bits people here might care about:
>
> - **ratatui 0.30 + edtui.** edtui wraps by character. I vendored it (`vendor/edtui`, MIT) and added word wrap in `line_wrapper.rs` with cursor mapping and scroll fixes. Published as `edtui-tnotes` on crates.io so `cargo install tnotes` works without git deps. If anyone wants to upstream it, I'd be glad to help.
> - **Fuzzy search with `nucleo-matcher`** over titles and bodies; the same matcher backs the `[[link` completion popup.
> - **File watching with `notify`** and a small reconciliation step so external edits (git pull, Syncthing) reload clean tabs and leave dirty ones alone.
> - **musl static build** in the release workflow: `x86_64-unknown-linux-musl` runs on any distro.
> - **Clipboard on Wayland**: arboard needs the `wayland-data-control` feature or copies silently go to XWayland. Cost me a day.
>
> Headless CLI: `tnotes ls --json`, `search`, `cat`, `new --stdin`, `append`, `write`. Same ids everywhere, so it's scriptable and agents can use it.
>
> Feedback on the edtui changes especially welcome.

### This Week in Rust line

`* [tnotes 1.3: terminal Markdown notes with [[links]], #tags and a JSON CLI](https://github.com/0x1ocean/tnotes/releases/tag/v1.3.0)`

### Mastodon / X

> tnotes: Markdown notes in your terminal.
>
> Folders, #tags, [[links]] with backlinks, tabs, live highlighting, mouse works. And a JSON CLI so scripts (or Claude Code) can read and write the same vault while it's open.
>
> Rust, MIT. brew install 0x1ocean/tnotes/tnotes
>
> https://tnotes.app
>
> #rustlang #tui #markdown #notes
>
> [demo.gif]

### r/ObsidianMD (week 2)

Title: `Terminal companion for your vault: tnotes reads the same [[links]] and #tags`

> Not a replacement. If you SSH into servers or live in tmux, tnotes opens your vault folder as a TUI: same `[[wikilinks]]`, same `#tags`, folders are the same folders. It ignores `.obsidian/`, so nothing in your setup changes.
>
> What carries over: links (resolved by title, then filename), tags including nested ones, folders, checkboxes. What doesn't: plugins, canvas, properties/frontmatter (shown as text), `[[Note#heading]]` and aliases.
>
> Also a CLI: `tnotes ls --json` lists every note with links and backlinks, `tnotes append` adds a line from a script.
>
> Honest comparison: https://tnotes.app/compare/tnotes-vs-obsidian
> Setup for Obsidian users: https://tnotes.app/for/obsidian-users

## What to measure

- GitHub: stars/day, unique cloners (Insights → Traffic), referrers.
- Site: Vercel Web Analytics (enabled on the project; script is injected in production builds only). Watch referrers and `/docs/getting-started` views.
- Installs: crates.io downloads, Homebrew tap has no analytics; GitHub release asset download counts via API.

## Next launch moments

Each release with a headline feature is a mini-launch: changelog → GitHub release → Mastodon post → TWiR line. Candidates: frontmatter support (the most-asked gap on compare pages), homebrew-core formula, AUR package, `[[Note#heading]]` links.
