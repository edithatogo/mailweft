# Plan — 07: Quality, security, scaling and optional knowledge graph

**Execution rule:** One task per agent turn/commit. Complete tasks in numeric order. Update checkboxes only after acceptance and record actual commands/results in `evidence.md`.
**Initial state:** Not started. Any unavailable requirement/tool is BLOCKED, not a pass.

- [ ] **T07.01 — Design blinded adjudication protocol and error taxonomy**
  - **Edit only:** `research/ANNOTATION_PROTOCOL.md` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** Gold labels not copied from LLM output. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/07_research/evidence.md`, step committed as `T07.01: <short description>`.

- [ ] **T07.02 — Define calibration, abstention and false-link evaluation**
  - **Edit only:** `research/EVALUATION.md` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** Per-task PR/F1 and calibration recorded. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/07_research/evidence.md`, step committed as `T07.02: <short description>`.

- [ ] **T07.03 — Benchmark rules vs local NLP vs optional local LLM**
  - **Edit only:** `research/RESULTS_TEMPLATE.md` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** Model/version/hardware/cost/privacy controlled. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/07_research/evidence.md`, step committed as `T07.03: <short description>`.

- [ ] **T07.04 — Compare no-graph, graph-only, vector and GraphRAG methods**
  - **Edit only:** `research/GRAPH_EVAL.md` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** No advantage claimed without evaluation. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/07_research/evidence.md`, step committed as `T07.04: <short description>`.

- [ ] **T07.05 — Define optional temporal evidence graph contract**
  - **Edit only:** `contracts/v1/graph-contract.md` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** Bitemporal, provenance, permissions, explicit alternatives. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/07_research/evidence.md`, step committed as `T07.05: <short description>`.

- [ ] **T07.06 — Fuzz parser and OCEL mapper under size/resource constraints**
  - **Edit only:** `fuzz/README.md` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** No crash or unbounded allocation on samples. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/07_research/evidence.md`, step committed as `T07.06: <short description>`.

- [ ] **T07.07 — Run supply-chain and licence audit across feature sets**
  - **Edit only:** `docs/SUPPLY_CHAIN.md` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** Only explicitly approved licences/features. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/07_research/evidence.md`, step committed as `T07.07: <short description>`.

- [ ] **T07.08 — Produce reproducibility bundle and limitations report**
  - **Edit only:** `research/REPRODUCIBILITY.md` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** Cite data and exact code commit. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/07_research/evidence.md`, step committed as `T07.08: <short description>`.

## Phase completion gate
- [ ] All Txx.yy steps verified or documented explicitly deferred with justification.
- [ ] Run appropriate module integration tests, audit source/data leakage and record results in `evidence.md`.
- [ ] No claim of operational or externally validated acceptance without real receipts.
