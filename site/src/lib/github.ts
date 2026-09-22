import { REPO } from "./meta";

/** Star count at build time, or null when the API is unreachable or rate-limited
 *  (unauthenticated: 60 req/h, one call per build). Callers render a plain
 *  "github" link on null; never fail the build over a vanity number. */
export const STARS: number | null = await (async () => {
  try {
    const res = await fetch(`https://api.github.com/repos/${REPO.replace("https://github.com/", "")}`, {
      headers: { Accept: "application/vnd.github+json", "User-Agent": "tnotes.app-build" },
      signal: AbortSignal.timeout(4000),
    });
    if (!res.ok) return null;
    const { stargazers_count } = (await res.json()) as { stargazers_count: number };
    return stargazers_count;
  } catch {
    return null;
  }
})();
