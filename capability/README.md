# Capability definitions and names

A capability provider describes what it can do as a **capability definition** ([`definition.schema.json`](definition.schema.json), `$id` `https://suiss.app/schemas/one/capability/definition/v1.json`). A definition is a description, never authority: the person's grants decide what may run.

A definition carries:

- a semantic, provider-independent **name** and a dated **version** (`YYYY-MM-DD`; newest is highest);
- a bilingual meaning (`tr` and `en`) and at least one Turkish and one English example;
- exactly one `primary` action, plus `undo` when reversible or `compensate` when compensable;
- closed JSON Schema 2020-12 **input and output schemas**, whose property names never name passwords, tokens, secrets, cookies or card data;
- the grant scope and consequence tier (`CT0` to `CT3`), the autonomy ceiling, reversibility, the data classes it reads and writes, the surfaces an approval may be given on, its providers and its test status;
- optional projections to an MCP tool, an A2A skill, Apple App Intents, Android App Functions or UCP. An MCP tool name and an A2A skill id are the capability name verbatim; a UCP projection is only for `app.suiss.commerce` capabilities.

Irreversible capabilities and capabilities with consequence tier `CT2` or `CT3` are approved only on a phone or desktop with a passkey.

## Names

[`name.schema.json`](name.schema.json) and [`naming.toml`](naming.toml) define the grammar:

- **Suiss names** live under `app.suiss` with two to five lower-case segments, the first being the category: `app.suiss.transport.ride.request`. Product names (`access`, `relay`, `work`, `pay`) and `one` (the reverse form of the handle subdomain) are reserved as first segments: names carry meaning, not providers.
- **Third-party names** live under the reverse DNS of a domain the publisher controls (`com.example-air.flight.book.seat_upgrade`) and must `extends` at least one parent, so their meaning reaches a Suiss name.
- At most 128 characters, lower case only.

**Negotiation** keeps a name only when both sides declare it with the same `extends`, picks the highest common version, and keeps an extension only when its `extends` chain reaches a kept Suiss name. Invalid declarations are ignored. The steps are in [`naming.toml`](naming.toml); [`../sdk/`](../sdk/) implements them.

## Files

| Path | Content |
| --- | --- |
| [`definition.schema.json`](definition.schema.json) | Capability definition |
| [`name.schema.json`](name.schema.json) | Names, versions, `extends`, negotiation declarations and capability references |
| [`naming.toml`](naming.toml) | Grammar, error codes, reserved segments and negotiation steps |
| [`vectors/definition.json`](vectors/definition.json) | Schema vectors for definitions |
| [`vectors/projection.json`](vectors/projection.json) | Definitions whose MCP tool or A2A skill differs from the capability name, which a schema cannot express |
| [`vectors/name.json`](vectors/name.json) | Name and version parsing vectors |
| [`vectors/negotiation.json`](vectors/negotiation.json) | Negotiation vectors |
