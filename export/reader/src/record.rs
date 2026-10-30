//! Memory records, relationship edges, episodes and their forgotten skeletons.

use serde::{Deserialize, Deserializer, Serialize};

use crate::reference::{EntityRef, ExternalRef, InstanceRef, PartyRef};
use crate::scalar::{
    DeviceId, EpisodeId, Instant, InteractionId, Lang, MemoryExtensionName, MemoryRecordId,
    ModelVersionId, Month, NonEmpty, Null, ObservationId, PredicateExtensionName, ProjectId,
    TaskId, TrustValue, const_tag,
};

const_tag!(
    /// `type: "MemoryRecord"`.
    MemoryRecordType,
    "MemoryRecord"
);
const_tag!(
    /// `type: "Episode"`.
    EpisodeType,
    "Episode"
);
const_tag!(
    /// `status: "forgotten"`.
    Forgotten,
    "forgotten"
);
const_tag!(
    /// `status: "active"`.
    Active,
    "active"
);
const_tag!(
    /// `formula: "trust/v1"`.
    TrustFormulaV1,
    "trust/v1"
);

/// Retention class of a record or package.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionClass {
    /// Ordinary memory.
    OneMemory,
    /// Memory touching a specially protected class.
    OneSensitive,
}

/// `life-area`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifeArea {
    /// Work.
    Work,
    /// Health.
    Health,
    /// Finance.
    Finance,
    /// Relationships.
    Relationships,
    /// Home.
    Home,
    /// Travel.
    Travel,
    /// Shopping.
    Shopping,
    /// Learning.
    Learning,
    /// Leisure.
    Leisure,
    /// General.
    General,
}

/// Kind of data in `data-class`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataKind {
    /// No personal data.
    General,
    /// About a person.
    Personal,
    /// Contact details.
    Contact,
    /// Money, accounts or payments.
    Financial,
    /// Place or position.
    Location,
    /// Message or call content.
    Communication,
    /// A one-time code or verification secret.
    OneTimeCode,
}

/// `sensitive-class`: a specially protected category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SensitiveClass {
    /// Health.
    Health,
    /// Religion or belief.
    ReligionBelief,
    /// Political opinion.
    PoliticalOpinion,
    /// Sex life or sexual orientation.
    SexLifeOrientation,
    /// Biometric data.
    Biometric,
    /// Precise location history.
    PreciseLocationHistory,
    /// Relationship status.
    RelationshipStatus,
    /// Genetic data.
    Genetic,
    /// Racial or ethnic origin.
    RacialEthnicOrigin,
    /// Philosophical belief or denomination.
    PhilosophicalBeliefDenomination,
    /// Union, association or foundation membership.
    UnionAssociationFoundationMembership,
    /// Criminal conviction or security measure.
    CriminalConvictionSecurityMeasure,
    /// Attire.
    Attire,
}

/// `provenance-label`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProvenanceLabel {
    /// The user, directly.
    #[serde(rename = "user.direct")]
    UserDirect,
    /// The user's own memory statement.
    #[serde(rename = "user.memory")]
    UserMemory,
    /// A trusted record.
    #[serde(rename = "record.trusted")]
    RecordTrusted,
    /// Inferred by the assistant.
    #[serde(rename = "one.inferred")]
    OneInferred,
    /// The web.
    #[serde(rename = "external.web")]
    ExternalWeb,
    /// E-mail.
    #[serde(rename = "external.email")]
    ExternalEmail,
    /// Calendar.
    #[serde(rename = "external.calendar")]
    ExternalCalendar,
    /// A document.
    #[serde(rename = "external.document")]
    ExternalDocument,
    /// Screen content.
    #[serde(rename = "external.screen")]
    ExternalScreen,
    /// Media.
    #[serde(rename = "external.media")]
    ExternalMedia,
    /// A message.
    #[serde(rename = "external.message")]
    ExternalMessage,
    /// Another person's assistant.
    #[serde(rename = "external.one")]
    ExternalOne,
    /// An agent.
    #[serde(rename = "external.agent")]
    ExternalAgent,
    /// A tool description.
    #[serde(rename = "external.tool_description")]
    ExternalToolDescription,
    /// A tool output.
    #[serde(rename = "external.tool_output")]
    ExternalToolOutput,
    /// An import.
    #[serde(rename = "external.import")]
    ExternalImport,
    /// An app.
    #[serde(rename = "external.app")]
    ExternalApp,
    /// Unknown external source.
    #[serde(rename = "external.unknown")]
    ExternalUnknown,
    /// Mixed external sources.
    #[serde(rename = "external.mixed")]
    ExternalMixed,
}

