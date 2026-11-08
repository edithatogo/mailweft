# Native converter execution ledger — 8 November 2026

Baseline: `14378d44f581d4dc0fddcb92c5d70b82439c90b4`. Feature branch: `feat/raw-email-ocel`. [PR #3](https://github.com/edithatogo/mailweft/pull/3).

## Architecture and scope

Two crates, mailweft-core and mailweft-cli; internal modules implement identity, parsing, OCEL projection and orchestration. No Enron-specific behavior, Python runtime, model, graph, source capture or FOI dependency. All fixtures are fictional. Runtime work is bounded conversion, not advanced research.

Local execution had no Rust toolchain or outbound DNS. Native red/green verification therefore ran on isolated GitHub-hosted branches. A temporary branch-only workflow resolved Cargo.lock and applied rustfmt; it must be retired before completion. Regular native/planning/dependency CI is read-only. No claim of local Rust execution or independent human review is made.

## Observed evidence

| Checkpoint | Actual result |
|---|---|
| Red: run 42477467667, job 128721894765 | Native dependencies and stub compiled; all 19 conversion tests failed on NOT_IMPLEMENTED as intended. |
| Header/CLI green, MIME red: run 42478153660, job 128724032484 | 19 conversion + 3 CLI tests passed; all three new MIME tests failed at explicit unsupported-profile guard. |
| MIME green: run 42478444974, job 128724897292; source 79e0a900d416770fe8ede6722c904695af2468f2 | All 25 native tests, rustfmt and Clippy -D warnings passed. |
| Additional hardening: run 42479026253 | Chronology test found equal instants/different offsets could produce input-order-dependent serialized output. Other eight hardening cases, prior core/MIME and CLI tests passed on Linux. |
| Dependency audit: run 42479026253, job 128726680842 | Licences/sources OK. Internal path version omission failed bans; two quick-xml advisories failed security. Findings were not suppressed. |
| Fix 46a8931f9fcabad7537dca6093425531eef321c4 | Offset-bearing fingerprint and exact internal path-dependency version. PR run 42479668859 verified Linux and macOS native tests/fmt/Clippy; Linux real-CLI schema output passed. Remaining exact-head status is read from CI, not inferred. |

## Task disposition

Completed foundation deliverables are limited to direct dependency inventory, existing source-exclusion policy checks, the two-crate workspace and pinned toolchain/lock. Broad ingestion/identity/OCEL tasks remain in progress where only their initial profile is implemented. T00.06 remains blocked by security issue #4 even though native/planning CI is implemented. No release, corpus, NLP, GraphRAG or FOI task is marked complete.

See docs/CORE-CHECKPOINT.md for exact API, limits, diagnostics and limitations. Native test assertions supply the previously missing concrete interfaces for this slice; future tasks must bind their own tests to the actual public API, not invent a second representation.

## Required follow-through

Read exact-head results after final documentation/status commit. Preserve red failures as history. Do not merge/release while issue #4 is unresolved. Never claim test-suite coverage as proof of full standard interoperability or safe live deployment.
