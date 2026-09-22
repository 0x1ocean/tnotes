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

const usecases = defineCollection({
  loader: glob({ pattern: "*.md", base: "./src/content/usecases" }),
  schema: z.object({
    title: z.string(),
    description: z.string(),
    persona: z.string(),
    order: z.number(),
    keywords: z.array(z.string()),
  }),
});

const guides = defineCollection({
  loader: glob({ pattern: "*.md", base: "./src/content/guides" }),
  schema: z.object({
    title: z.string(),
    description: z.string(),
    published: z.coerce.date(),
    order: z.number(),
    keywords: z.array(z.string()),
  }),
});

export const collections = { docs, usecases, guides };

