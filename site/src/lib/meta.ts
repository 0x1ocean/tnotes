import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { parse } from "smol-toml";

/** Build-time facts pulled from the crate so the site never drifts from it.
 *  Resolved from cwd (the Astro root, `site/`), which is also Vercel's root directory. */
const cargo = parse(readFileSync(resolve("../Cargo.toml"), "utf8")) as {
  package: { version: string; description: string; repository: string; "rust-version": string };
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
