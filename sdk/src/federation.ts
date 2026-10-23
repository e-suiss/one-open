/** Types and helpers of the One Federation A2A extension, matching `federation/schemas/*.v1.json`. */

import type { CapabilityRef, Declaration } from "./name.ts";

/** The extension URI: the key of the extension object in A2A metadata and the value in A2A `extensions` arrays. */
export const EXTENSION_URI = "https://suiss.app/a2a/one-federation/v1";

/** The extension schema version this package describes. */
export const SCHEMA_VERSION = "1.0";

/** The longest time between `sentAt` and `expiresAt`, and the longest handle lifetime, in seconds (7 days). */
export const MAX_EXPIRY_SECONDS = 604_800;

/** The longest note text, in characters. */
export const MAX_NOTE_LENGTH = 2000;

/** Typed message kinds. */
export const KINDS = ["offer", "counter-offer", "commitment", "rejection", "query", "answer"] as const;

/** A typed message kind. */
export type Kind = (typeof KINDS)[number];

/** Reason codes of a rejection. */
export type DeclineReason =
  | "no-capacity"
  | "no-authority"
  | "out-of-scope"
  | "missing-information"
  | "unacceptable-terms"
  | "other";

/** Protocol-level refusal codes. */
export const REFUSALS = [
  "schema-invalid",
  "extension-not-activated",
  "schema-version-unsupported",
  "kind-mismatch",
  "correlation-mismatch",
  "identity-mismatch",
  "authority-invalid",
  "version-not-monotonic",
  "reply-version-mismatch",
  "digest-mismatch",
  "offer-expired",
  "expiry-invalid",
  "operation-not-permitted",
  "note-not-convertible",
  "handle-missing",
  "handle-malformed",
  "handle-tampered",
  "handle-expired",
  "handle-replayed",
  "handle-binding-mismatch",
  "handle-digest-mismatch",
  "first-contact-not-permitted",
] as const;

/** A protocol-level refusal code. */
export type Refusal = (typeof REFUSALS)[number];

/** A `did:web` identifier. */
export type Did = string;

/** A lower-case RFC 9562 version 7 UUID. */
export type Uuid7 = string;

/** `sha-256:` and 43 unpadded base64url characters: SHA-256 over the JCS form of the referenced message without its handle. */
export type Digest = string;

/** RFC 3339 instant in UTC with the `Z` suffix. */
export type Timestamp = string;

/** The opaque, single-use suspended-negotiation handle (JWE compact, `dir` + `A256GCM`). Echo it unchanged; never parse it. */
export type HandleCompact = string;

/** A person and the agent acting for that person. */
export interface Party {
  readonly person: Did;
  readonly agent: Did;
}

/** A reference to the authority proof the recipient verifies; the reference itself is not authority. */
export interface AuthorityRef {
  readonly ref: string;
  readonly digest: Digest;
}

/** The message a reply answers, with the handle that message carried. */
export interface InReplyTo {
  readonly messageId: Uuid7;
  readonly offerVersion: number;
  readonly digest: Digest;
  readonly handle: HandleCompact;
}

/** First-contact fields. The relationship level is never carried. */
export type Contact =
  | { readonly firstContact: false }
  | {
      readonly firstContact: true;
      readonly initiation: "user-request";
      readonly recipientPartyKind: "person" | "organization";
    }
  | { readonly firstContact: true; readonly initiation: "mandate"; readonly recipientPartyKind: "organization" };

