import { createMarkdownProcessor } from "@astrojs/markdown-remark";

/** Renders Markdown held in data files (comparison intros, glossary bodies).
 *  Same shiki theme as content collections so code blocks match. */
const processor = await createMarkdownProcessor({ shikiConfig: { theme: "nord" } });

export async function md(source: string): Promise<string> {
  return (await processor.render(source)).code;
}
