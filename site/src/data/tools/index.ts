import type { Tool } from "../types";

/** Every comparison page. Order = hub order and "related" order. Files are
 *  loaded eagerly so a missing or malformed one fails the build, not a page. */
const modules = import.meta.glob<{ tool: Tool }>("./*.ts", { eager: true });

export const TOOLS: Tool[] = Object.entries(modules)
  .filter(([path]) => !path.endsWith("/index.ts"))
  .map(([, m]) => m.tool)
  .sort((a, b) => a.name.localeCompare(b.name));
