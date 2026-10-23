# Export format

A person can export everything One holds about them as one package: `app.suiss.one.export`. The format is defined in CDDL (RFC 8610) in [`export.cddl`](export.cddl) and encoded as UTF-8 JSON, so ordinary tools can read it without One.

The package holds:

- `memory`: memory records, relationship edges and episodes; a forgotten record or episode appears only as its skeleton (identifier, owner, version, status `forgotten`, times and retention class);
- `sensitive`: the same for records touching a specially protected class, unless listed in `excluded_sections`;
- `personality`: the assistant profile and the behaviour-rule records;
- `grants` and `connections`: as read from the identity and authority service at export time;
- `tasks`: tasks with their steps and outcomes;
- `adapter`: the personal model adapter as safetensors with its SHA-256, or `null`; the weights work only with the base model named in the package;
- `text_view`: a human-readable summary.

## Compatibility

- `schema_version` starts at 1. Versions are backward compatible: a new version only adds fields.
- Every map in the format is open (`* tstr => any`). A reader **skips fields it does not know**, at every level, and still reads the package.
- A required field that is missing, or a value that breaks its rule (identifier prefix, RFC 3339 UTC instant, enumerated value, trust in `0..1`), makes the package unreadable.

## Reader

[`reader/`](reader/) is `esuiss-one-export-reader`, a Rust crate that decodes a package into typed structs with these rules. Its tests run every vector in [`vectors/`](vectors/).

## Vectors

| Path | Content |
| --- | --- |
| [`vectors/export-schema.json`](vectors/export-schema.json) | Packages and whether they are valid against `one-export` |
| [`vectors/round-trip.json`](vectors/round-trip.json) | Decoding then encoding keeps every field, Turkish text byte for byte |
| [`vectors/unknown-fields.json`](vectors/unknown-fields.json) | Unknown fields are skipped at every level; a later version is still read |

The `grants` and `connections` items keep the identity and authority service's own item schema, named in each projection's `schema` field; the reader keeps those items as JSON objects.