/** The envelope: the extension object in `Message.metadata[EXTENSION_URI]`, holding every mandatory field. */
export interface Envelope {
  readonly schemaVersion: string;
  readonly kind: Kind;
  readonly sender: Party;
  readonly recipient: Party;
  readonly authority: AuthorityRef;
  /** Identical for the whole offer chain and equal to the A2A `contextId`. */
  readonly correlationId: Uuid7;
  /** 1 for the first offer; each counter-offer is exactly one greater; 0 only for a query or answer before any offer. */
  readonly offerVersion: number;
  /** Informational: timeliness is judged on the receiving side. */
  readonly sentAt: Timestamp;
  /** At most 7 days after `sentAt`. When it passes the message is expired, never accepted or rejected. */
  readonly expiresAt: Timestamp;
  /** Required on counter-offer, commitment, rejection and answer; absent on offer. */
  readonly inReplyTo?: InReplyTo;
  /** Issued by the sender of an offer, counter-offer or query for the reply it awaits. */
  readonly handle?: HandleCompact;
  readonly contact?: Contact;
}

/** Typed field values; valid only against the referenced capability's schema. */
export type TypedFields = { readonly [field: string]: unknown };

/** A selective-disclosure request for person attributes. */
export interface DisclosureRequest {
  readonly vct: string;
  readonly claims: readonly string[];
}

/** An SD-JWT+KB presentation of exactly the requested claims. Not authority. */
export interface Presentation {
  readonly format: "dc+sd-jwt";
  readonly sdJwtKb: string;
}

/** The typed body. It carries no free text. */
export type Body =
  | { readonly kind: "offer"; readonly capability: CapabilityRef; readonly fields: TypedFields }
  | { readonly kind: "counter-offer"; readonly capability: CapabilityRef; readonly fields: TypedFields }
  | { readonly kind: "commitment" }
  | { readonly kind: "rejection"; readonly reason: DeclineReason }
  | { readonly kind: "query"; readonly capability: CapabilityRef; readonly fields: TypedFields }
  | { readonly kind: "query"; readonly disclosureRequest: DisclosureRequest }
  | { readonly kind: "answer"; readonly capability: CapabilityRef; readonly fields: TypedFields }
  | { readonly kind: "answer"; readonly presentation: Presentation };

/** The body DataPart. */
export interface BodyPart {
  readonly data: Body;
  readonly mediaType: "application/json";
  readonly metadata: { readonly [EXTENSION_URI]: { readonly part: "body" } };
}

/**
 * A free-text note for a human only. It is untrusted, displayed without rendering, and never
 * converted into a field value, operation or action.
 */
export interface NotePart {
  readonly text: string;
  readonly mediaType: "text/plain";
  readonly metadata: {
    readonly [EXTENSION_URI]: { readonly part: "note"; readonly trust: "untrusted"; readonly lang: string };
  };
}

/** An A2A v1.0 Message carrying one typed message: exactly one body part, then at most one note. */
export interface Message {
  readonly messageId: Uuid7;
  /** Equals the envelope `correlationId`. */
  readonly contextId: Uuid7;
  readonly taskId?: string;
  readonly role: "ROLE_USER" | "ROLE_AGENT";
  readonly parts: readonly [BodyPart] | readonly [BodyPart, NotePart];
  readonly metadata: { readonly [EXTENSION_URI]: Envelope };
  readonly extensions: readonly [typeof EXTENSION_URI];
}

/** Negotiation phase of a Task. */
export type Phase =
  | "evaluating"
  | "awaiting-counterparty"
  | "awaiting-principal"
  | "committed"
  | "declined"
  | "expired"
  | "refused";

/** A2A Task states used by the extension, each mapped to one phase. */
export type TaskState =
  | "TASK_STATE_SUBMITTED"
  | "TASK_STATE_WORKING"
  | "TASK_STATE_INPUT_REQUIRED"
  | "TASK_STATE_AUTH_REQUIRED"
  | "TASK_STATE_COMPLETED"
  | "TASK_STATE_REJECTED"
  | "TASK_STATE_CANCELED"
  | "TASK_STATE_FAILED";

/** The phase each Task state maps to. */
export const PHASE_OF_STATE: { readonly [S in TaskState]: Phase } = {
  TASK_STATE_SUBMITTED: "evaluating",
  TASK_STATE_WORKING: "evaluating",
  TASK_STATE_INPUT_REQUIRED: "awaiting-counterparty",
  TASK_STATE_AUTH_REQUIRED: "awaiting-principal",
  TASK_STATE_COMPLETED: "committed",
  TASK_STATE_REJECTED: "declined",
  TASK_STATE_CANCELED: "expired",
  TASK_STATE_FAILED: "refused",
};

