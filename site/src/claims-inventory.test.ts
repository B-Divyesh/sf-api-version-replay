import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";

interface Claim { id: string; claim: string; where: string; test: string; sandbox: string }

const root = resolve(import.meta.dirname, "../..");
const claims = JSON.parse(readFileSync(resolve(root, ".factory/claims.json"), "utf8")) as Claim[];
const taggedSources = ["tests/claims.rs", "scripts/browser-claims.mjs"]
  .map((path) => readFileSync(resolve(root, path), "utf8"))
  .join("\n");

describe("claims inventory", () => {
  it("gives every retained claim exactly one tagged test and runnable command", () => {
    expect(claims.length).toBeGreaterThan(0);
    expect(new Set(claims.map(({ id }) => id)).size).toBe(claims.length);
    for (const claim of claims) {
      expect(claim.claim).not.toBe("");
      expect(claim.where).not.toBe("");
      expect(claim.sandbox).not.toBe("");
      expect(claim.test).toBe(`npm run test:claims -- ${claim.id}`);
      expect(taggedSources.split(`@claim:${claim.id}`).length - 1).toBe(1);
    }
  });
});
