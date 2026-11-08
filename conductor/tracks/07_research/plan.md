# Quality, security, scaling and optional knowledge graph — executable task plan

Read the track spec, AGENTS.md and the task manifest. Each unchecked task is unimplemented. Dependencies are exact task IDs, not whole-track gates.

## [ ] **T07.01 — Design blinded adjudication protocol and error taxonomy**

- Lane: `research`; dependencies: T06.08.
- Requirements: R18, R22, R23, R24, R26.
- Allowed paths: `research/ANNOTATION_PROTOCOL.md`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: Gold labels not copied from LLM output. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/07_research/evidence.md`.

## [ ] **T07.02 — Define calibration, abstention and false-link evaluation**

- Lane: `research`; dependencies: T07.01.
- Requirements: R18, R22, R23, R24, R26.
- Allowed paths: `research/EVALUATION.md`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: Per-task PR/F1 and calibration recorded. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/07_research/evidence.md`.

## [ ] **T07.03 — Benchmark rules vs local NLP vs optional local LLM**

- Lane: `research`; dependencies: T07.02.
- Requirements: R18, R22, R23, R24, R26.
- Allowed paths: `research/RESULTS_TEMPLATE.md`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: Model/version/hardware/cost/privacy controlled. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/07_research/evidence.md`.

## [ ] **T07.04 — Consolidate qualified optional retrieval benchmark evidence**

- Lane: `research`; dependencies: T07.03, T13.06.
- Requirements: R18, R22, R23, R24, R26, R52, R53.
- Allowed paths: `research/GRAPH_EVAL.md`.
- Action: Summarize the executed five-arm evaluation specified in T13.06; do not create a duplicate graph implementation or infer results from planning packets.
- Acceptance: No advantage claimed without evaluation. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/07_research/evidence.md`.

## [ ] **T07.05 — Define optional temporal evidence graph contract**

- Lane: `research`; dependencies: T06.08.
- Requirements: R18, R22, R23, R24, R26.
- Allowed paths: `contracts/v1/graph-contract.md`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: Bitemporal, provenance, permissions, explicit alternatives. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/07_research/evidence.md`.

## [ ] **T07.06 — Qualify default-feature parser/exporter fuzz and bounded resource regressions**

- Lane: `core`; dependencies: T03.08.
- Requirements: R18, R22, R23, R24, R26.
- Allowed paths: `fuzz/README.md`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: No crash or unbounded allocation on samples. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/07_research/evidence.md`.

## [ ] **T07.07 — Audit default-feature supply chain; optional feature audits remain separate**

- Lane: `core`; dependencies: T00.06, T03.08.
- Requirements: R18, R22, R23, R24, R26.
- Allowed paths: `docs/SUPPLY_CHAIN.md`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: Only explicitly approved licences/features. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/07_research/evidence.md`.

## [ ] **T07.08 — Produce reproducibility bundle and limitations report**

- Lane: `research`; dependencies: T07.03, T07.04, T07.06, T07.07.
- Requirements: R18, R22, R23, R24, R26.
- Allowed paths: `research/REPRODUCIBILITY.md`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: Cite data and exact code commit. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/07_research/evidence.md`.
