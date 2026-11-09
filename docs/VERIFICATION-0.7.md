# Qualification record — corpus reconciliation, transfer maps and Conductor

Base: 20a94560f6af91b2bc3764bd5bce93ddf0dff21d. Branch feat/corpus-mapping-conductor. All fixtures are invented; no source acquisition or real corpus processing.

## Observed RED

Commit ac1bff1fdb4ddd0ec8b6bfe4159c310b24fbb9f8:
- Rust run 42592315276/job129076633696: four corpus CLI regressions and four transfer-provenance regressions failed because behavior was absent; existing tests passed.
- Planning run42592315302/job129076633717: five Conductor tests exposed missing per-track indexes/doctor and the false rust_workspace_implemented flag. The other 17 planning tests passed.

## Acceptance commands

The preparation job must run locked workspace tests, Rust4PM tests, actual-output schema checks, fmt, all-feature Clippy, planning checks and Conductor integrity before committing generated status acceptance. The ordinary read-only workflows must rerun on the final candidate and merged commit. Inspect those exact commit checks/PR receipts for final results; the presence of this document does not assert tests have already passed.

Completed manifest entries summarize accepted narrow contracts, not whole scientific requirements. In-progress entries retain explicit remaining work. Conductor sync never decides completion. The pinned upstream checkout and six command TOMLs are verified; no user-host extension registration is claimed.

## Rulings

- Do not make parser integration depend on maildir traversal: supplied bytes work without directory discovery. T01.03 depends on the source boundary instead.
- Claimed-time extraction depends on parser qualification, not completion of every address/header edge case.
- Conductor repository integrity does not depend on unrelated vulnerability-scan completion; T00.07 depends on the actual workspace and toolchain pin.
- Existing modules are authoritative implementations; amend obsolete proposed paths rather than add duplicate code merely to satisfy filenames.
- Source selection completeness never proves all correspondence is present. Partial projection is opt-in and keeps exit status2; failure receipts contain no private path or body.
- Transfer maps are byte/quantum relations, not exact character positions. Malformed/ambiguous encoding, charset changes and map bounds abstain explicitly.

Residual work: directory confinement, streaming, full typed OCEL/format roundtrip, broader header/identity edge cases, semantic NLP, model/graph evaluation, real-data rights and independent FOI consumer qualification. These remain tracked, not silently marked complete.
