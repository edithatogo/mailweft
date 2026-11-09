# Track 01 — source-envelope and bounded-reader evidence

## 2026-11-09: T01.01 implementation slice; T01.07 remains partial

Base: f6999713b1c42f1fc7d56d8a16bac4df649f8bf7. Source envelope, semantic metadata fingerprint, byte hash/length verification, opaque receipt, stable errors and counted limit+1 readers are implemented in `crates/mailweft-core/src/source.rs`. All CLI modes use bounded reads; `convert-evidence` uses the strict ingress.

RED: commit 427418bdb6bd41edd2b2cacf05fe89f1f04208aa, Rust run 42587746942/job 129061531302: all five new source-envelope CLI tests failed for missing behavior, not compilation errors. GREEN for those tests and three reader unit tests: commit 190f050dab3be69a1bea0a6baed351588bf2154b, run 42588267305/job 129063252374. That run then correctly failed four separate zoning tests, so it is **not** a whole-suite success.

Additional tests cover metadata revisions preserving the OCEL, schema roundtrip/order invariance, duplicate JSON fields, unknown producer keys and absent observation timestamps. The runtime contract checker validates actual CLI output and negative mutations. Final full-suite/format/Clippy/Rust4PM results are recorded on the PR and final commit, not inferred from partial runs.

This slice satisfies byte-exact source verification and bounded file reading but does not complete the full ingestion programme. Outstanding: corpus accepted/rejected reconciliation, maildir/root confinement, source-specific rights enforcement, authentication, archive completeness verification and independent FOI producer qualification. Do not mark T01.07 complete or imply a live acquisition occurred. No real correspondence used.
