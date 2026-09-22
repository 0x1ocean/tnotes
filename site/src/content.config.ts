import { defineCollection, z } from "astro:content";
import { glob } from "astro/loaders";

const docs = defineCollection({
  loader: glob({ pattern: "*.{md,mdx}", base: "./src/content/docs" }),
  schema: z.object({
    title: z.string(),
    description: z.string(),
    order: z.number(),
  }),
});

/** The crate's own CHANGELOG.md, rendered as-is: one source of truth. */
const changelog = defineCollection({
  loader: glob({ pattern: "CHANGELOG.md", base: ".." }),
});

export const collections = { docs, changelog };
