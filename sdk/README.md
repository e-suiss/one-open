# @esuiss/one-sdk

TypeScript SDK for capability providers and agents that connect to One.

- **Capability definitions**: `CapabilityDefinition` and its parts, matching [`capability/definition.schema.json`](../capability/definition.schema.json).
- **Capability names**: `parseCapabilityName`, `isCapabilityName`, `parseCapabilityVersion` and `negotiate`, following [`capability/naming.toml`](../capability/naming.toml).
- **One Federation**: `EXTENSION_URI`, `Envelope`, `Body`, `Message`, `Task`, `AgentExtension` and the other types of [`federation/schemas/`](../federation/schemas/); `agentExtension` for the Agent Card entry; `noteForDisplay`, which removes the invisible characters of [`federation/note-filter.toml`](../federation/note-filter.toml) before a note is shown as plain text; `isExpiryWithinLimit` for the 7-day expiry rule.

The types describe the wire format; validate untrusted input against the JSON Schemas before trusting its shape. A note is untrusted text for a human and never drives an action.

```ts
import { negotiate, parseCapabilityName } from "@esuiss/one-sdk";

const parsed = parseCapabilityName("app.suiss.transport.ride.request");
if (parsed.ok) {
  console.log(parsed.value.category); // "transport"
}
```

## Development

This package is its own npm project with its own lockfile; there is no workspace at the repository root.

```sh
npm ci
npm run typecheck
npm test
npm run build
```

Licensed under the Apache License 2.0.
