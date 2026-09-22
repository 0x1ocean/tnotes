import rss from "@astrojs/rss";
import type { APIRoute } from "astro";
import { getCollection } from "astro:content";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { md } from "@/lib/markdown";
import { DESCRIPTION, REPO } from "@/lib/meta";

/** Releases (one item per `## [x.y.z] - date` section of CHANGELOG.md) plus
 *  guides, newest first. Unreleased changes are not items. */
export const GET: APIRoute = async ({ site }) => {
  const changelog = readFileSync(resolve("../CHANGELOG.md"), "utf8");
  const releases = [...changelog.matchAll(/^## \[(\d+\.\d+\.\d+)\] - (\d{4}-\d{2}-\d{2})\n([\s\S]*?)(?=^## \[|^\[)/gm)];
  const releaseItems = await Promise.all(
    releases.map(async ([, version, date, body]) => ({
      title: `tnotes ${version}`,
      link: `${REPO}/releases/tag/v${version}`,
      pubDate: new Date(date),
      description: `Release notes for tnotes ${version}.`,
      content: await md(body.trim()),
    })),
  );
  const guides = await getCollection("guides");
  const guideItems = await Promise.all(
    guides.map(async (g) => ({
      title: g.data.title,
      link: `/guides/${g.id}`,
      pubDate: g.data.published,
      description: g.data.description,
      content: await md(g.body ?? ""),
    })),
  );
  const items = [...releaseItems, ...guideItems].sort((a, b) => b.pubDate.getTime() - a.pubDate.getTime());
  return rss({
    title: "tnotes",
    description: `${DESCRIPTION}. Releases and guides.`,
    site: site!,
    items,
    customData: "<language>en</language>",
  });
};
