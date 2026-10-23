# One Open

**The open contracts and client libraries for connecting to One.**

One is Suiss's personal agent: user-owned and independent of any single model. One itself is closed source and operated only by Suiss. This repository holds everything another party needs to work with it: the wire format One agents speak to each other and to organisation agents, the definition format for capabilities, the format of a user's data export, and the libraries that read and write them.

[![CI](https://github.com/e-suiss/one-open/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/e-suiss/one-open/actions/workflows/ci.yml)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
![Status: pre-alpha](https://img.shields.io/badge/status-pre--alpha-orange.svg)

---

## Status

Pre-alpha. The contracts are versioned and covered by test vectors, but nothing has been released yet and no package is published.

## What is here

| Folder | What it is |
| --- | --- |
| [`federation/`](federation/) | The **One Federation** A2A extension (`https://suiss.app/a2a/one-federation/v1`): typed offers, counter-offers, commitments, rejections, queries and answers between agents. JSON Schemas, test vectors and the A2A mapping tables. |
| [`capability/`](capability/) | The capability definition format and capability naming rules, with JSON Schemas and test vectors. |
| [`export/`](export/) | The export package format (`app.suiss.one.export`) in CDDL, its JSON test vectors, and [`esuiss-one-export-reader`](export/reader/), a Rust crate that reads an export without One. |
| [`sdk/`](sdk/) | [`@esuiss/one-sdk`](sdk/), a TypeScript package for capability providers and agents: typed definitions of the capability and federation formats, a capability name validator, capability negotiation and note display. |
| [`schemas/`](schemas/) | Shared JSON Schemas (identifiers, references, data classes, provenance labels) that the federation and capability schemas refer to. |
| [`tools/vectors/`](tools/vectors/) | The check that compiles every schema and runs every schema vector; it runs in CI. |

Every JSON Schema carries its `$id` under `https://suiss.app/schemas/one/`. Each folder's `index.toml` lists its schemas, vectors and tables.

## How it fits together

- **Agents talking to each other.** A person's One and another party's agent (another person's One, or an organisation's agent such as an airline or a bank) exchange typed A2A messages with the One Federation extension. An agent that implements the extension can negotiate with One; free text never turns into an action.
- **Capability providers.** A provider describes what it can do as a capability definition: a semantic, provider-independent name, typed input and output schemas, actions, consequence tier, reversibility and data access. A definition is a description, never authority; authority comes from the person's grants in Access.
- **Export.** A person can export everything One holds about them as one JSON document. The reader in this repository decodes it without One, so the data stays usable even without the service.

One works with the other Suiss products: identity and authority come from [Access](https://github.com/e-suiss/access), delivery and waits from [Relay](https://github.com/e-suiss/relay), and commitments between people live in [Work](https://github.com/e-suiss/work).

## Development

You need Rust (installed from `rust-toolchain.toml` on first use) and Node.js (version in `.node-version`).

```sh
git clone https://github.com/e-suiss/one-open.git
cd one-open
cargo test --workspace --locked        # export reader and every schema vector
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo fmt --all --check
cd sdk && npm ci && npm run typecheck && npm test
```

## Contributing

Contributions are welcome. Read [CONTRIBUTING.md](CONTRIBUTING.md) before opening a pull request, and report vulnerabilities privately as described in [SECURITY.md](SECURITY.md).

## License

Everything in this repository is open source under the [Apache License 2.0](LICENSE). One itself is not part of this repository and is not open source.
