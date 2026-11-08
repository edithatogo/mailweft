# Synthetic and Enron raw-to-processed validation implementation plan

**Status:** all implementation tasks not started. **Goal:** the bounded specification in `spec.md`.
**Architecture:** retain independent Rust raw-email -> OCEL 2.0 core; optional lanes do not gate core.
**Tech stack:** qualified Rust dependencies; Python helpers only for development-document validation.
**Spec:** `spec.md`, `docs/CONTRACT.md`, `docs/FRONTIER-DESIGN.md`.

## Global constraints and review focus
No real email/source data in Git; no network/model fallback; no participant-role/authority conflation; preserve source attribution and OCEL attributes/qualifiers; observe exact task prerequisites, not whole-track gates. Test unknowns, malicious data, copied quotations, Unicode spans and missing/ambiguous references where applicable.

The manifest is authoritative for dependencies and lane. Checkboxes are completion records, not approval requests. Planning checks do not mark implementation complete.

- [ ] **T04.01 — Invent version-controlled synthetic MIME corpus generator**
  - Lane: `core`. Depends on: T00.05.
  - Edit only: `tools/generate_fixture.rs`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: No real addresses, reproducible deterministic files. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/04_data-validation/evidence.md`. Commit: `T04.01: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.

- [ ] **T04.02 — Write rights-gated raw corpus manifest template**
  - Lane: `corpus`. Depends on: T00.02, T04.01.
  - Edit only: `datasets/README.md`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Requires real source/version/rights and SHA256. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/04_data-validation/evidence.md`. Commit: `T04.02: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.

- [ ] **T04.03 — Build optional CMU raw maildir reader adapter**
  - Lane: `corpus`. Depends on: T04.02.
  - Edit only: `crates/email-ocel-cli/src/adapter_cmu.rs`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Local source, no automatic download. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/04_data-validation/evidence.md`. Commit: `T04.03: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.

- [ ] **T04.04 — Build optional EDRM EML/XML metadata reconciler**
  - Lane: `corpus`. Depends on: T04.03.
  - Edit only: `crates/email-ocel-cli/src/adapter_edrm.rs`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Correlate source hashes and attachment metadata. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/04_data-validation/evidence.md`. Commit: `T04.04: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.

- [ ] **T04.05 — Build Berti OCEL reference reader with record scope checks**
  - Lane: `corpus`. Depends on: T04.04.
  - Edit only: `crates/email-ocel-cli/src/adapter_reference.rs`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: No unsupported one-to-one claims. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/04_data-validation/evidence.md`. Commit: `T04.05: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.

- [ ] **T04.06 — Implement candidate cross-corpus record linkage**
  - Lane: `corpus`. Depends on: T04.05.
  - Edit only: `crates/email-ocel-core/src/matching.rs`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: exact/likely/ambiguous/unmatched classes. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/04_data-validation/evidence.md`. Commit: `T04.06: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.

- [ ] **T04.07 — Generate paired/unpaired validation reports**
  - Lane: `corpus`. Depends on: T04.06.
  - Edit only: `crates/email-ocel-cli/src/validation.rs`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Denominators and exclusions explicitly stated. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/04_data-validation/evidence.md`. Commit: `T04.07: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.

- [ ] **T04.08 — Run 10k/100k/full corpus benchmarks only after gate**
  - Lane: `corpus`. Depends on: T04.07.
  - Edit only: `benchmarks/README.md`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Hardware/RSS/time/source SHA and exclusions. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/04_data-validation/evidence.md`. Commit: `T04.08: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.
