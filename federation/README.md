# One Federation A2A extension

| | |
| --- | --- |
| Extension URI | `https://suiss.app/a2a/one-federation/v1` |
| Type | A2A v1.0 profile extension, **required** (`required: true`) |
| Target | A2A Protocol Specification v1.0.1, HTTP+JSON binding, media type `application/a2a+json` |
| Schema version | `1.0` (`MAJOR.MINOR`) |
| Schemas | [`schemas/`](schemas/), JSON Schema 2020-12, `$id` under `https://suiss.app/schemas/one/federation/` |

The One Federation extension carries typed negotiation between two parties' agents over A2A: one person's One and another person's One, or One and an organisation's agent. A2A carries the Task, Message, Part and Artifact objects; the extension adds what A2A does not have: typed message kinds, a correlation identifier and offer version for the whole chain, the sender's and recipient's person and agent identities, an authority reference, expiry, first-contact fields, selective disclosure of person attributes and a suspended-negotiation handle.

## Activation

- An agent lists the extension in its Agent Card under `capabilities.extensions` with `required: true` and the parameters of [`agent-extension.v1.json`](schemas/agent-extension.v1.json): the schema versions it accepts, the message kinds it accepts and the capabilities it can negotiate.
- A client sends the URI in the `A2A-Extensions` header on every request. A server refuses a request without it with `ExtensionSupportRequiredError` and lists the URI in the response `A2A-Extensions` header.
- Every Message, and the commitment Artifact, lists the URI in `extensions` and keys its extension object by the URI in `metadata`.

Between two people, the A2A HTTP+JSON request is the plaintext of an end-to-end encrypted MLS (RFC 9420) application message delivered through the recipient's agent mailbox; the mailbox sees only delivery metadata. With an organisation agent, the same request goes directly over TLS.

## Message kinds

| Kind | Meaning | Body | Commitment operation |
| --- | --- | --- | --- |
| `offer` | Proposes something (time, place, work, exchange) | capability + typed fields | `request` |
| `counter-offer` | A new version replacing the open offer | same capability + typed fields | `counter` |
| `commitment` | Accepts the version named by `inReplyTo` and promises it | none | `promise` or `accept-counter` |
| `rejection` | Rejects the version named by `inReplyTo` | reason code | `decline` |
| `query` | Asks for information, or for person attributes by selective disclosure | capability + typed fields, or a disclosure request | opens an information wait |
| `answer` | Answers the query with the least information | typed output values, or an SD-JWT+KB presentation | satisfies the wait |

Typed fields are values of a capability's input or output schema (see [`../capability/`](../capability/)); the extension defines no field dictionary of its own and anything that does not validate is refused, never coerced. [`a2a-mapping.toml`](a2a-mapping.toml) is the full table: which kind a sender in which role may send in which state, the operation it maps to and the Task state after it. Exactly one row matches a valid message; any other message is refused with `operation-not-permitted`.

## Mandatory fields

The envelope ([`envelope.v1.json`](schemas/envelope.v1.json)) is the extension object in `Message.metadata`. Every typed message carries:

- `schemaVersion`, `kind`
- `sender` and `recipient`, each a person and the agent acting for that person (`did:web`)
- `authority`: a reference to the authority proof and its digest; the recipient verifies it with the identity and authority service, and the reference itself is not authority
- `correlationId`: created with the first offer, identical for the whole chain and equal to the A2A `contextId`
- `offerVersion`, `sentAt`, `expiresAt`

`inReplyTo` (message id, offer version, digest and the handle it carried) is required on counter-offers, commitments, rejections and answers and absent on offers. A digest is SHA-256 over the RFC 8785 (JCS) form of the referenced A2A Message without the extension object's `handle` member, written as `sha-256:` and unpadded base64url.

## Offer versioning

- The first offer has `offerVersion` 1.
- Each counter-offer is exactly one greater than the highest version in the chain and names the version it replaces in `inReplyTo`; it keeps the same capability.
- A commitment or rejection answers one named version; a reply to a superseded version is refused.
- `offerVersion` 0 appears only on a query or answer before any offer.

Non-monotonic versions are refused with `version-not-monotonic`, mismatched replies with `reply-version-mismatch`. See [`vectors/offer-version.json`](vectors/offer-version.json).

