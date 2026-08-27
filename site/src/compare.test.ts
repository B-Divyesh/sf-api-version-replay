import { describe, expect, it } from "vitest";
import { compareJson } from "./compare";

describe("browser contract comparison", () => {
  it("finds additions, removals, and type changes", () => {
    const result = compareJson(
      '{"amount":1000,"legacy":true}',
      '{"amount":"1000","currency":"usd"}'
    );
    expect(result.schema).toContainEqual({
      path: "$.amount",
      kind: "changed",
      before: "integer",
      after: "string"
    });
    expect(result.schema.some((change) => change.path === "$.currency" && change.kind === "added")).toBe(true);
    expect(result.schema.some((change) => change.path === "$.legacy" && change.kind === "removed")).toBe(true);
  });

  it("throws on malformed fixture JSON", () => {
    expect(() => compareJson("{", "{}")).toThrow();
  });
});
