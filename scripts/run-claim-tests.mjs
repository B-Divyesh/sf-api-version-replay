import { spawnSync } from "node:child_process";

const requested = process.argv[2];
const rustClaims = [
  "cli-demo-workflow",
  "redaction-before-storage",
  "loopback-only",
  "encrypted-storage",
  "capture-loopback",
  "report-formats",
  "contract-dimensions",
  "exact-replay",
  "fixture-formats",
  "exit-codes",
  "no-provider-credentials",
  "no-telemetry",
  "mit-license"
];
const browserClaims = ["browser-demo-isolation", "browser-storage-scope", "offline-demo", "route-metadata"];
const claims = requested ? [requested] : [...rustClaims, ...browserClaims];
let siteBuilt = false;

for (const id of claims) {
  let command;
  let args;
  if (rustClaims.includes(id)) {
    command = "cargo";
    args = ["test", "--test", "claims", `claim_${id.replaceAll("-", "_")}`, "--", "--exact"];
  } else if (browserClaims.includes(id)) {
    if (!siteBuilt) {
      const build = spawnSync("npm", ["run", "build:site"], { stdio: "inherit", env: process.env });
      if (build.status !== 0) process.exit(build.status ?? 1);
      siteBuilt = true;
    }
    command = "node";
    args = ["scripts/browser-claims.mjs", id];
  } else {
    throw new Error(`Unknown claim: ${id}`);
  }
  const result = spawnSync(command, args, { stdio: "inherit", env: process.env });
  if (result.status !== 0) process.exit(result.status ?? 1);
}
