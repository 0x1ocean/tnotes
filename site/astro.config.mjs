// @ts-check
import { defineConfig } from "astro/config";
import react from "@astrojs/react";
import mdx from "@astrojs/mdx";
import sitemap from "@astrojs/sitemap";
import tailwindcss from "@tailwindcss/vite";

/** Canonical origin. The only place the domain lives: Seo.astro, sitemap,
 *  robots.txt, llms.txt and JSON-LD all derive from `Astro.site`. */
export const SITE_URL = "https://tnotes.app";

export default defineConfig({
  site: SITE_URL,
  trailingSlash: "never",
  build: { format: "directory" },
  integrations: [react(), mdx(), sitemap()],
  markdown: { shikiConfig: { theme: "nord" } },
  vite: { plugins: [tailwindcss()] },
});