/// `domain-info`: life areas and an optional user project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Domain {
    /// At least one life area.
    pub areas: NonEmpty<LifeArea>,
    /// The user project, or `null`.
    #[serde(deserialize_with = "Option::deserialize")]
    pub project: Option<ProjectId>,
}

/// `data-class`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DataClass {
    /// At least one kind.
    pub kinds: NonEmpty<DataKind>,
    /// Specially protected classes it touches.
    pub sensitive: Vec<SensitiveClass>,
}

/// Who produced a record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Producer {
    /// The user.
    User,
    /// The assistant.
    One,
    /// A sub-agent.
    SubAgent,
}

/// `produced-by`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProducedBy {
    /// The producer.
    pub actor: Producer,
    /// The agent instance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance: Option<InstanceRef>,
    /// The model version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_version: Option<ModelVersionId>,
    /// The configuration version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_version: Option<String>,
}

/// `header`: the common fields of a live record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordHeader {
    /// Owner.
    pub owner: PartyRef,
    /// Domain.
    pub domain: Domain,
    /// Data class.
    pub data_class: DataClass,
    /// Retention class.
    pub retention_class: RetentionClass,
    /// Provenance label.
    pub provenance: ProvenanceLabel,
    /// Transaction time.
    pub recorded_at: Instant,
    /// Record version, from 1.
    pub version: core::num::NonZeroU64,
    /// Privacy promise version the record was collected under, from 1.
    pub promise_version: core::num::NonZeroU64,
    /// Always `null` on a live record.
    pub deleted_at: Null,
    /// Producer.
    pub produced_by: ProducedBy,
}

/// `text-statement`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Statement {
    /// Text.
    pub text: String,
    /// Language of the text.
    pub lang: Lang,
}

/// `core-content-type`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoreContentType {
    /// Preference.
    Preference,
    /// Routine.
    Routine,
    /// Goal.
    Goal,
    /// Constraint.
    Constraint,
    /// Relationship.
    Relationship,
    /// Personal fact.
    PersonalFact,
    /// Opinion.
    Opinion,
    /// Event.
    Event,
}

/// A content-type extension that maps to a core content type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentTypeExtension {
    /// Name under `app.suiss.memory`.
    pub name: MemoryExtensionName,
    /// Version.
    pub version: u64,
    /// The core content type it maps to.
    pub maps_to: CoreContentType,
}

/// `content-type`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContentType {
    /// A core content type.
    Core(CoreContentType),
    /// An extension.
    Extension(ContentTypeExtension),
}

/// `core-predicate`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CorePredicate {
    /// Has manager.
    HasManager,
    /// Has report.
    HasReport,
    /// Has colleague.
    HasColleague,
    /// Has partner.
    HasPartner,
    /// Has parent.
    HasParent,
    /// Has child.
    HasChild,
    /// Has sibling.
    HasSibling,
    /// Has friend.
    HasFriend,
    /// Knows.
    Knows,
    /// Works at.
    WorksAt,
    /// Studies at.
    StudiesAt,
    /// Lives in.
    LivesIn,
    /// Member of.
    MemberOf,
}

/// A predicate extension that maps to a core predicate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PredicateExtension {
    /// Name under `app.suiss.memory.predicate`.
    pub name: PredicateExtensionName,
    /// Version.
    pub version: u64,
    /// The core predicate it maps to.
    pub maps_to: CorePredicate,
}

