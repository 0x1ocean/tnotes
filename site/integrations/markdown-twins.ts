import { readFile, writeFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import type { AstroIntegration } from "astro";
import TurndownService from "turndown";
// @ts-expect-error no types shipped
import { gfm } from "turndown-plugin-gfm";

/**
 * After the static build, writes a Markdown twin next to every HTML page
 * (`/docs/cli` → `/docs/cli.md`, `/` → `/index.md`) and concatenates them into
 * `/llms-full.txt`. The edge middleware serves the twin when a client sends
 * `Accept: text/markdown`, so agents get the page's real content at the
 * canonical URL without rendering HTML. Source of truth stays the HTML: the
 * twin is derived, never hand-maintained.
 */
export default function markdownTwins(): AstroIntegration {
  return {
    name: "markdown-twins",
    hooks: {
      "astro:build:done": async ({ dir, pages, logger }) => {
        const root = fileURLToPath(dir);
        const td = new TurndownService({ headingStyle: "atx", codeBlockStyle: "fenced", bulletListMarker: "-" });
        td.use(gfm);
        td.remove(["script", "style", "nav", "footer", "header", "button"]);
        // The fake TUI, ASCII logo and feature pictograms are decoration; keep the alt text only.
        td.addRule("decorative", {
          filter: (node) => node.getAttribute?.("aria-hidden") === "true",
          replacement: () => "",
        });
        td.addRule("figure-img", {
          filter: (node) => node.nodeName === "FIGURE" && node.getAttribute("role") === "img",
          replacement: (_c, node) => `\n\n_${(node as Element).getAttribute("aria-label") ?? ""}_\n\n`,
        });

        const site = "https://tnotes.app";
        const full: string[] = [];
        let count = 0;
        for (const { pathname } of pages) {
          if (pathname === "404") continue;
          const clean = pathname.replace(/\/$/, "");
          const htmlPath = `${root}${clean ? clean + "/" : ""}index.html`;
          const html = await readFile(htmlPath, "utf8").catch(() => null);
          if (!html) continue;
          const title = html.match(/<title>([^<]*)<\/title>/)?.[1] ?? "";
          const description = html.match(/name="description" content="([^"]*)"/)?.[1] ?? "";
          const main = html.match(/<main[^>]*>([\s\S]*?)<\/main>/)?.[1] ?? "";
          const body = td.turndown(main).replace(/\n{3,}/g, "\n\n").trim();
          const url = `${site}/${clean}`;
          const md = `---\ntitle: "${title.replace(/"/g, '\\"')}"\ndescription: "${description.replace(/"/g, '\\"')}"\ncanonical: ${url}\n---\n\n${body}\n`;
          await writeFile(`${root}${clean || "index"}.md`, md);
          full.push(`<!-- ${url} -->\n# ${title}\n\n${body}`);
          count++;
        }
        await writeFile(`${root}llms-full.txt`, `# tnotes.app — full site content\n\n${full.join("\n\n---\n\n")}\n`);
        logger.info(`wrote ${count} Markdown twins and llms-full.txt`);
      },
    },
  };
}
