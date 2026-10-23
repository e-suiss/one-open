//! References of `export.cddl`: every reference is `{ref, …}` where `ref` names its kind.

use serde::{Deserialize, Serialize};

use crate::scalar::{OpaqueId, RecordId, const_tag};

const_tag!(
    /// `ref: "PartyRef"`.
    PartyRefTag,
    "PartyRef"
);
const_tag!(
    /// `ref: "InstanceRef"`.
    InstanceRefTag,
    "InstanceRef"
);
const_tag!(
    /// `ref: "ConnectedAccountRef"`.
    ConnectedAccountRefTag,
    "ConnectedAccountRef"
);
const_tag!(
    /// `ref: "ExternalPersonRef"`.
    ExternalPersonRefTag,
    "ExternalPersonRef"
);
const_tag!(
    /// `ref: "ExternalRef"`.
    ExternalRefTag,
    "ExternalRef"
);
const_tag!(
    /// `ref: "RecordRef"`.
    RecordRefTag,
    "RecordRef"
);

/// `party-ref`: a person or agent party, by the identity service's opaque identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PartyRef {
    /// `PartyRef`.
    #[serde(rename = "ref")]
    pub tag: PartyRefTag,
    /// Opaque identifier.
    pub id: OpaqueId,
}

/// `instance-ref`: an agent instance, by opaque identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstanceRef {
    /// `InstanceRef`.
    #[serde(rename = "ref")]
    pub tag: InstanceRefTag,
    /// Opaque identifier.
    pub id: OpaqueId,
}

/// A connected account an external reference was read through.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectedAccountRef {
    /// `ConnectedAccountRef`.
    #[serde(rename = "ref")]
    pub tag: ConnectedAccountRefTag,
    /// Opaque identifier.
    pub id: OpaqueId,
}

/// Source system of an external person.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PersonSystem {
    /// Contacts.
    Contacts,
    /// E-mail.
    Email,
    /// Messaging.
    Messaging,
}

/// Source system of an external object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalSystem {
    /// Contacts.
    Contacts,
    /// E-mail.
    Email,
    /// Calendar.
    Calendar,
    /// Messaging.
    Messaging,
    /// Document.
    Document,
    /// Files.
    Files,
    /// Photos.
    Photos,
    /// Web.
    Web,
    /// Place.
    Place,
    /// Agent.
    Agent,
    /// Service.
    Service,
}

/// `external-person-ref`: a person outside Suiss, as a pointer to a source entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalPersonRef {
    /// `ExternalPersonRef`.
    #[serde(rename = "ref")]
    pub tag: ExternalPersonRefTag,
    /// Source system.
    pub system: PersonSystem,
    /// Source-side locator.
    pub locator: String,
    /// The account it was read through.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account: Option<ConnectedAccountRef>,
}

/// `external-ref`: an object held by a source system.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalRef {
    /// `ExternalRef`.
    #[serde(rename = "ref")]
    pub tag: ExternalRefTag,
    /// Source system.
    pub system: ExternalSystem,
    /// Source-side locator.
    pub locator: String,
    /// The account it was read through.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account: Option<ConnectedAccountRef>,
}

/// `record-ref`: a goal or routine record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordRef {
    /// `RecordRef`.
    #[serde(rename = "ref")]
    pub tag: RecordRefTag,
    /// Goal or routine identifier.
    pub id: RecordId,
}

/// `entity-ref`: what a record is about.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EntityRef {
    /// A Suiss party.
    Party(PartyRef),
    /// A person outside Suiss.
    ExternalPerson(ExternalPersonRef),
    /// A source-held object.
    External(ExternalRef),
    /// A goal or routine.
    Record(RecordRef),
}