/// `relationship-predicate`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Predicate {
    /// A core predicate.
    Core(CorePredicate),
    /// An extension.
    Extension(PredicateExtension),
}

/// Lifecycle of a live record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LiveStatus {
    /// Not yet confirmed.
    Candidate,
    /// In use.
    Active,
    /// Kept but not used.
    Suppressed,
}

/// Epistemic state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EpistemicState {
    /// Stated by the user.
    Stated,
    /// Observed.
    Observed,
    /// Inferred.
    Inferred,
    /// A hypothesis.
    Hypothesis,
    /// Verified.
    Verified,
    /// Invalidated.
    Invalidated,
}

/// Memory layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Layer {
    /// An event.
    Event,
    /// A consolidated fact.
    ConsolidatedFact,
    /// A behaviour rule.
    BehaviourRule,
}

/// Time kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimeKind {
    /// Permanent.
    Permanent,
    /// Current, with an expected end.
    Current,
    /// Seasonal, with months.
    Seasonal,
    /// Once.
    OneOff,
    /// In the past.
    Past,
}

/// Months of a seasonal record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Season {
    /// At least one month.
    pub months: NonEmpty<Month>,
}

/// `source-kind`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceKind {
    /// A user statement.
    UserStatement,
    /// User behaviour.
    UserBehaviour,
    /// A connected source.
    ConnectedSource,
    /// External content.
    ExternalContent,
    /// An import.
    Import,
}

/// `evidence-info`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Evidence {
    /// Observations, at least one.
    pub observation_count: core::num::NonZeroU64,
    /// Supporting observations.
    pub supporting_count: u64,
    /// First observation.
    pub first_observed_at: Instant,
    /// Last observation.
    pub last_observed_at: Instant,
    /// Contradicting records.
    pub contradicting_records: Vec<MemoryRecordId>,
    /// When the user confirmed it, or `null`.
    #[serde(deserialize_with = "Option::deserialize")]
    pub user_confirmed_at: Option<Instant>,
}

/// `trust-info`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trust {
    /// Value in `0..=1`.
    pub value: TrustValue,
    /// `trust/v1`.
    pub formula: TrustFormulaV1,
    /// When it was computed.
    pub computed_at: Instant,
}

/// Kind of alternative explanation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AlternativeKind {
    /// About someone or something else.
    DifferentSubject,
    /// For another purpose.
    DifferentPurpose,
    /// A one-off context.
    OneOffContext,
    /// An external cause.
    ExternalCause,
}

/// `alternative-explanation`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlternativeExplanation {
    /// Kind.
    pub kind: AlternativeKind,
    /// What it is about instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub about: Option<EntityRef>,
    /// A related record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record: Option<MemoryRecordId>,
}

/// `usage-restriction`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UsageRestriction {
    /// Life areas the record may be used in.
    pub areas: Vec<LifeArea>,
    /// Projects the record may be used in.
    pub projects: Vec<ProjectId>,
}

/// `live-record`: a memory record with every field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LiveMemoryRecord {
    /// `MemoryRecord`.
    #[serde(rename = "type")]
    pub concept: MemoryRecordType,
    /// Identifier.
    pub id: MemoryRecordId,
    /// Common fields.
    #[serde(flatten)]
    pub header: RecordHeader,
    /// The record this version replaces, or `null`.
    #[serde(deserialize_with = "Option::deserialize")]
    pub supersedes: Option<MemoryRecordId>,
    /// Lifecycle.
    pub status: LiveStatus,
    /// Content type.
    pub content_type: ContentType,
    /// Epistemic state.
    pub epistemic_state: EpistemicState,
    /// Layer.
    pub layer: Layer,
    /// Time kind.
    pub time_kind: TimeKind,
    /// Valid time start.
    pub valid_from: Instant,
    /// Valid time end, or `null` while open.
    #[serde(deserialize_with = "Option::deserialize")]
    pub valid_to: Option<Instant>,
    /// Expected end.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_end: Option<Instant>,
    /// Season.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub season: Option<Season>,
    /// When it was superseded, or `null`.
    #[serde(deserialize_with = "Option::deserialize")]
    pub superseded_at: Option<Instant>,
    /// Source episodes, at least one.
    pub source_episodes: NonEmpty<EpisodeId>,
    /// Source kind.
    pub source_kind: SourceKind,
    /// Evidence.
    pub evidence: Evidence,
    /// Trust.
    pub trust: Trust,
    /// Alternative explanations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alternative_explanations: Option<Vec<AlternativeExplanation>>,
    /// Usage restriction, or `null`.
    #[serde(deserialize_with = "Option::deserialize")]
    pub usage_restriction: Option<UsageRestriction>,
    /// Whom or what it is about.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub about: Option<Vec<EntityRef>>,
    /// The statement.
    pub statement: Statement,
}

