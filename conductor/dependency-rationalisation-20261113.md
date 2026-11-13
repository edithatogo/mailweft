# Rust dependency rationalisation — mailweft (2026-11-13)

**Status:** scoped Conductor adoption contract; code substitutions not yet implemented or certified by this file.

## Repository-specific decisions

- Link to existing tracks 00, 03, 07, 08, 10 and 11; the generated track registry remains authoritative.
- **Verified:** the core uses `mail-parser`, Serde, Chrono, `thiserror` and optional Rust4PM `process_mining`; CLI argument handling is manual in `crates/mailweft-cli/src/main.rs` and `reconcile.rs`.
- **Adopt if proven:** Clap for subcommands and positional validation after golden tests for existing invocation, `--allow-partial`, error codes, JSON output and exit statuses 0/1/2.
- **Extend:** Proptest for OCEL identity, duplicate handling, batch permutations and resource caps; Criterion on synthetic fixed-size EML batches.
- **Conditional:** Rayon only if fixed-seed and selection-order replay, capacity/rejection semantics and memory limits remain identical.
- **Retain:** existing MIME parser, Rust4PM interoperability and bounded/offline source policy. Do not add Tokio, Reqwest, SQLx or Ratatui to this offline converter.

## Delivery and acceptance contract
- Reconcile existing Conductor tracks and ownership before editing code; do not duplicate completed tasks.
- Write a failing behavioural/regression test first for each runtime change; preserve APIs, schema versions, stdout/stderr, exit codes, error codes and MSRV or explicitly version breaking changes.
- Run the existing repo gates: targeted tests, full workspace tests, fmt, clippy and contract/interop checks. Record actual passing CI SHA; this document is planning, not validation.
- Inventory duplicate crate versions/features with `cargo tree -d` and `cargo tree -e features`; review unused dependencies, `cargo audit`, `cargo deny`, licence, MSRV, SBOM and release implications where applicable.
- Benchmark performance proposals with fixed fixtures and baseline/candidate receipts, including correctness, memory, CPU and determinism.
- Decision for each candidate must be recorded as adopt, retain, reject or defer with evidence, risks and rollback.
- Do not expose identifiable, secret, copyrighted or otherwise restricted raw data in tracing, snapshots, benchmarks or CI artifacts.
