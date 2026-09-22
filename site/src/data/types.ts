/** A yes/no/partial cell in a comparison matrix, with an optional one-line note. */
export type Cell = { v: "yes" | "no" | "partial"; note?: string };

/** Glyph and colour class for each cell value; shared by the matrix and the hub table. */
export const MARK: Record<Cell["v"], readonly [string, string]> = {
  yes: ["✓", "text-ok"],
  partial: ["~", "text-warn"],
  no: ["✗", "text-dim"],
};

/** Feature rows shared by every comparison page. Order = table order. */
export const FEATURES = [
  ["storage", "Storage format"],
  ["terminal", "Runs in the terminal"],
  ["wikilinks", "[[Wikilinks]]"],
  ["backlinks", "Backlinks"],
  ["tags", "#tags"],
  ["cli", "Headless CLI"],
  ["json", "JSON output for scripts / agents"],
  ["liveReload", "Live reload on external edits"],
  ["vim", "Vim keys"],
  ["mouse", "Mouse support"],
  ["sync", "Sync"],
  ["encryption", "Encryption"],
  ["mobile", "Mobile app"],
  ["plugins", "Plugins"],
  ["openSource", "Open source"],
] as const;

export type FeatureKey = (typeof FEATURES)[number][0];

export type Tool = {
  slug: string;
  name: string;
  url: string;
  /** One sentence: what it is, for whom. */
  tagline: string;
  category: "desktop app" | "terminal app" | "CLI" | "editor plugin" | "web app";
  platforms: string;
  license: string;
  pricing: string;
  features: Record<FeatureKey, Cell>;
  /** 2–3 short Markdown paragraphs. Must be specific to this tool: what it does
   *  differently, who it is really for, what changed recently. Never templated. */
  intro: string;
  /** Bullet points, Markdown inline allowed. 3–5 each. */
  chooseTool: string[];
  chooseTnotes: string[];
  /** Markdown with a code block: a concrete recipe for using both. Omit only if
   *  there is genuinely no overlap. */
  together?: string;
  faq: { q: string; a: string }[];
  /** ISO date the facts were checked against the sources below. */
  verifiedOn: string;
  sources: { label: string; url: string }[];
};

export type GlossaryEntry = {
  slug: string;
  term: string;
  /** ≤160 chars, a complete definition on its own (used as meta description and DefinedTerm). */
  short: string;
  /** Markdown, 200–400 words: origin, how it works, common pitfalls. */
  body: string;
  /** Markdown, 60–150 words: how tnotes implements or uses it, with a key or command. */
  inTnotes: string;
  related: string[];
};