/// `skeleton`: what remains of a forgotten memory record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemorySkeleton {
    /// `MemoryRecord`.
    #[serde(rename = "type")]
    pub concept: MemoryRecordType,
    /// Identifier.
    pub id: MemoryRecordId,
    /// Owner.
    pub owner: PartyRef,
    /// Last version.
    pub version: core::num::NonZeroU64,
    /// `forgotten`.
    pub status: Forgotten,
    /// Transaction time of the last version.
    pub recorded_at: Instant,
    /// When it was forgotten.
    pub deleted_at: Instant,
    /// Retention class.
    pub retention_class: RetentionClass,
}

/// `memory-record`: live, or the skeleton of a forgotten one.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum MemoryRecord {
    /// Forgotten.
    Forgotten(MemorySkeleton),
    /// Live.
    Live(Box<LiveMemoryRecord>),
}

impl MemoryRecord {
    /// The identifier.
    #[must_use]
    pub fn id(&self) -> &MemoryRecordId {
        match self {
            Self::Forgotten(s) => &s.id,
            Self::Live(l) => &l.id,
        }
    }

    /// The live record, unless forgotten.
    #[must_use]
    pub fn live(&self) -> Option<&LiveMemoryRecord> {
        match self {
            Self::Forgotten(_) => None,
            Self::Live(l) => Some(l),
        }
    }
}

/// `live-relationship-edge`: a memory record with subject, predicate and object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LiveRelationshipEdge {
    /// The record fields.
    #[serde(flatten)]
    pub record: LiveMemoryRecord,
    /// Subject.
    pub subject: EntityRef,
    /// Predicate.
    pub predicate: Predicate,
    /// Object.
    pub object: EntityRef,
}

/// `relationship-edge`: live, or the skeleton of a forgotten one.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum RelationshipEdge {
    /// Forgotten.
    Forgotten(MemorySkeleton),
    /// Live.
    Live(Box<LiveRelationshipEdge>),
}

impl RelationshipEdge {
    /// The identifier.
    #[must_use]
    pub fn id(&self) -> &MemoryRecordId {
        match self {
            Self::Forgotten(s) => &s.id,
            Self::Live(l) => &l.record.id,
        }
    }
}

/// Kind of episode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EpisodeKind {
    /// A conversation.
    Conversation,
    /// A task run.
    TaskRun,
    /// A meeting transcript.
    MeetingTranscript,
    /// An observation window.
    ObservationWindow,
    /// An event from a connected source.
    ConnectedSourceEvent,
    /// An import.
    Import,
}

/// Interaction mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    /// Text.
    Text,
    /// Voice.
    Voice,
    /// Card.
    Card,
    /// Workspace.
    Workspace,
}

/// Actor of an episode entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryActor {
    /// The user.
    User,
    /// The assistant.
    One,
    /// A sub-agent.
    Subagent,
    /// The other party.
    OtherParty,
}

/// Kind of episode entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryKind {
    /// A conversation turn.
    ConversationTurn,
    /// An explicit "remember this".
    ExplicitRemember,
    /// A reference to a source event.
    SourceEventRef,
    /// An imported item.
    ImportedItem,
    /// An observation.
    Observation,
    /// An interaction.
    Interaction,
}

