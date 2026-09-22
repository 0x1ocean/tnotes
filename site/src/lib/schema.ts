/** JSON-LD node builders shared by pages. Nodes are merged into the
 *  `@graph` that Base.astro emits; `@context` is added there. */

export type Faq = { q: string; a: string };

export function faqPage(faq: Faq[]) {
  return {
    "@type": "FAQPage",
    mainEntity: faq.map(({ q, a }) => ({
      "@type": "Question",
      name: q,
      acceptedAnswer: { "@type": "Answer", text: a.replace(/`/g, "") },
    })),
  };
}

export function breadcrumb(site: URL | undefined, items: { name: string; href: string }[]) {
  return {
    "@type": "BreadcrumbList",
    itemListElement: items.map(({ name, href }, i) => ({
      "@type": "ListItem",
      position: i + 1,
      name,
      item: new URL(href, site).toString(),
    })),
  };
}

/** A hub page: CollectionPage whose mainEntity is an ItemList of its child pages. */
export function collection(site: URL | undefined, name: string, description: string, items: { name: string; href: string }[]) {
  return {
    "@type": "CollectionPage",
    name,
    description,
    mainEntity: {
      "@type": "ItemList",
      numberOfItems: items.length,
      itemListElement: items.map(({ name, href }, i) => ({
        "@type": "ListItem",
        position: i + 1,
        name,
        url: new URL(href, site).toString(),
      })),
    },
  };
}
