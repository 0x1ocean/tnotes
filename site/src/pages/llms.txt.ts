import type { APIRoute } from "astro";
import { getCollection } from "astro:content";
import { DESCRIPTION, REPO, VERSION } from "@/lib/meta";
import { TOOLS } from "@/data/tools";
import { GLOSSARY } from "@/data/glossary";

/** https://llmstxt.org — a plain-text map of the site for AI crawlers,
 *  with the raw Markdown of every doc appended so one fetch is enough. */
export const GET: APIRoute = async ({ site }) => {
  const docs = (await getCollection("docs")).sort((a, b) => a.data.order - b.data.order);
  const usecases = (await getCollection("usecases")).sort((a, b) => a.data.order - b.data.order);
  const guides = (await getCollection("guides")).sort((a, b) => a.data.order - b.data.order);
  const line = (href: string, title: string, desc: string) => `- [${title}](${new URL(href, site)}): ${desc}`;
  const index = docs.map((d) => line(`/docs/${d.id}`, d.data.title, d.data.description)).join("\n");
  const compare = TOOLS.map((t) => line(`/compare/tnotes-vs-${t.slug}`, `tnotes vs ${t.name}`, t.tagline)).join("\n");
  const forList = usecases.map((e) => line(`/for/${e.id}`, e.data.title, e.data.description)).join("\n");
  const guideList = guides.map((e) => line(`/guides/${e.id}`, e.data.title, e.data.description)).join("\n");
  const glossary = GLOSSARY.map((e) => line(`/glossary/${e.slug}`, e.term, e.short)).join("\n");
  const bodies = docs.map((d) => `\n---\n\n# ${d.data.title}\n\n${d.body ?? ""}`).join("\n");

  const text = `# tnotes

> ${DESCRIPTION}. Version ${VERSION}. MIT. Source: ${REPO}

tnotes is a Rust terminal application (ratatui + edtui). Notes are plain .md files; folders are directories; #tags and [[links]] are parsed from the text. A headless CLI (ls, search, cat, new, append, write, trash, restore; all with --json) lets scripts and AI agents work on the same files while the TUI is open.

## Docs

${index}
- [Use with AI agents](${new URL("/agents", site)}): recipes for driving the vault from Claude Code, Codex, cron and shell.
- [Changelog](${new URL("/changelog", site)}): every release.

## Comparisons

${compare}

## Use cases

${forList}

## Guides

${guideList}

## Glossary

${glossary}

## Install

brew install 0x1ocean/tnotes/tnotes
cargo install tnotes
${bodies}
`;
  return new Response(text, { headers: { "Content-Type": "text/plain; charset=utf-8" } });
};
