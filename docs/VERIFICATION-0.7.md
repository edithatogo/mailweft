# Qualification record — corpus reconciliation, transfer maps and Conductor

Base: 20a94560f6af91b2bc3764bd5bce93ddf0dff21d. Branch feat/corpus-mapping-conductor. All fixtures are invented; no source acquisition or real corpus processing.

## Observed RED

Commit ac1bff1fdb4ddd0ec8b6bfe4159c310b24fbb9f8:
- Rust run 42592315276/job129076633696: four corpus CLI regressions and four transfer-provenance regressions failed because behavior was absent; existing tests passed.
- Planning run42592315302/job129076633717: five Conductor tests exposed missing per-track indexes/doctor and the false rust_workspace_implemented flag. The other 17 planning tests passed.

Final review at 38a3ff868de56276a76923e248c8102db6ff32e0, planning run42626883968/job129192829247, added two observed failures: the pinned implement command could not parse list-style registry entries, and the doctor failed to reject a completed task with an unaccepted prerequisite. The existing accepted task graph itself passed the prerequisite check.

## Executed GREEN and final gate

Source checkpoint 4ef2bf7160320f26728515c46868e09fc22ef61d passed read-only Rust run42596001104 and planning run42596001113. This includes selection reconciliation, encoded-transfer provenance, Rust4PM, output-contract checks, format and Clippy.

Protocol correction 18e258657764bcae69c65675ab905fbc61eb9571 passed planning run42627398022 including 25 tests. The renderer and tracked registry now use the upstream `## [status] Track:` grammar. The dependency guard rejects premature completion even after context is re-synchronized.

Ordinary read-only workflows rerun on the final PR and merged commit. Record their links in the PR receipt; do not relabel historical checks as final-head evidence. No local Rust or network checkout was available in this assistant environment, so hosted Actions is the actual compilation and execution environment. The whole-branch source review was performed here; no independent reviewer agent was available.

Completed manifest entries summarize accepted narrow contracts, not whole scientific requirements. In-progress entries retain explicit remaining work. Conductor sync never decides completion. The pinned upstream checkout and six command TOMLs are verified, with a consumer-protocol regression; no user-host extension registration or end-to-end Gemini model session is claimed.

## Rulings

- Do not make parser integration depend on maildir traversal: supplied bytes work without directory discovery. T01.03 depends on the source boundary instead.
- Claimed-time extraction depends on parser qualification, not completion of every address/header edge case.
- Conductor repository integrity does not depend on unrelated vulnerability-scan completion; T00.07 depends on the actual workspace and toolchain pin. Release security qualification remains separate and unfinished.
- Existing modules are authoritative implementations; amend obsolete proposed paths rather than add duplicate code merely to satisfy filenames.
- Source selection completeness never proves all correspondence is present. Partial projection is opt-in and keeps exit status 2; failure receipts contain no private path or body.
- Transfer maps are byte/quantum relations, not exact character positions. Malformed/ambiguous encoding, charset changes and map bounds abstain explicitly.
- Resume the existing isolated branch and preserve its commits rather than repeat already implemented work. Main is changed only after final PR checks.

Residual work: directory confinement, streaming, full typed OCEL/format roundtrip, broader header/identity edge cases, dependency advisory issue #4, semantic NLP, model/graph evaluation, real-data rights and independent FOI consumer qualification. These remain tracked, not silently marked complete.
