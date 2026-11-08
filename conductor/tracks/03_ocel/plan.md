# Canonical OCEL 2.0 projection and Rust4PM verification implementation plan

**Status:** all implementation tasks not started. **Goal:** the bounded specification in `spec.md`.
**Architecture:** retain independent Rust raw-email -> OCEL 2.0 core; optional lanes do not gate core.
**Tech stack:** qualified Rust dependencies; Python helpers only for development-document validation.
**Spec:** `spec.md`, `docs/CONTRACT.md`, `docs/FRONTIER-DESIGN.md`.

## Global constraints and review focus
No real email/source data in Git; no network/model fallback; no participant-role/authority conflation; preserve source attribution and OCEL attributes/qualifiers; observe exact task prerequisites, not whole-track gates. Test unknowns, malicious data, copied quotations, Unicode spans and missing/ambiguous references where applicable.

The manifest is authoritative for dependencies and lane. Checkboxes are completion records, not approval requests. Planning checks do not mark implementation complete.

- [ ] **T03.01 — Lock Rust4PM process_mining API and audit supported formats**
  - Lane: `core`. Depends on: T02.08.
  - Edit only: `docs/OCEL_COMPAT.md`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Exact version and capabilities documented. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/03_ocel/evidence.md`. Commit: `T03.01: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.

- [ ] **T03.02 — Define type and qualifier vocabulary in mapping contract**
  - Lane: `core`. Depends on: T03.01.
  - Edit only: `contracts/v1/ocel-mapping.md`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Typed E2O/O2O roles precise. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/03_ocel/evidence.md`. Commit: `T03.02: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.

- [ ] **T03.03 — Map Stage 1 message/source/participant objects and events**
  - Lane: `core`. Depends on: T03.02.
  - Edit only: `crates/email-ocel-core/src/ocel.rs`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: One source copy not falsely equal message delivered. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/03_ocel/evidence.md`. Commit: `T03.03: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.

- [ ] **T03.04 — Produce full OCEL 2.0 native JSON using Rust4PM**
  - Lane: `core`. Depends on: T03.03.
  - Edit only: `crates/email-ocel-cli/src/export.rs`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Independent import succeeds. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/03_ocel/evidence.md`. Commit: `T03.04: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.

- [ ] **T03.05 — Round-trip variable object attributes + typed values**
  - Lane: `core`. Depends on: T03.04.
  - Edit only: `crates/email-ocel-core/tests/ocel_roundtrip.rs`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: No dropped type/change history. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/03_ocel/evidence.md`. Commit: `T03.05: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.

- [ ] **T03.06 — Round-trip E2O/O2O qualifiers and multiplicity**
  - Lane: `core`. Depends on: T03.05.
  - Edit only: `crates/email-ocel-core/tests/ocel_relations.rs`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Equality of relation count and qualifiers. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/03_ocel/evidence.md`. Commit: `T03.06: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.

- [ ] **T03.07 — Run Rust4PM basic object-centric mining smoke test**
  - Lane: `core`. Depends on: T03.06.
  - Edit only: `crates/email-ocel-core/tests/mining.rs`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Algorithm runs against emitted fixture. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/03_ocel/evidence.md`. Commit: `T03.07: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.

- [ ] **T03.08 — Add loss ledger and format compatibility matrix**
  - Lane: `core`. Depends on: T03.07.
  - Edit only: `docs/OCEL_COMPAT.md`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Every unsupported field reported, not erased. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/03_ocel/evidence.md`. Commit: `T03.08: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.
