/** Capability names, dated versions and negotiation, as defined by `capability/name.schema.json` and `capability/naming.toml`. */

/** The reverse-DNS root of Suiss capability names. */
export const ROOT = "app.suiss";

/** The longest capability name, in characters. */
export const MAX_NAME_LENGTH = 128;

/** First segments that never follow `app.suiss`. `one` is the reverse form of the handle subdomain `one.suiss.app`. */
export const RESERVED_SEGMENTS = ["one", "access", "relay", "work", "pay", "suiss", "www", "api"] as const;

/** Why a name does not parse; the first failing check decides. */
export type NameError =
  | "not_lowercase"
  | "too_long"
  | "bad_root"
  | "bad_segment"
  | "too_few_segments"
  | "too_many_segments"
  | "reserved_segment"
  | "handle_namespace";

/** A Suiss name lives under `app.suiss`; a third-party name under the reverse DNS of a domain its publisher controls. */
export type NameKind = "suiss" | "third_party";

/** A capability name that passed {@link parseCapabilityName}. */
export type CapabilityName = string & { readonly __brand: "CapabilityName" };

/** A dated capability version `YYYY-MM-DD` that passed {@link parseCapabilityVersion}. */
export type CapabilityVersion = string & { readonly __brand: "CapabilityVersion" };

/** A parsed capability name. */
export interface ParsedName {
  readonly name: CapabilityName;
  readonly kind: NameKind;
  /** Semantic segments: below `app.suiss` for a Suiss name, every label for a third-party name. */
  readonly segments: readonly string[];
  /** The category (first semantic segment) of a Suiss name. */
  readonly category?: string;
}

/** The result of {@link parseCapabilityName}. */
export type NameResult = { readonly ok: true; readonly value: ParsedName } | { readonly ok: false; readonly error: NameError };

/** A capability by name and dated version: the one capability reference form. */
export interface CapabilityRef {
  readonly name: string;
  readonly version: string;
}

/** One side's offer of a capability during negotiation. A third-party name must extend at least one parent. */
export interface Declaration {
  readonly name: string;
  readonly versions: readonly string[];
  readonly extends?: string | readonly string[];
}

const isLower = (c: string): boolean => c >= "a" && c <= "z";
const isDigit = (c: string): boolean => c >= "0" && c <= "9";

function isSuissSegment(segment: string): boolean {
  const first = segment[0];
  return (
    first !== undefined &&
    isLower(first) &&
    segment.length <= 32 &&
    [...segment].every((c) => isLower(c) || isDigit(c))
  );
}

function isLabel(label: string): boolean {
  const edge = (c: string | undefined): boolean => c !== undefined && (isLower(c) || isDigit(c));
  return (
    label.length >= 1 &&
    label.length <= 63 &&
    edge(label[0]) &&
    edge(label[label.length - 1]) &&
    [...label].every((c) => isLower(c) || isDigit(c) || c === "_" || c === "-")
  );
}

function isTld(label: string): boolean {
  return label.length >= 2 && label.length <= 63 && [...label].every(isLower);
}

const fail = (error: NameError): NameResult => ({ ok: false, error });

/** Parses a capability name by the grammar of `capability/naming.toml`. */
export function parseCapabilityName(text: string): NameResult {
  if (/[A-Z]/.test(text)) {
    return fail("not_lowercase");
  }
  if (text.length > MAX_NAME_LENGTH) {
    return fail("too_long");
  }
  if (text === ROOT) {
    return fail("too_few_segments");
  }
  const name = text as CapabilityName;
  if (text.startsWith(`${ROOT}.`)) {
    const segments = text.slice(ROOT.length + 1).split(".");
    if (!segments.every(isSuissSegment)) {
      return fail("bad_segment");
    }
    const first = segments[0];
    if (first === "one") {
      return fail("handle_namespace");
    }
    if (first !== undefined && (RESERVED_SEGMENTS as readonly string[]).includes(first)) {
      return fail("reserved_segment");
    }
    if (segments.length < 2) {
      return fail("too_few_segments");
    }
    if (segments.length > 5) {
      return fail("too_many_segments");
    }
    return first === undefined
      ? fail("too_few_segments")
      : { ok: true, value: { name, kind: "suiss", segments, category: first } };
  }
  const labels = text.split(".");
  const [tld, ...rest] = labels;
  if (tld === undefined || !isTld(tld)) {
    return fail("bad_root");
  }
  if (!rest.every(isLabel)) {
    return fail("bad_segment");
  }
  if (rest.length < 2) {
    return fail("bad_root");
  }
  if (rest.length > 9) {
    return fail("too_many_segments");
  }
  return { ok: true, value: { name, kind: "third_party", segments: labels } };
}

