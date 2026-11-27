# Contributing to mailweft

Mailweft converts supplied RFC 5322/MIME email into portable OCEL 2.0 communication logs. Contributions should strengthen source fidelity, conservative mappings, diagnostics, interoperability and reproducible synthetic verification.

## Before you start

Read [AGENTS.md](AGENTS.md), [the Conductor index](conductor/index.md), [execution order](docs/EXECUTION-ORDER.md), and the single assigned task's specification and plan. Check current ownership before editing. Preserve the canonical Rust model, OCEL qualifiers and typed values. `foi-process` consumes Mailweft; source capture, preservation and ontology promotion remain separate responsibilities.

For bugs, provide the affected version or commit, environment, minimal reproduction, expected result and observed result. For a substantial proposal, describe the use case and compatibility impact in an issue before expanding implementation scope. Use existing issue templates when offered.

## Development and validation

Use [rust-toolchain.toml](rust-toolchain.toml), the committed lockfile and the task's specified checks. The existing [Rust workflow](.github/workflows/rust-core.yml) includes:

```sh
cargo fmt --all -- --check
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-targets --no-fail-fast
cargo test --locked -p mailweft-core --features rust4pm-validation --test rust4pm_interop
```

For output-contract changes, also follow the workflow's development-requirements setup and runtime/selection validation commands. Record the actual test count; a filter selecting zero tests is not acceptance. Changes to externally visible formats need the existing versioning and migration process.

Use invented synthetic mail fixtures. Preserve source observations separately from claimed headers, inferred relationships and downstream interpretations. Enron is optional verification data; legacy downstream output is not an independent expected result.

## Pull requests

Create a focused branch from the current `main` branch and submit a pull request to this repository. Link the relevant issue or existing Conductor task, explain the resulting behaviour, and update affected examples, contracts and documentation. Keep unrelated changes in separate pull requests.

Report executed commands, outcomes and the commit they validate. Identify unavailable checks and remaining limitations explicitly. Follow the repository's existing review and automated-check requirements; retain required gates and current ownership. Address findings and verify the final head before integration. Repository changes do not themselves authorise a release or external operational action.

## Security, rights and conduct

Follow the [security policy](https://github.com/edithatogo/.github/blob/main/SECURITY.md). Use private vulnerability reporting if available; otherwise request a private contact without publishing vulnerability details. Keep credentials, personal information and restricted source material out of issues, fixtures, logs and artifacts.

Read the [existing licence](LICENSE) and preserve attribution and third-party terms. Follow the [code of conduct](https://github.com/edithatogo/.github/blob/main/CODE_OF_CONDUCT.md) and keep technical discussions respectful.

## Basis for this guide

This guide follows [GitHub's contribution-guideline guidance](https://docs.github.com/en/communities/setting-up-your-project-for-healthy-contributions/setting-guidelines-for-repository-contributors) and the contribution-process and testing principles in [OpenSSF Best Practices](https://www.bestpractices.dev/en/criteria). It does not assert certification.