/** The Artifact of a completed chain: a reference to the commitment the acceptance created. */
export interface CommitmentArtifact {
  readonly artifactId: string;
  readonly parts: readonly [
    {
      readonly data: {
        readonly correlationId: Uuid7;
        readonly workOperation: "promise" | "accept-counter";
        readonly acceptedOfferVersion: number;
        readonly acceptedDigest: Digest;
        readonly commitmentRef: string;
      };
      readonly mediaType: "application/json";
      readonly metadata: { readonly [EXTENSION_URI]: { readonly part: "commitment" } };
    },
  ];
  readonly extensions: readonly [typeof EXTENSION_URI];
}

/** The A2A Task that projects one offer chain. */
export interface Task {
  readonly id: string;
  readonly contextId: Uuid7;
  readonly status: { readonly state: TaskState; readonly timestamp: Timestamp; readonly message?: Message };
  readonly artifacts?: readonly [CommitmentArtifact];
  readonly history?: readonly Message[];
  readonly metadata: {
    readonly [EXTENSION_URI]: {
      readonly schemaVersion: string;
      readonly phase: Phase;
      readonly offerVersion: number;
      readonly expiresAt?: Timestamp;
      readonly refusal?: Refusal;
    };
  };
}

/** The AgentExtension entry an agent lists in its Agent Card `capabilities.extensions`. */
export interface AgentExtension {
  readonly uri: typeof EXTENSION_URI;
  readonly description?: string;
  readonly required: true;
  readonly params: {
    readonly schemaVersions: readonly string[];
    readonly kinds: readonly Kind[];
    readonly capabilities: readonly Declaration[];
  };
}

/** The Agent Card extension declaration for the given kinds and capabilities. */
export function agentExtension(kinds: readonly Kind[], capabilities: readonly Declaration[]): AgentExtension {
  return { uri: EXTENSION_URI, required: true, params: { schemaVersions: [SCHEMA_VERSION], kinds, capabilities } };
}

const NOTE_REMOVED: readonly (readonly [number, number])[] = [
  [0x0000, 0x0008],
  [0x000b, 0x001f],
  [0x007f, 0x009f],
  [0x00ad, 0x00ad],
  [0x034f, 0x034f],
  [0x061c, 0x061c],
  [0x115f, 0x1160],
  [0x17b4, 0x17b5],
  [0x180b, 0x180f],
  [0x200b, 0x200f],
  [0x202a, 0x202e],
  [0x2060, 0x206f],
  [0x3164, 0x3164],
  [0xfe00, 0xfe0f],
  [0xfeff, 0xfeff],
  [0xffa0, 0xffa0],
  [0xfff9, 0xfffb],
  [0x1d173, 0x1d17a],
  [0xe0000, 0xe0fff],
];

/**
 * The note text as it is displayed: every character in the ranges of `federation/note-filter.toml`
 * removed, everything else (markup included) kept as literal characters. Display it as plain text only.
 */
export function noteForDisplay(text: string): string {
  let out = "";
  for (const ch of text) {
    const cp = ch.codePointAt(0) ?? 0;
    if (!NOTE_REMOVED.some(([from, to]) => cp >= from && cp <= to)) {
      out += ch;
    }
  }
  return out;
}

/** True when `expiresAt` is after `sentAt` and at most {@link MAX_EXPIRY_SECONDS} later. */
export function isExpiryWithinLimit(sentAt: Timestamp, expiresAt: Timestamp): boolean {
  const sent = Date.parse(sentAt);
  const expires = Date.parse(expiresAt);
  if (Number.isNaN(sent) || Number.isNaN(expires)) {
    return false;
  }
  return expires > sent && expires - sent <= MAX_EXPIRY_SECONDS * 1000;
}
