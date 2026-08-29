import { readFileSync, existsSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";

const publicDir = resolve(import.meta.dirname, "../public");
const policy = JSON.parse(
  readFileSync(resolve(publicDir, "staticwebapp.config.json"), "utf8")
) as {
  globalHeaders: Record<string, string>;
  routes: Array<{ route: string; headers: Record<string, string> }>;
};

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
});
