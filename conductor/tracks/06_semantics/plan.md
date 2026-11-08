# Versioned NLP claims, entity resolution and evidence sidecar implementation plan

**Status:** all implementation tasks not started. **Goal:** the bounded specification in `spec.md`.
**Architecture:** retain independent Rust raw-email -> OCEL 2.0 core; optional lanes do not gate core.
**Tech stack:** qualified Rust dependencies; Python helpers only for development-document validation.
**Spec:** `spec.md`, `docs/CONTRACT.md`, `docs/FRONTIER-DESIGN.md`.

## Global constraints and review focus
No real email/source data in Git; no network/model fallback; no participant-role/authority conflation; preserve source attribution and OCEL attributes/qualifiers; observe exact task prerequisites, not whole-track gates. Test unknowns, malicious data, copied quotations, Unicode spans and missing/ambiguous references where applicable.

The manifest is authoritative for dependencies and lane. Checkboxes are completion records, not approval requests. Planning checks do not mark implementation complete.

- [ ] **T06.01 — Implement ExtractionClaim and EvidenceReference serde types**
  - Lane: `enrichment`. Depends on: T05b.07.
  - Edit only: `crates/email-ocel-semantics/src/contracts.rs`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: JSON schema validation and round trips. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/06_semantics/evidence.md`. Commit: `T06.01: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.

- [ ] **T06.02 — Add rule-only request/commitment/deadline baseline**
  - Lane: `enrichment`. Depends on: T06.01.
  - Edit only: `crates/email-ocel-semantics/src/rules.rs`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Synthetic gold fixture precision measured. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/06_semantics/evidence.md`. Commit: `T06.02: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.

- [ ] **T06.03 — Add explicit negation/modality/actor/tense detection tests**
  - Lane: `enrichment`. Depends on: T06.02.
  - Edit only: `crates/email-ocel-semantics/tests/semantics.rs`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: No approval claimed from negative example. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/06_semantics/evidence.md`. Commit: `T06.03: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.

- [ ] **T06.04 — Implement source span anchoring and protected locators**
  - Lane: `enrichment`. Depends on: T06.03.
  - Edit only: `crates/email-ocel-semantics/src/provenance.rs`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: No raw text in analytics logs. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/06_semantics/evidence.md`. Commit: `T06.04: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.

- [ ] **T06.05 — Implement optional local model provider trait**
  - Lane: `enrichment`. Depends on: T06.04.
  - Edit only: `crates/email-ocel-semantics/src/provider.rs`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: No default network/LLM dependency. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/06_semantics/evidence.md`. Commit: `T06.05: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.

- [ ] **T06.06 — Add schema-constrained extraction and abstention**
  - Lane: `enrichment`. Depends on: T06.05.
  - Edit only: `crates/email-ocel-semantics/src/extractor.rs`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Reject hallucinated IDs and absent sources. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/06_semantics/evidence.md`. Commit: `T06.06: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.

- [ ] **T06.07 — Add human adjudication/correction append-only records**
  - Lane: `enrichment`. Depends on: T06.06.
  - Edit only: `crates/email-ocel-semantics/src/review.rs`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Never mutate observed email facts. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/06_semantics/evidence.md`. Commit: `T06.07: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.

- [ ] **T06.08 — Project only accepted semantics to separate enriched OCEL**
  - Lane: `enrichment`. Depends on: T06.07.
  - Edit only: `crates/email-ocel-semantics/src/projection.rs`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Stage 1 output unchanged after rerun. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/06_semantics/evidence.md`. Commit: `T06.08: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.
