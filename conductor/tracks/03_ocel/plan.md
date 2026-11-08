# Canonical OCEL 2.0 projection and Rust4PM verification — executable task plan

Read the track spec, AGENTS.md and the task manifest. Each unchecked task is unimplemented. Dependencies are exact task IDs, not whole-track gates.

## [ ] **T03.01 — Lock Rust4PM process_mining API and audit supported formats**

- Lane: `core`; dependencies: T02.08.
- Requirements: R07, R08.
- Allowed paths: `docs/OCEL_COMPAT.md`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: Exact version and capabilities documented. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/03_ocel/evidence.md`.

## [ ] **T03.02 — Define type and qualifier vocabulary in mapping contract**

- Lane: `core`; dependencies: T03.01.
- Requirements: R07, R08.
- Allowed paths: `contracts/v1/ocel-mapping.md`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: Typed E2O/O2O roles precise. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/03_ocel/evidence.md`.

## [ ] **T03.03 — Map Stage 1 message/source/participant objects and events**

- Lane: `core`; dependencies: T03.02.
- Requirements: R07, R08.
- Allowed paths: `crates/mailweft-core/src/ocel.rs`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: One source copy not falsely equal message delivered. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/03_ocel/evidence.md`.

## [ ] **T03.04 — Produce full OCEL 2.0 native JSON using Rust4PM**

- Lane: `core`; dependencies: T03.03.
- Requirements: R07, R08.
- Allowed paths: `crates/mailweft-cli/src/export.rs`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: Independent import succeeds. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/03_ocel/evidence.md`.

## [ ] **T03.05 — Round-trip variable object attributes + typed values**

- Lane: `core`; dependencies: T03.04.
- Requirements: R07, R08.
- Allowed paths: `crates/mailweft-core/tests/ocel_roundtrip.rs`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: No dropped type/change history. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/03_ocel/evidence.md`.

## [ ] **T03.06 — Round-trip E2O/O2O qualifiers and multiplicity**

- Lane: `core`; dependencies: T03.05.
- Requirements: R07, R08.
- Allowed paths: `crates/mailweft-core/tests/ocel_relations.rs`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: Equality of relation count and qualifiers. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/03_ocel/evidence.md`.

## [ ] **T03.07 — Run Rust4PM basic object-centric mining smoke test**

- Lane: `core`; dependencies: T03.06.
- Requirements: R07, R08.
- Allowed paths: `crates/mailweft-core/tests/mining.rs`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: Algorithm runs against emitted fixture. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/03_ocel/evidence.md`.

## [ ] **T03.08 — Add loss ledger and format compatibility matrix**

- Lane: `core`; dependencies: T03.07.
- Requirements: R07, R08, R42.
- Allowed paths: `docs/OCEL_COMPAT.md`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: Every unsupported field reported, not erased. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/03_ocel/evidence.md`.
