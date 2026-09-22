import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { parse } from "smol-toml";

/** Build-time facts pulled from the crate so the site never drifts from it.
 *  Resolved from cwd (the Astro root, `site/`), which is also Vercel's root directory. */
const cargo = parse(readFileSync(resolve("../Cargo.toml"), "utf8")) as {
  package: { version: string; description: string; repository: string; "rust-version": string };
};

/** Date of the current release, from `## [VERSION] - YYYY-MM-DD` in CHANGELOG.md.
 *  Used as `dateModified` for docs: they describe this version, so this is
 *  the honest freshness signal (git dates are unreliable in shallow CI clones). */
export const RELEASE_DATE =
  readFileSync(resolve("../CHANGELOG.md"), "utf8").match(new RegExp(`^## \\[${cargo.package.version.replace(/\\./g, "\\\\.")}\\] - (\\d{4}-\\d{2}-\\d{2})`, "m"))?.[1] ??
  new Date().toISOString().slice(0, 10);

/** The author, as a schema.org Person; referenced by `@id` from articles. */
export const AUTHOR_ID = "https://github.com/0x1ocean#person";
export const AUTHOR = {
  "@type": "Person",
  "@id": AUTHOR_ID,
  name: "0x1ocean",
  url: "https://github.com/0x1ocean",
  sameAs: ["https://github.com/0x1ocean", "https://crates.io/users/0x1ocean", "https://romaocean.com", "https://0x1ocean.com"],
};

export const VERSION = cargo.package.version;
export const DESCRIPTION = cargo.package.description;
export const REPO = cargo.package.repository;
export const RUST_VERSION = cargo.package["rust-version"];
export const RELEASES = `${REPO}/releases`;
export const TAP = "0x1ocean/tnotes/tnotes";

export const SITE_NAME = "tnotes";
export const TAGLINE = "Markdown notes that live in your terminal";

export const TARGETS = [
  "aarch64-apple-darwin",
  "x86_64-apple-darwin",
  "x86_64-unknown-linux-gnu",
  "x86_64-unknown-linux-musl",
  "aarch64-unknown-linux-gnu",
] as const;
