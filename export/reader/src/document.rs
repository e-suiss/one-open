//! The `one-export` root of `export.cddl` and its sections.

use serde::{Deserialize, Serialize};

use crate::record::{Episode, MemoryRecord, RelationshipEdge, RetentionClass, Statement};
use crate::reference::PartyRef;
use crate::scalar::{
    AdapterId, Base64Url, Instant, MemoryRecordId, Sha256Hex, StepCapabilityName, TaskId, const_tag,
};

const_tag!(
    /// `format: "app.suiss.one.export"`.
    ExportFormat,
    "app.suiss.one.export"
);
const_tag!(
    /// `source: "access-e65"`: the item list comes from the identity and authority service's read API.
    AccessReadApi,
    "access-e65"
);
const_tag!(
    /// `format: "safetensors"`.
    Safetensors,
    "safetensors"
);

/// `generator-info`: the program that wrote the package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Generator {
    /// Name.
    pub name: String,
    /// Version.
    pub version: String,
}

/// `memory-section`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemorySection {
    /// Memory records; forgotten ones as skeletons.
    pub records: Vec<MemoryRecord>,
    /// Relationship edges.
    pub relationship_edges: Vec<RelationshipEdge>,
    /// Episodes.
    pub episodes: Vec<Episode>,
}

/// A section left out of the package.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExcludedSection {
    /// The sensitive section.
    Sensitive,
}

/// The assistant profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Profile {
    /// Assistant name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Form of address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub form_of_address: Option<String>,
    /// Style.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    /// Values.
    pub values: Vec<String>,
}

/// `personality-export`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Personality {
    /// Profile.
    pub profile: Profile,
    /// Behaviour-rule records.
    pub rules: Vec<MemoryRecordId>,
}

/// `access-projection`: grants or connections as the identity and authority service returned them.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccessProjection {
    /// `access-e65`.
    pub source: AccessReadApi,
    /// When it was read.
    pub read_at: Instant,
    /// Schema of the items.
    pub schema: String,
    /// Items, unchanged.
    pub items: Vec<serde_json::Map<String, serde_json::Value>>,
}

/// `task-step`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskStep {
    /// Capability name.
    pub capability: StepCapabilityName,
    /// State.
    pub state: String,
    /// Result.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<Statement>,
}

/// `task`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    /// Identifier.
    pub id: TaskId,
    /// Creation time.
    pub created_at: Instant,
    /// State.
    pub state: String,
    /// Goal.
    pub goal: Statement,
    /// Steps.
    pub steps: Vec<TaskStep>,
    /// Outcome.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outcome: Option<Statement>,
}

/// The base model an adapter needs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BaseModel {
    /// Name.
    pub name: String,
    /// Version.
    pub version: String,
}

/// `adapter-export`: the personal adapter weights.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Adapter {
    /// Identifier.
    pub id: AdapterId,
    /// The base model it needs.
    pub base_model: BaseModel,
    /// Training time.
    pub trained_at: Instant,
    /// `safetensors`.
    pub format: Safetensors,
    /// SHA-256 of the weights.
    pub sha256: Sha256Hex,
    /// Weights, unpadded base64url.
    pub weights_b64: Base64Url,
    /// The note that the weights need the base model.
    pub note: Statement,
}

/// `one-export`: an export package.
///
/// Every map in the format is open: fields this reader does not know are skipped
/// when reading and are not written back.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExportDocument {
    /// `app.suiss.one.export`.
    pub format: ExportFormat,
    /// Schema version, from 1. Versions only add fields.
    pub schema_version: core::num::NonZeroU64,
    /// Export time.
    pub exported_at: Instant,
    /// Owner.
    pub owner: PartyRef,
    /// Generator.
    pub generator: Generator,
    /// Retention class of the package.
    pub retention_class: RetentionClass,
    /// Memory.
    pub memory: MemorySection,
    /// The sensitive section, unless excluded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sensitive: Option<MemorySection>,
    /// Sections left out.
    pub excluded_sections: Vec<ExcludedSection>,
    /// Personality.
    pub personality: Personality,
    /// Grants.
    pub grants: AccessProjection,
    /// Connections.
    pub connections: AccessProjection,
    /// Tasks.
    pub tasks: Vec<Task>,
    /// Personal adapter, or `null`.
    #[serde(deserialize_with = "Option::deserialize")]
    pub adapter: Option<Adapter>,
    /// Human-readable view.
    pub text_view: String,
}