/** True when `text` is a valid capability name. */
export function isCapabilityName(text: string): text is CapabilityName {
  return parseCapabilityName(text).ok;
}

/** Parses a dated version `YYYY-MM-DD`: zero-padded fields, month 01–12, day 01–31. */
export function parseCapabilityVersion(text: string): CapabilityVersion | undefined {
  return /^[0-9]{4}-(0[1-9]|1[0-2])-(0[1-9]|[12][0-9]|3[01])$/.test(text) ? (text as CapabilityVersion) : undefined;
}

function parents(declaration: Declaration): readonly string[] | undefined {
  const value = declaration.extends;
  if (value === undefined) {
    return undefined;
  }
  return typeof value === "string" ? [value] : value;
}

/** Checks a declaration against the name schema; an invalid declaration is ignored during negotiation. */
export function isValidDeclaration(declaration: Declaration): boolean {
  const parsed = parseCapabilityName(declaration.name);
  if (!parsed.ok) {
    return false;
  }
  const versions = declaration.versions;
  if (versions.length === 0 || new Set(versions).size !== versions.length) {
    return false;
  }
  if (!versions.every((v) => parseCapabilityVersion(v) !== undefined)) {
    return false;
  }
  const ps = parents(declaration);
  if (ps === undefined) {
    return parsed.value.kind === "suiss";
  }
  if (typeof declaration.extends !== "string" && (ps.length < 2 || new Set(ps).size !== ps.length)) {
    return false;
  }
  return ps.every(isCapabilityName);
}

function sameExtends(a: Declaration, b: Declaration): boolean {
  const x = parents(a);
  const y = parents(b);
  if (x === undefined || y === undefined) {
    return x === y;
  }
  const sx = new Set(x);
  const sy = new Set(y);
  return sx.size === sy.size && [...sx].every((p) => sy.has(p));
}

/**
 * Negotiates two capability offers by the steps of `capability/naming.toml`: intersect by name, drop
 * names whose `extends` differ, keep the highest common version, keep Suiss names, then keep an
 * extension only when a parent is kept, until nothing changes. Invalid declarations are ignored;
 * a name declared twice on one side counts once, by its first valid declaration.
 */
export function negotiate(local: readonly Declaration[], remote: readonly Declaration[]): CapabilityRef[] {
  const mine = local.filter(isValidDeclaration);
  const theirs = remote.filter(isValidDeclaration);
  const seen = new Set<string>();
  const candidates: { readonly declaration: Declaration; readonly version: string }[] = [];
  for (const d of mine) {
    if (seen.has(d.name)) {
      continue;
    }
    seen.add(d.name);
    const other = theirs.find((t) => t.name === d.name);
    if (other === undefined || !sameExtends(d, other)) {
      continue;
    }
    const common = d.versions.filter((v) => other.versions.includes(v)).sort();
    const best = common[common.length - 1];
    if (best !== undefined) {
      candidates.push({ declaration: d, version: best });
    }
  }
  const kept = new Set(candidates.filter((c) => parents(c.declaration) === undefined).map((c) => c.declaration.name));
  let changed = true;
  while (changed) {
    changed = false;
    for (const c of candidates) {
      const ps = parents(c.declaration);
      if (!kept.has(c.declaration.name) && ps !== undefined && ps.some((p) => kept.has(p))) {
        kept.add(c.declaration.name);
        changed = true;
      }
    }
  }
  return candidates
    .filter((c) => kept.has(c.declaration.name))
    .map((c) => ({ name: c.declaration.name, version: c.version }))
    .sort((a, b) => (a.name < b.name ? -1 : a.name > b.name ? 1 : 0));
}
