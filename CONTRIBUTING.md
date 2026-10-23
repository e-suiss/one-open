# Contributing to One Open

Thank you for your interest. This repository holds the open contracts and client libraries for connecting to One. One itself is closed source and is not developed here.

The project is pre-alpha. The rules below are binding and enforced in CI.

## Reporting security issues

Do **not** open a public issue for a vulnerability. Follow [`SECURITY.md`](SECURITY.md).

## Development environment

You need Rust (installed from `rust-toolchain.toml` on first use) and Node.js (version in `.node-version`; npm 11).

```sh
git clone https://github.com/e-suiss/one-open.git
cd one-open
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo fmt --all --check
cd sdk && npm ci && npm run typecheck && npm test
```

## Rules

- Code, comments, commit messages and pull requests are in English.
- **Contracts first.** A change to a schema, CDDL file, table or vector is a contract change and comes before, or together with, the code that depends on it. Every schema change carries vectors that show the new behaviour. Released schema versions only grow: a new optional member is a MINOR change; anything that breaks an existing reader takes a new version (and, for the federation extension, a new URI).
- Every JSON Schema is listed in its folder's `index.toml` with its `$id`; schema vectors are listed there too and run in CI (`tools/vectors`).
- Rust: `rustfmt`, the workspace lints, warnings are errors. No `unsafe`; no panics in library code; arithmetic is checked.
- TypeScript: every package is its own npm project with its own `package.json`, `package-lock.json` and `.npmrc`; there is no JavaScript at the repository root. `strict` mode with `noUncheckedIndexedAccess` and `exactOptionalPropertyTypes`; no `any`.
- Dependencies: exact versions, committed lockfiles (`Cargo.lock`, `package-lock.json`), at least 7 days between a release and its adoption, no install scripts (`ignore-scripts=true`), only crates.io and the npm registry as sources, licenses as allowed by `deny.toml`. Explain a new dependency (why, maintenance, license) in the pull request.
- GitHub Actions are pinned by full commit SHA.
- Tests are named after the behaviour they prove; flaky tests are fixed or removed, never retried until green.
- No secrets or personal data in code, tests or vectors; vectors use synthetic data.

## Commits and changes

- Commit messages and pull request titles follow [Conventional Commits](https://www.conventionalcommits.org/), with the area as scope: `feat(federation): add a refusal code`, `fix(export-reader): reject a negative trust value`.
- Commits must be signed. `main` has a linear history; force pushes are not allowed.
- External contributions come as pull requests, are kept small and are merged with squash merge after CI passes and a maintainer reviews them. The maintainer pushes signed commits to `main` directly; CI runs after the push.

## Code of conduct

Participation in this project is governed by the [Code of Conduct](CODE_OF_CONDUCT.md).
