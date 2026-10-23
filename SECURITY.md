# Security Policy

## Reporting a vulnerability

Please report vulnerabilities **privately** through GitHub's private vulnerability reporting:
**Security → Report a vulnerability** on this repository.

Do not open a public issue, pull request or discussion for a suspected vulnerability.

Please include:
- a description of the issue and its impact,
- steps to reproduce or a proof of concept,
- affected versions or components, if known.

## What to expect

- We acknowledge reports and keep you informed while we investigate.
- We follow coordinated vulnerability disclosure with a default embargo of 90 days, shortened when a vulnerability is actively exploited.
- Advisories are published as GitHub Security Advisories, and reporters are credited unless they ask otherwise.

## Supported versions

The project is pre-alpha and nothing has been released yet. Report issues against `main`; fixes land on `main`.

## Scope

Everything in this repository: the federation extension and its schemas, the capability and export formats, the export reader, the SDK, and the CI workflows. A weakness in a contract (for example a message that a conforming receiver would act on although it should be refused) is in scope. Reports about One itself are welcome through the same private channel.
