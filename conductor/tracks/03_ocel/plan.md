# Canonical OCEL 2.0 projection and Rust4PM verification — executable task plan

Generated from the canonical task manifest; edit that manifest, then run `python3 scripts/conductor_doctor.py --sync`.
A partial task is not unimplemented and not complete. Evidence and remaining acceptance work are explicit.

- [~] Task: **T03.01 — Lock Rust4PM process_mining API and audit supported formats**

  - Lane: `core`; dependencies: T02.08.
  - Requirements: R07, R08.
  - Allowed paths: `crates/mailweft-core/Cargo.toml`, `crates/mailweft-core/tests/rust4pm_interop.rs`, `docs/OCEL_COMPAT.md`.
  - Action: Resume from the recorded implementation; implement only remaining acceptance, with a failing regression first.
  - Acceptance: Exact version and capabilities documented. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `cargo test --locked --workspace --all-targets --no-fail-fast; cargo fmt --all -- --check; python3 scripts/conductor_doctor.py --check`.
  - Evidence: `conductor/tracks/03_ocel/evidence.md`.
  - Implemented: Rust4PM 0.6.2 is pinned and independently imports the emitted JSON fixtures.
  - Remaining: Full supported-format/API compatibility audit remains open.

- [ ] Task: **T03.02 — Define type and qualifier vocabulary in mapping contract**

  - Lane: `core`; dependencies: T03.01.
  - Requirements: R07, R08.
  - Allowed paths: `contracts/v1/ocel-mapping.md`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Typed E2O/O2O roles precise. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/03_ocel/evidence.md`.

- [~] Task: **T03.03 — Map Stage 1 message/source/participant objects and events**

  - Lane: `core`; dependencies: T03.02.
  - Requirements: R07, R08.
  - Allowed paths: `crates/mailweft-core/src/lib.rs`, `crates/mailweft-core/src/ocel.rs`.
  - Action: Resume from the recorded implementation; implement only remaining acceptance, with a failing regression first.
  - Acceptance: One source copy not falsely equal message delivered. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `cargo test --locked --workspace --all-targets --no-fail-fast; cargo fmt --all -- --check; python3 scripts/conductor_doctor.py --check`.
  - Evidence: `conductor/tracks/03_ocel/evidence.md`.
  - Implemented: Stage 1 source-copy, message, address and attachment objects/events are projected.
  - Remaining: The wider mapping contract and complete typed-value parity are not yet accepted.

- [~] Task: **T03.04 — Produce full OCEL 2.0 native JSON using Rust4PM**

  - Lane: `core`; dependencies: T03.03.
  - Requirements: R07, R08.
  - Allowed paths: `crates/mailweft-cli/src/export.rs`, `crates/mailweft-core/src/lib.rs`, `crates/mailweft-core/tests/rust4pm_interop.rs`.
  - Action: Resume from the recorded implementation; implement only remaining acceptance, with a failing regression first.
  - Acceptance: Independent import succeeds. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `cargo test --locked --workspace --all-targets --no-fail-fast; cargo fmt --all -- --check; python3 scripts/conductor_doctor.py --check`.
  - Evidence: `conductor/tracks/03_ocel/evidence.md`.
  - Implemented: Portable JSON projection imports through Rust4PM without a mandatory Rust4PM runtime dependency.
  - Remaining: Full OCEL typed-value/export parity and the canonical-representation decision require resolution.

- [~] Task: **T03.05 — Round-trip variable object attributes + typed values**

  - Lane: `core`; dependencies: T03.04.
  - Requirements: R07, R08.
  - Allowed paths: `crates/mailweft-core/src/batch.rs`, `crates/mailweft-core/tests/attachment_metadata.rs`, `crates/mailweft-core/tests/ocel_roundtrip.rs`.
  - Action: Resume from the recorded implementation; implement only remaining acceptance, with a failing regression first.
  - Acceptance: No dropped type/change history. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `cargo test --locked --workspace --all-targets --no-fail-fast; cargo fmt --all -- --check; python3 scripts/conductor_doctor.py --check`.
  - Evidence: `conductor/tracks/03_ocel/evidence.md`.
  - Implemented: Time-qualified attachment attribute histories are merged and imported.
  - Remaining: Current attributes are strings; heterogeneous primitive values and full roundtrips remain.

- [~] Task: **T03.06 — Round-trip E2O/O2O qualifiers and multiplicity**

  - Lane: `core`; dependencies: T03.05.
  - Requirements: R07, R08.
  - Allowed paths: `crates/mailweft-core/tests/ocel_relations.rs`, `crates/mailweft-core/tests/rust4pm_interop.rs`.
  - Action: Resume from the recorded implementation; implement only remaining acceptance, with a failing regression first.
  - Acceptance: Equality of relation count and qualifiers. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `cargo test --locked --workspace --all-targets --no-fail-fast; cargo fmt --all -- --check; python3 scripts/conductor_doctor.py --check`.
  - Evidence: `conductor/tracks/03_ocel/evidence.md`.
  - Implemented: Observed and candidate E2O/O2O relationships have independent-import regression tests.
  - Remaining: Complete value/qualifier/multiplicity roundtrip equality remains to be tested.

- [ ] Task: **T03.07 — Run Rust4PM basic object-centric mining smoke test**

  - Lane: `core`; dependencies: T03.06.
  - Requirements: R07, R08.
  - Allowed paths: `crates/mailweft-core/tests/mining.rs`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Algorithm runs against emitted fixture. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/03_ocel/evidence.md`.

- [ ] Task: **T03.08 — Add loss ledger and format compatibility matrix**

  - Lane: `core`; dependencies: T03.07.
  - Requirements: R07, R08, R42.
  - Allowed paths: `docs/OCEL_COMPAT.md`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Every unsupported field reported, not erased. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/03_ocel/evidence.md`.
