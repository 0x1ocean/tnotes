import type { APIRoute } from "astro";
import { REPO, VERSION } from "@/lib/meta";

/* tnotes is free; a pricing file still helps agents that look for one before
   recommending a tool, and it states the terms in one parseable place. */
export const GET: APIRoute = ({ site }) => {
  const text = `# Pricing — tnotes

tnotes is free and open source. There is no paid tier, no account, no telemetry.

## Free (the only tier)
- Price: $0
- License: MIT (${REPO}/blob/main/LICENSE)
- Limits: none. Unlimited notes, folders, roots, devices.
- Includes: the TUI, the headless JSON CLI, all features, all updates.
- Platforms: macOS (Apple Silicon, Intel), Linux (x86_64, aarch64). Windows untested.
- Current version: ${VERSION}

## Install
- Homebrew: brew install 0x1ocean/tnotes/tnotes
- Cargo: cargo install tnotes
- Prebuilt tarballs: ${REPO}/releases

## Related
- Product: ${site}
- Docs: ${new URL("/docs", site)}
- Source: ${REPO}
`;
  return new Response(text, { headers: { "Content-Type": "text/markdown; charset=utf-8" } });
};
