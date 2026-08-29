import { readFileSync, existsSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";

const publicDir = resolve(import.meta.dirname, "../public");
const root = resolve(import.meta.dirname, "../..");
const policy = JSON.parse(
  readFileSync(resolve(publicDir, "staticwebapp.config.json"), "utf8")
) as {
  globalHeaders: Record<string, string>;
  routes: Array<{ route: string; headers: Record<string, string> }>;
};

function githubHeadingSlug(heading: string): string {
  return heading
    .trim()
    .toLowerCase()
    .replace(/[`*_~]/g, "")
    .replace(/[^\p{L}\p{N}\s-]/gu, "")
    .replace(/\s+/g, "-")
    .replace(/-+/g, "-");
}

describe("Azure Static Web Apps response policy", () => {
  it("uses the platform configuration rather than publishing an unsupported _headers file", () => {
    expect(existsSync(resolve(publicDir, "_headers"))).toBe(false);
    expect(policy.globalHeaders).toMatchObject({
      "X-Content-Type-Options": "nosniff",
      "X-Frame-Options": "DENY",
      "Referrer-Policy": "strict-origin-when-cross-origin",
      "Permissions-Policy": "camera=(), microphone=(), geolocation=()"
    });
    expect(policy.globalHeaders["Content-Security-Policy"]).toContain("default-src 'self'");
    expect(policy.globalHeaders["Content-Security-Policy"]).toContain("frame-ancestors 'none'");
    expect(policy.globalHeaders["Content-Security-Policy"]).toContain("connect-src 'self'");
    expect(policy.globalHeaders["Content-Security-Policy"]).not.toContain("api.sociobot.in");
    expect((policy as { responseOverrides?: Record<string, unknown> }).responseOverrides?.["404"]).toEqual({ rewrite: "/404.html" });
  });

  it("keeps immutable assets and the updateable worker on distinct cache policies", () => {
    const cachePolicy = Object.fromEntries(
      policy.routes.map((route) => [route.route, route.headers["Cache-Control"]])
    );
    expect(cachePolicy).toMatchObject({
      "/assets/*": "public, max-age=31536000, immutable",
      "/version-specimen.webp": "public, max-age=31536000, immutable",
      "/service-worker.js": "no-cache"
    });
  });

  it("points GitHub fragment links at real README headings", () => {
    const home = readFileSync(resolve(root, "site/index.html"), "utf8");
    const readme = readFileSync(resolve(root, "README.md"), "utf8");
    const headingSlugs = [...readme.matchAll(/^#{1,6}\s+(.+)$/gm)]
      .map(([, heading]) => githubHeadingSlug(heading));
    const fragmentLinks = [...home.matchAll(/href="(https:\/\/github\.com\/B-Divyesh\/sf-api-version-replay#[^"]+)"/g)]
      .map(([, href]) => new URL(href));

    expect(fragmentLinks.length).toBeGreaterThan(0);
    for (const link of fragmentLinks) {
      expect(headingSlugs, `${link.href} must resolve to a README heading`).toContain(
        decodeURIComponent(link.hash.slice(1))
      );
    }
  });
});