/// Source of an imported item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportSource {
    /// An export package.
    OneExport,
    /// Another assistant's dump.
    AssistantDump,
    /// A transfer protocol.
    TransferProtocol,
}

/// The source object an entry refers to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntrySource {
    /// The source object.
    #[serde(rename = "ref")]
    pub reference: ExternalRef,
    /// When it was read.
    pub read_at: Instant,
}

/// `episode-entry`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EpisodeEntry {
    /// Sequence number.
    pub seq: u64,
    /// Time.
    pub at: Instant,
    /// Actor.
    pub actor: EntryActor,
    /// Mode.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<Mode>,
    /// Kind.
    pub kind: EntryKind,
    /// Source kind.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_kind: Option<SourceKind>,
    /// Provenance label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provenance: Option<ProvenanceLabel>,
    /// Content.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<Statement>,
    /// Source object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<EntrySource>,
    /// Import source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub import_source: Option<ImportSource>,
    /// Observation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observation: Option<ObservationId>,
    /// Interaction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interaction: Option<InteractionId>,
}

/// `live-episode`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LiveEpisode {
    /// `Episode`.
    #[serde(rename = "type")]
    pub concept: EpisodeType,
    /// Identifier.
    pub id: EpisodeId,
    /// Common fields.
    #[serde(flatten)]
    pub header: RecordHeader,
    /// The episode this version replaces, or `null`.
    #[serde(deserialize_with = "Option::deserialize")]
    pub supersedes: Option<EpisodeId>,
    /// `active`.
    pub status: Active,
    /// Kind.
    pub kind: EpisodeKind,
    /// Modes.
    pub modes: Vec<Mode>,
    /// Start.
    pub started_at: Instant,
    /// End.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<Instant>,
    /// Task.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task: Option<TaskId>,
    /// Entries.
    pub entries: Vec<EpisodeEntry>,
    /// Device it is queued on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queued_on_device: Option<DeviceId>,
}

/// `episode-skeleton`: what remains of a forgotten episode.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EpisodeSkeleton {
    /// `Episode`.
    #[serde(rename = "type")]
    pub concept: EpisodeType,
    /// Identifier.
    pub id: EpisodeId,
    /// Owner.
    pub owner: PartyRef,
    /// Last version.
    pub version: core::num::NonZeroU64,
    /// `forgotten`.
    pub status: Forgotten,
    /// Transaction time of the last version.
    pub recorded_at: Instant,
    /// When it was forgotten.
    pub deleted_at: Instant,
    /// Retention class.
    pub retention_class: RetentionClass,
}

/// `episode`: live, or the skeleton of a forgotten one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum Episode {
    /// Forgotten.
    Forgotten(EpisodeSkeleton),
    /// Live.
    Live(Box<LiveEpisode>),
}

fn by_status<'de, D, S, L>(
    deserializer: D,
    skeleton: fn(S) -> L,
    live: fn(serde_json::Value) -> Result<L, serde_json::Error>,
) -> Result<L, D::Error>
where
    D: Deserializer<'de>,
    S: serde::de::DeserializeOwned,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    let forgotten =
        value.get("status").and_then(serde_json::Value::as_str) == Some(Forgotten::VALUE);
    let out = if forgotten {
        serde_json::from_value::<S>(value).map(skeleton)
    } else {
        live(value)
    };
    out.map_err(serde::de::Error::custom)
}

impl<'de> Deserialize<'de> for MemoryRecord {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        by_status(deserializer, Self::Forgotten, |v| {
            serde_json::from_value(v).map(Self::Live)
        })
    }
}

impl<'de> Deserialize<'de> for RelationshipEdge {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        by_status(deserializer, Self::Forgotten, |v| {
            serde_json::from_value(v).map(Self::Live)
        })
    }
}

impl<'de> Deserialize<'de> for Episode {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        by_status(deserializer, Self::Forgotten, |v| {
            serde_json::from_value(v).map(Self::Live)
        })
    }
}