## Expiry

- `expiresAt` is after `sentAt` and **at most 7 days** (604 800 seconds) later; anything else is refused with `expiry-invalid`.
- Timeliness is judged on the mailbox enqueue time, or for direct A2A on the receiver's receipt time; the sender's `sentAt` is informational.
- A reply enqueued after expiry is refused with `offer-expired`.
- Silence is not an answer. When `expiresAt` passes without a reply, the chain is recorded as **expired**, never as accepted or rejected, and the Task becomes `TASK_STATE_CANCELED`.

See [`vectors/expiry.json`](vectors/expiry.json).

## Notes are untrusted, display-only text

A message has exactly one typed body part and at most one note part ([`note.v1.json`](schemas/note.v1.json)): `text/plain`, at most 2000 characters, labelled `trust: "untrusted"`. A note is for a human only:

- it is shown as the other party's note, as literal characters, after the invisible characters listed in [`note-filter.toml`](note-filter.toml) are removed; markup is never rendered;
- it never becomes a field value, an operation or an action, whatever it says, and an agent never acts on it.

A note that asks to accept while the body rejects is still a rejection. See [`vectors/note.json`](vectors/note.json).

## Suspended-negotiation handle

The sender of an offer, counter-offer or query issues a handle for the reply it awaits; the replying side echoes it unchanged in `inReplyTo.handle`.

- To the replying side the handle is **opaque**: never parse, change or reuse it.
- To its issuer it is attacker-controlled input. It is a JWE compact serialisation with `alg` `dir` and `enc` `A256GCM` under a key only the issuer holds ([`handle.v1.json`](schemas/handle.v1.json), [`handle.toml`](handle.toml)). It binds the sender and recipient identities, the chain, the offer version and the digest of the suspended message, and expires with that message (at most 7 days).
- It is **single use**: the issuer records it as spent atomically with accepting the reply. A reply refused for any reason does not consume it.

Verification order and refusal codes are in [`handle.toml`](handle.toml); test vectors with a test-only key are in [`vectors/handle.json`](vectors/handle.json).

## Tasks

An offer chain is one A2A Task ([`task.v1.json`](schemas/task.v1.json)). The Task is a projection; the canonical state is the commitment and its records. `TASK_STATE_INPUT_REQUIRED` waits for the other party's typed reply, `TASK_STATE_AUTH_REQUIRED` waits for the recipient person's own decision (the reason is never shared), `TASK_STATE_COMPLETED` carries one Artifact referencing the commitment that was created, and `TASK_STATE_FAILED` carries a refusal code.

## First contact and disclosure

[`contact.v1.json`](schemas/contact.v1.json) marks first contact. A first contact is opened only on the person's explicit request, or under an explicit mandate and only towards an organisation agent. The relationship level is never carried; the recipient evaluates it on its own side. Person attributes are disclosed only by SD-JWT VC with key binding ([`disclosure.v1.json`](schemas/disclosure.v1.json)), exactly the requested claims, and a presentation is never authority.

## Versioning

The URI carries the major version. A breaking change takes a new URI (`/v2`), and peers never fall back to an older URI. Inside the envelope, `schemaVersion` is `MAJOR.MINOR`; a MINOR increase only adds optional members, and peers use the highest version both list in their Agent Card.

## Files

| Path | Content |
| --- | --- |
| [`schemas/`](schemas/) | Message, envelope, body, note, contact, disclosure, handle, Task and Agent Card extension schemas |
| [`vectors/schema/`](vectors/schema/) | Schema vectors: inputs and whether they validate |
| [`vectors/`](vectors/) | Behaviour vectors: kind mapping, offer versions, expiry, notes, handles |
| [`a2a-mapping.toml`](a2a-mapping.toml) | Objects, kinds, expiry and Task states mapped to A2A |
| [`a2a-conformance.toml`](a2a-conformance.toml) | A2A v1.0.1 conformance checks |
| [`handle.toml`](handle.toml) | Handle format, lifetime, binding, single use and verification order |
| [`note-filter.toml`](note-filter.toml) | Characters removed from a note before display |
| [`index.toml`](index.toml) | Index of the above |
