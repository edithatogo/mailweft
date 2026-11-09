# Track 05b — partial structural baseline and explicit exclusions

Status: **PARTIAL IMPLEMENTATION**, not complete T05b.01–08 acceptance.

Rust 0.4 added content-free source hashes/offsets and line zones; 0.6 adds explicit origin scope, nested quote depth, forwarded-block context, inline header labels, disclaimer precedence and a skipped-part ledger. Existing exact/derived offset semantics and immutable Stage 1 OCEL remain unchanged.

RED for the new regression boundaries: commit 190f050dab3be69a1bea0a6baed351588bf2154b, run 42588267305/job 129063252374. Four executed tests reproduced forwarded passages labelled `authored`, disclaimers labelled `signature`, missing quote depth and silent oversized-part exclusion. No performance claim is based on these synthetic regressions.

Verification commands: `cargo test --locked -p mailweft-core --test zoning_boundaries`; `cargo test --locked -p mailweft-core --test source_zones`; full workspace and Rust4PM tests; `python3 scripts/validate_runtime_contracts.py` against the built CLI; fmt and all-feature Clippy. Final run evidence belongs to the exact PR/head.

The body-evidence 1.1 schema describes this implemented sidecar, not the older proposed nested email-zone contract. Open: raw-byte/decoded-character maps for transfer encodings, HTML zoning, author attribution, person/role linking, multi-label speech acts, calibrated multilingual evaluation and independent corpora. Forwarded origin remains conservative to end-of-part, potentially losing recall rather than inventing new authored acts. No payload or private correspondence is tracked.
