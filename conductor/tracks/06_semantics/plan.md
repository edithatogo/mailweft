# Versioned NLP claims, entity resolution and evidence sidecar — executable task plan

Generated from the canonical task manifest; edit that manifest, then run `python3 scripts/conductor_doctor.py --sync`.
A partial task is not unimplemented and not complete. Evidence and remaining acceptance work are explicit.

- [ ] Task: **T06.01 — Implement ExtractionClaim and EvidenceReference serde types**

  - Lane: `enrichment`; dependencies: T05b.07.
  - Requirements: R20, R21, R22.
  - Allowed paths: `crates/mailweft-semantics/src/contracts.rs`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: JSON schema validation and round trips. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/06_semantics/evidence.md`.

- [ ] Task: **T06.02 — Add rule-only request/commitment/deadline baseline**

  - Lane: `enrichment`; dependencies: T06.01.
  - Requirements: R20, R21, R22.
  - Allowed paths: `crates/mailweft-semantics/src/rules.rs`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Synthetic gold fixture precision measured. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/06_semantics/evidence.md`.

- [ ] Task: **T06.03 — Add explicit negation/modality/actor/tense detection tests**

  - Lane: `enrichment`; dependencies: T06.02.
  - Requirements: R20, R21, R22.
  - Allowed paths: `crates/mailweft-semantics/tests/semantics.rs`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: No approval claimed from negative example. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/06_semantics/evidence.md`.

- [ ] Task: **T06.04 — Implement source span anchoring and protected locators**

  - Lane: `enrichment`; dependencies: T06.03.
  - Requirements: R20, R21, R22.
  - Allowed paths: `crates/mailweft-semantics/src/provenance.rs`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: No raw text in analytics logs. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/06_semantics/evidence.md`.

- [ ] Task: **T06.05 — Implement optional local model provider trait**

  - Lane: `enrichment`; dependencies: T06.04.
  - Requirements: R20, R21, R22.
  - Allowed paths: `crates/mailweft-semantics/src/provider.rs`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: No default network/LLM dependency. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/06_semantics/evidence.md`.

- [ ] Task: **T06.06 — Add schema-constrained extraction and abstention**

  - Lane: `enrichment`; dependencies: T06.05.
  - Requirements: R20, R21, R22.
  - Allowed paths: `crates/mailweft-semantics/src/extractor.rs`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Reject hallucinated IDs and absent sources. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/06_semantics/evidence.md`.

- [ ] Task: **T06.07 — Add human adjudication/correction append-only records**

  - Lane: `enrichment`; dependencies: T06.06.
  - Requirements: R20, R21, R22.
  - Allowed paths: `crates/mailweft-semantics/src/review.rs`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Never mutate observed email facts. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/06_semantics/evidence.md`.

- [ ] Task: **T06.08 — Project only accepted semantics to separate enriched OCEL**

  - Lane: `enrichment`; dependencies: T06.07.
  - Requirements: R20, R21, R22.
  - Allowed paths: `crates/mailweft-semantics/src/projection.rs`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Stage 1 output unchanged after rerun. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/06_semantics/evidence.md`.
