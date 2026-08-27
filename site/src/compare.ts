export type ChangeKind = "added" | "removed" | "changed";

export interface BrowserChange {
  path: string;
  kind: ChangeKind;
  before?: string;
  after?: string;
}

export interface BrowserDiff {
  schema: BrowserChange[];
  values: BrowserChange[];
}

type FlatMap = Map<string, string>;

function typeOf(value: unknown): string {
  if (value === null) return "null";
  if (Array.isArray(value)) return "array";
  if (typeof value === "number" && Number.isInteger(value)) return "integer";
  return typeof value;
}

function flatten(value: unknown, path: string, schema: FlatMap, values: FlatMap): void {
  schema.set(path, typeOf(value));
  if (Array.isArray(value)) {
    value.forEach((entry, index) => flatten(entry, `${path}[${index}]`, schema, values));
    return;
  }
  if (value !== null && typeof value === "object") {
    Object.entries(value as Record<string, unknown>).forEach(([key, entry]) => {
      flatten(entry, `${path}.${key}`, schema, values);
    });
    return;
  }
  values.set(path, JSON.stringify(value));
}

function compareMaps(before: FlatMap, after: FlatMap): BrowserChange[] {
  const paths = [...new Set([...before.keys(), ...after.keys()])].sort();
  return paths.flatMap<BrowserChange>((path): BrowserChange[] => {
    const left = before.get(path);
    const right = after.get(path);
    if (left === right) return [];
    if (left === undefined) return [{ path, kind: "added" as const, after: right }];
    if (right === undefined) return [{ path, kind: "removed" as const, before: left }];
    return [{ path, kind: "changed" as const, before: left, after: right }];
  });
}

export function compareJson(beforeText: string, afterText: string): BrowserDiff {
  const before = JSON.parse(beforeText) as unknown;
  const after = JSON.parse(afterText) as unknown;
  const beforeSchema: FlatMap = new Map();
  const afterSchema: FlatMap = new Map();
  const beforeValues: FlatMap = new Map();
  const afterValues: FlatMap = new Map();
  flatten(before, "$", beforeSchema, beforeValues);
  flatten(after, "$", afterSchema, afterValues);
  return {
    schema: compareMaps(beforeSchema, afterSchema),
    values: compareMaps(beforeValues, afterValues)
  };
}
