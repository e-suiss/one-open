/** The capability and federation vectors of this repository through the SDK helpers. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { test } from "node:test";

import {
  type Declaration,
  isExpiryWithinLimit,
  negotiate,
  noteForDisplay,
  parseCapabilityName,
  parseCapabilityVersion,
} from "../src/index.ts";

interface Vector {
  readonly description: string;
  readonly input: unknown;
  readonly expected: unknown;
}

function vectors(path: string): Vector[] {
  const parsed: unknown = JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
  assert.ok(Array.isArray(parsed));
  return parsed as Vector[];
}

function nameResult(input: string, expected: { readonly kind?: string }): unknown {
  if (expected.kind === "version") {
    return { valid: parseCapabilityVersion(input) !== undefined, kind: "version" };
  }
  const result = parseCapabilityName(input);
  if (!result.ok) {
    return { valid: false, error: result.error };
  }
  const { kind, category, segments } = result.value;
  return category === undefined
    ? { valid: true, kind, segments }
    : { valid: true, kind, category, segments };
}

test("capability names and versions follow the naming rules", () => {
  const all = vectors("../../capability/vectors/name.json");
  assert.equal(all.length, 26);
  for (const v of all) {
    assert.ok(typeof v.input === "string");
    assert.deepEqual(nameResult(v.input, v.expected as { readonly kind?: string }), v.expected, v.description);
  }
});

test("negotiation keeps the highest common version and reachable extensions", () => {
  const all = vectors("../../capability/vectors/negotiation.json");
  assert.equal(all.length, 12);
  for (const v of all) {
    const input = v.input as { readonly local: Declaration[]; readonly remote: Declaration[] };
    assert.deepEqual(negotiate(input.local, input.remote), v.expected, v.description);
  }
});

test("expiry is after sentAt and at most seven days later", () => {
  let checked = 0;
  for (const v of vectors("../../federation/vectors/expiry.json")) {
    const input = v.input as { readonly message?: { readonly sentAt: string; readonly expiresAt: string } };
    if (input.message === undefined) {
      continue;
    }
    const expected = v.expected as { readonly accept: boolean };
    assert.equal(isExpiryWithinLimit(input.message.sentAt, input.message.expiresAt), expected.accept, v.description);
    checked += 1;
  }
  assert.equal(checked, 3);
});

test("note display removes invisible characters and keeps markup literal", () => {
  assert.equal(noteForDisplay("a​b‮c\u0007d\te\nf"), "abcd\te\nf");
  assert.equal(noteForDisplay("<b>hi</b>"), "<b>hi</b>");
  assert.equal(noteForDisplay("x\u{E0041}y"), "xy");
});
