import type { GlossaryEntry } from "../types";

const modules = import.meta.glob<{ entry: GlossaryEntry }>("./*.ts", { eager: true });

export const GLOSSARY: GlossaryEntry[] = Object.entries(modules)
  .filter(([path]) => !path.endsWith("/index.ts"))
  .map(([, m]) => m.entry)
  .sort((a, b) => a.term.localeCompare(b.term));
