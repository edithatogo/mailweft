# Quality, security, scaling and optional knowledge graph implementation plan

**Status:** all implementation tasks not started. **Goal:** the bounded specification in `spec.md`.
**Architecture:** retain independent Rust raw-email -> OCEL 2.0 core; optional lanes do not gate core.
**Tech stack:** qualified Rust dependencies; Python helpers only for development-document validation.
**Spec:** `spec.md`, `docs/CONTRACT.md`, `docs/FRONTIER-DESIGN.md`.

## Global constraints and review focus
No real email/source data in Git; no network/model fallback; no participant-role/authority conflation; preserve source attribution and OCEL attributes/qualifiers; observe exact task prerequisites, not whole-track gates. Test unknowns, malicious data, copied quotations, Unicode spans and missing/ambiguous references where applicable.

The manifest is authoritative for dependencies and lane. Checkboxes are completion records, not approval requests. Planning checks do not mark implementation complete.

- [ ] **T07.01 — Design blinded adjudication protocol and error taxonomy**
  - Lane: `research`. Depends on: T06.08.
  - Edit only: `research/ANNOTATION_PROTOCOL.md`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Gold labels not copied from LLM output. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/07_research/evidence.md`. Commit: `T07.01: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.

- [ ] **T07.02 — Define calibration, abstention and false-link evaluation**
  - Lane: `research`. Depends on: T07.01.
  - Edit only: `research/EVALUATION.md`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Per-task PR/F1 and calibration recorded. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/07_research/evidence.md`. Commit: `T07.02: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.

- [ ] **T07.03 — Benchmark rules vs local NLP vs optional local LLM**
  - Lane: `research`. Depends on: T07.02.
  - Edit only: `research/RESULTS_TEMPLATE.md`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Model/version/hardware/cost/privacy controlled. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/07_research/evidence.md`. Commit: `T07.03: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.

- [ ] **T07.04 — Compare no-graph, graph-only, vector and GraphRAG methods**
  - Lane: `research`. Depends on: T07.03, T07.05.
  - Edit only: `research/GRAPH_EVAL.md`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: No advantage claimed without evaluation. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/07_research/evidence.md`. Commit: `T07.04: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.

- [ ] **T07.05 — Define optional temporal evidence graph contract**
  - Lane: `research`. Depends on: T06.08.
  - Edit only: `contracts/v1/graph-contract.md`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Bitemporal, provenance, permissions, explicit alternatives. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/07_research/evidence.md`. Commit: `T07.05: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.

- [ ] **T07.06 — Qualify default-feature parser/exporter fuzz and bounded resource regressions**
  - Lane: `core`. Depends on: T03.08.
  - Edit only: `fuzz/README.md`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: No crash or unbounded allocation on samples. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/07_research/evidence.md`. Commit: `T07.06: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.

- [ ] **T07.07 — Audit default-feature supply chain; optional feature audits remain separate**
  - Lane: `core`. Depends on: T00.06, T03.08.
  - Edit only: `docs/SUPPLY_CHAIN.md`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Only explicitly approved licences/features. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/07_research/evidence.md`. Commit: `T07.07: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.

- [ ] **T07.08 — Produce reproducibility bundle and limitations report**
  - Lane: `research`. Depends on: T07.03, T07.04, T07.06, T07.07.
  - Edit only: `research/REPRODUCIBILITY.md`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Cite data and exact code commit. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/07_research/evidence.md`. Commit: `T07.08: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.
