# esuiss-one-export-reader

Reads a One export package (`app.suiss.one.export`, defined in [`../export.cddl`](../export.cddl)) without One.

- Decodes the JSON package into typed structs.
- Checks every value against its CDDL rule: typed identifier prefixes, RFC 3339 UTC instants, enumerated values, non-empty lists, trust in `0..1`.
- Refuses a package with a missing required field; nullable fields must be present.
- Skips fields it does not know at every level, so a version 1 reader reads later versions, which only add fields. Writing a package back emits only the fields the reader knows.

```rust
let package = one_export_reader::from_str(&std::fs::read_to_string("export.json")?)?;
println!("{} records", package.memory.records.len());
```

Licensed under the Apache License 2.0.
