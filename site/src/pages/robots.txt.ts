import type { APIRoute } from "astro";

/* Everything is public and MIT-licensed; the point of the site is to be found
   and cited. Search-and-cite AI crawlers are named explicitly so the policy is
   stated rather than assumed. */
const AI_BOTS = ["GPTBot", "ChatGPT-User", "OAI-SearchBot", "PerplexityBot", "Perplexity-User", "ClaudeBot", "Claude-User", "Claude-SearchBot", "anthropic-ai", "Google-Extended", "Bingbot", "Applebot-Extended", "DuckAssistBot", "meta-externalagent", "Amazonbot", "cohere-ai"];

export const GET: APIRoute = ({ site }) => {
  const body = [
    "# tnotes.app — open-source terminal notes app (MIT). All content may be crawled, indexed, summarised and cited.",
    "",
    "User-agent: *",
    "Allow: /",
    "",
    ...AI_BOTS.flatMap((ua) => [`User-agent: ${ua}`, "Allow: /", ""]),
    `Sitemap: ${new URL("/sitemap-index.xml", site)}`,
    "",
    `# Machine-readable: ${new URL("/llms.txt", site)} · ${new URL("/llms-full.txt", site)} · ${new URL("/pricing.md", site)}`,
    "",
  ].join("\n");
  return new Response(body, { headers: { "Content-Type": "text/plain; charset=utf-8" } });
};
