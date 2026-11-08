# Plan — 06: Versioned NLP claims, entity resolution and evidence sidecar

**Execution rule:** One task per agent turn/commit. Complete tasks in numeric order. Update checkboxes only after acceptance and record actual commands/results in `evidence.md`.
**Initial state:** Not started. Any unavailable requirement/tool is BLOCKED, not a pass.

- [ ] **T06.01 — Implement ExtractionClaim and EvidenceReference serde types**
  - **Edit only:** `crates/email-ocel-semantics/src/contracts.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** JSON schema validation and round trips. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/06_semantics/evidence.md`, step committed as `T06.01: <short description>`.

- [ ] **T06.02 — Add rule-only request/commitment/deadline baseline**
  - **Edit only:** `crates/email-ocel-semantics/src/rules.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** Synthetic gold fixture precision measured. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/06_semantics/evidence.md`, step committed as `T06.02: <short description>`.

- [ ] **T06.03 — Add explicit negation/modality/actor/tense detection tests**
  - **Edit only:** `crates/email-ocel-semantics/tests/semantics.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** No approval claimed from negative example. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/06_semantics/evidence.md`, step committed as `T06.03: <short description>`.

- [ ] **T06.04 — Implement source span anchoring and protected locators**
  - **Edit only:** `crates/email-ocel-semantics/src/provenance.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** No raw text in analytics logs. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/06_semantics/evidence.md`, step committed as `T06.04: <short description>`.

- [ ] **T06.05 — Implement optional local model provider trait**
  - **Edit only:** `crates/email-ocel-semantics/src/provider.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** No default network/LLM dependency. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/06_semantics/evidence.md`, step committed as `T06.05: <short description>`.

- [ ] **T06.06 — Add schema-constrained extraction and abstention**
  - **Edit only:** `crates/email-ocel-semantics/src/extractor.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** Reject hallucinated IDs and absent sources. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/06_semantics/evidence.md`, step committed as `T06.06: <short description>`.

- [ ] **T06.07 — Add human adjudication/correction append-only records**
  - **Edit only:** `crates/email-ocel-semantics/src/review.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** Never mutate observed email facts. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/06_semantics/evidence.md`, step committed as `T06.07: <short description>`.

- [ ] **T06.08 — Project only accepted semantics to separate enriched OCEL**
  - **Edit only:** `crates/email-ocel-semantics/src/projection.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** Stage 1 output unchanged after rerun. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/06_semantics/evidence.md`, step committed as `T06.08: <short description>`.

## Phase completion gate
- [ ] All Txx.yy steps verified or documented explicitly deferred with justification.
- [ ] Run appropriate module integration tests, audit source/data leakage and record results in `evidence.md`.
- [ ] No claim of operational or externally validated acceptance without real receipts.
