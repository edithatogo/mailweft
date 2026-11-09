# Track 05b — evidence and implementation boundary

Status: **PARTIAL BASELINE IMPLEMENTED**. Do not mark T05b.01–08 complete: the planned nested `mailweft-semantics` contract, person-linking, multi-class benchmarks and the full source byte/character map are **not** implemented.

A narrower, tested source-provenanced Stage 1.5 baseline lives in `mailweft-core::inspect_body_zones` and the `mailweft inspect` CLI. It produces source-part and decoded-text hashes, exact raw offsets only when bytes match, `derived` offset status otherwise, deterministic candidate zone labels, and no body text. HTML-only fallback is not reclassified as source plaintext. Synthetic regressions verify all-line coverage, source-offset roundtrip on raw UTF-8, derived offsets for base64, no plaintext leakage, absence of side effects on the authoritative OCEL, and rejection of unstructured source.

A separate OCEL regression checks the `mime_attachment` type and attribute declarations; earlier v0.3 output could omit the catalogue entry even when Rust4PM import succeeded. The change is versioned 0.4.0 (Rust crates), `Conversion.schema_version=1.3.0`.

Targeted tests: `cargo test --locked -p mailweft-core --test source_zones`; full Rust and independent Rust4PM acceptance: `cargo test --locked --workspace --all-targets` and `cargo test --locked -p mailweft-core --features rust4pm-validation --test rust4pm_interop`. Include exact final GitHub Actions links and checks after running. No real email corpus was ingested. This baseline does not claim Quagga/Zebra parity, calibrated NLP, verified authorship, HTML mapping, or full text span provenance for encoded MIME.
