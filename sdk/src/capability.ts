/** Types of a capability registry entry, matching `capability/definition.schema.json`. A definition is not authority. */

/** A JSON value. */
export type Json = null | boolean | number | string | readonly Json[] | { readonly [key: string]: Json };

/** RFC 3339 instant in UTC with the `Z` suffix. */
export type Instant = string;

/** `cap_` followed by 26 Crockford base32 digits of a version 7 UUID. */
export type CapabilityId = string;

/** An identifier in the owning system's own form: 1–255 printable ASCII characters. */
export type OpaqueRef = string;

/** A party (person or agent) owned by the identity and authority service. */
export interface PartyRef {
  readonly ref: "PartyRef";
  readonly id: OpaqueRef;
}

/** An agent instance owned by the identity and authority service. */
export interface InstanceRef {
  readonly ref: "InstanceRef";
  readonly id: OpaqueRef;
}

/** Text in both supported languages; each a single line. */
export interface LocalizedText {
  readonly tr: string;
  readonly en: string;
}

/** Fixed life areas. */
export type LifeArea =
  | "work"
  | "health"
  | "finance"
  | "relationships"
  | "home"
  | "travel"
  | "shopping"
  | "learning"
  | "leisure"
  | "general";

/** Kind of data. */
export type DataKind = "general" | "personal" | "contact" | "financial" | "location" | "communication" | "one_time_code";

/** Specially protected categories. */
export type SensitiveClass =
  | "health"
  | "religion_belief"
  | "political_opinion"
  | "sex_life_orientation"
  | "biometric"
  | "precise_location_history"
  | "relationship_status"
  | "genetic"
  | "racial_ethnic_origin"
  | "philosophical_belief_denomination"
  | "union_association_foundation_membership"
  | "criminal_conviction_security_measure"
  | "attire";

/** The data a capability reads or writes. `general` stands alone and touches no sensitive class. */
export interface DataClass {
  readonly kinds: readonly [DataKind, ...DataKind[]];
  readonly sensitive: readonly SensitiveClass[];
}

/** Provenance labels. */
export type ProvenanceLabel =
  | "user.direct"
  | "user.memory"
  | "record.trusted"
  | "one.inferred"
  | "external.web"
  | "external.email"
  | "external.calendar"
  | "external.document"
  | "external.screen"
  | "external.media"
  | "external.message"
  | "external.one"
  | "external.agent"
  | "external.tool_description"
  | "external.tool_output"
  | "external.import"
  | "external.app"
  | "external.unknown"
  | "external.mixed";

/** Retention class of a registry entry. */
export type RetentionClass = "one_memory" | "one_sensitive";

/** Consequence tier, from no consequence (`CT0`) to the highest (`CT3`). */
export type ConsequenceTier = "CT0" | "CT1" | "CT2" | "CT3";

/** Whether an effect can be undone. */
export type Reversibility = "reversible" | "compensable" | "irreversible";

/** The most a capability may do without asking. */
export type AutonomyBehaviour = "act" | "act-if-preapproved" | "ask" | "handoff";

/** Surfaces an approval can be given on. */
export type SurfaceClass = "phone" | "desktop" | "watch" | "voice:car" | "voice:glasses";

/** Assurance of an approval. */
export type Authentication = "none" | "device_unlock" | "passkey";

/** Output modalities. */
export type Modality = "voice" | "haptic" | "small_visual" | "full_visual";

/** How a provider is reached. */
export type PathType = "api" | "mcp" | "commerce_protocol" | "os_intent" | "browser";

/** A structured action reference: capability namespace, action name, dated version and schema digest. */
export interface ActionRef {
  readonly namespace: string;
  readonly name: string;
  readonly version: string;
  /** `sha256:` and 64 lower-case hex digits. */
  readonly schema_digest: string;
}

/** Who produced the entry. */
export type ProducedBy =
  | { readonly actor: "user" }
  | {
      readonly actor: "one" | "sub_agent";
      readonly instance: InstanceRef;
      readonly model_version: string;
      readonly config_version: string;
    };

/**
 * A closed JSON Schema 2020-12 object for input or output, or a `$ref` to a published schema.
 * Property names are snake_case and never name passwords, tokens, secrets, cookies or card data.
 */
export type TypedSchema = { readonly $ref: string } | { readonly [keyword: string]: Json };

/** A capability registry entry. */
export interface CapabilityDefinition {
  readonly type: "CapabilityDefinition";
  readonly id: CapabilityId;
  readonly status: "active" | "deleted";
  readonly supersedes: CapabilityId | null;
  readonly owner: PartyRef;
  readonly domain: { readonly areas: readonly [LifeArea, ...LifeArea[]]; readonly project: string | null };
  readonly data_class: DataClass;
  readonly retention_class: RetentionClass;
  readonly provenance: ProvenanceLabel;
  readonly recorded_at: Instant;
  /** Record version, from 1. */
  readonly version: number;
  /** Privacy promise version the record was collected under, from 1. */
  readonly promise_version: number;
  readonly deleted_at: Instant | null;
  readonly produced_by: ProducedBy;
  /** Capability name. */
  readonly name: string;
  /** Dated capability version `YYYY-MM-DD`. */
  readonly capability_version: string;
  /** Required for a third-party name. */
  readonly extends?: string | readonly string[];
  readonly publisher: PartyRef;
  readonly meaning: LocalizedText;
  /** Exactly one `primary` action; `undo` when reversible, `compensate` when compensable. */
  readonly actions: readonly { readonly role: "primary" | "undo" | "compensate"; readonly action_ref: ActionRef }[];
  readonly input_schema: TypedSchema;
  readonly output_schema: TypedSchema;
  /** At least one Turkish and one English example. */
  readonly examples: readonly {
    readonly lang: "tr" | "en";
    readonly utterance: string;
    readonly input: { readonly [key: string]: Json };
  }[];
  readonly authority: { readonly grant_scope: OpaqueRef; readonly consequence_tier: ConsequenceTier };
  readonly autonomy_ceiling: AutonomyBehaviour;
  readonly reversibility: Reversibility;
  readonly data_access: { readonly reads: DataClass | null; readonly writes: DataClass | null };
  readonly surface: {
    readonly modalities: readonly Modality[];
    readonly approval_ceiling: {
      readonly surface_classes: readonly [SurfaceClass, ...SurfaceClass[]];
      readonly min_assurance: Authentication;
    };
  };
  readonly providers: readonly [
    { readonly provider_ref: OpaqueRef; readonly path_type: PathType },
    ...{ readonly provider_ref: OpaqueRef; readonly path_type: PathType }[],
  ];
  readonly test_status: {
    readonly last_success_at: Instant;
    readonly last_result: "pass" | "fail";
    readonly evaluation_ref: OpaqueRef;
    readonly verified: { readonly reversibility: boolean; readonly data_access: boolean; readonly surface: boolean };
  };
  readonly projections: {
    readonly mcp_tool?: string;
    readonly a2a_skill?: string;
    readonly apple_app_schema?: string;
    readonly android_app_function?: string;
    readonly ucp?: string;
  };
}

/** The `$id` of the capability definition schema. */
export const DEFINITION_SCHEMA_ID = "https://suiss.app/schemas/one/capability/definition/v1.json";

/** The `$id` of the capability name schema. */
export const NAME_SCHEMA_ID = "https://suiss.app/schemas/one/capability/name/v1.json";
