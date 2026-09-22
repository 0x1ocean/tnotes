import { next, rewrite } from "@vercel/functions/middleware";

/**
 * Markdown content negotiation for a static site. `Accept: text/markdown`
 * gets the page's Markdown twin (generated at build by integrations/markdown-twins.ts)
 * at the same URL; everyone else gets HTML. `Vary: Accept` keeps the two
 * apart in caches. Assets and generated text files are passed through.
 */
export const config = {
  matcher: ["/((?!_astro|_vercel|favicon\\.svg|og\\.png|.*\\.(?:md|txt|xml|png|svg|woff2)$).*)"],
};

export default async function middleware(request: Request) {
  const url = new URL(request.url);
  const path = url.pathname.replace(/\/$/, "") || "/index";
  const twin = `${path}.md`;
  const accept = request.headers.get("accept") ?? "";
  const wantsMarkdown = accept.includes("text/markdown") && !accept.includes("text/html");
  if (!wantsMarkdown) {
    return next({ headers: { Vary: "Accept", Link: `<${twin}>; rel="alternate"; type="text/markdown"` } });
  }
  url.pathname = twin;
  /* A twin exists for every real page; anything else is a 404 and gets the
     Markdown 404 body (with a real 404 status via the /404.md rewrite below). */
  const exists = (await fetch(url, { method: "HEAD" })).ok;
  if (!exists) url.pathname = "/404.md";
  return rewrite(url, { headers: { Vary: "Accept" }, ...(exists ? {} : { status: 404 }) });
}
