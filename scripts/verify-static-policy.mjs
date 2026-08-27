import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";

const dist = resolve(process.argv[2] ?? "dist/site");
const configPath = resolve(dist, "staticwebapp.config.json");
const unsupportedHeadersPath = resolve(dist, "_headers");

if (!existsSync(configPath)) {
  throw new Error(`Missing Azure Static Web Apps policy file: ${configPath}`);
}
if (existsSync(unsupportedHeadersPath)) {
  throw new Error("Unsupported _headers file would be deployed as a public static object");
}

const config = JSON.parse(readFileSync(configPath, "utf8"));
const headers = config.globalHeaders ?? {};
const expectedHeaders = {
  "X-Content-Type-Options": "nosniff",
  "X-Frame-Options": "DENY",
  "Referrer-Policy": "strict-origin-when-cross-origin",
  "Permissions-Policy": "camera=(), microphone=(), geolocation=()"
};
for (const [header, value] of Object.entries(expectedHeaders)) {
  if (headers[header] !== value) throw new Error(`Missing or incorrect ${header}`);
}
const csp = headers["Content-Security-Policy"] ?? "";
for (const directive of ["default-src 'self'", "frame-ancestors 'none'", "connect-src 'self' https://api.sociobot.in"]) {
  if (!csp.includes(directive)) throw new Error(`CSP is missing ${directive}`);
}

const cacheRules = Object.fromEntries(
  (config.routes ?? []).map((route) => [route.route, route.headers?.["Cache-Control"]])
);
const expectedCacheRules = {
  "/assets/*": "public, max-age=31536000, immutable",
  "/version-specimen.webp": "public, max-age=31536000, immutable",
  "/service-worker.js": "no-cache"
};
for (const [route, value] of Object.entries(expectedCacheRules)) {
  if (cacheRules[route] !== value) throw new Error(`Missing or incorrect cache policy for ${route}`);
}

console.log(`Verified Azure response policy artifact: ${configPath}`);
