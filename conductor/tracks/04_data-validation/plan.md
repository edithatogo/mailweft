# Synthetic and Enron raw-to-processed validation — executable task plan

Generated from the canonical task manifest; edit that manifest, then run `python3 scripts/conductor_doctor.py --sync`.
A partial task is not unimplemented and not complete. Evidence and remaining acceptance work are explicit.

- [ ] Task: **T04.01 — Invent version-controlled synthetic MIME corpus generator**

  - Lane: `core`; dependencies: T00.05.
  - Requirements: R11, R13, R14, R17, R24, R48.
  - Allowed paths: `tools/generate_fixture.rs`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: No real addresses, reproducible deterministic files. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/04_data-validation/evidence.md`.

- [ ] Task: **T04.02 — Write rights-gated raw corpus manifest template**

  - Lane: `corpus`; dependencies: T00.02, T04.01.
  - Requirements: R11, R13, R14, R24, R17.
  - Allowed paths: `datasets/README.md`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Requires real source/version/rights and SHA256. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/04_data-validation/evidence.md`.

- [ ] Task: **T04.03 — Add canonical Enron example recipe over the generic raw-mail reader**

  - Lane: `corpus`; dependencies: T04.02.
  - Requirements: R11, R13, R14, R17, R24, R48.
  - Allowed paths: `crates/mailweft-cli/src/adapter_cmu.rs`.
  - Action: Use the existing generic raw reader; keep CMU/Enron paths and metadata in optional benchmark recipes, never core branches or defaults.
  - Acceptance: Local source, no automatic download. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/04_data-validation/evidence.md`.

- [ ] Task: **T04.04 — Build optional EDRM EML/XML metadata reconciler**

  - Lane: `corpus`; dependencies: T04.03.
  - Requirements: R11, R13, R14, R24, R17.
  - Allowed paths: `crates/mailweft-cli/src/adapter_edrm.rs`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Correlate source hashes and attachment metadata. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/04_data-validation/evidence.md`.

- [ ] Task: **T04.05 — Build Berti OCEL reference reader with record scope checks**

  - Lane: `corpus`; dependencies: T04.04.
  - Requirements: R11, R13, R14, R24, R17.
  - Allowed paths: `crates/mailweft-cli/src/adapter_reference.rs`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: No unsupported one-to-one claims. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/04_data-validation/evidence.md`.

- [ ] Task: **T04.06 — Implement candidate cross-corpus record linkage**

  - Lane: `corpus`; dependencies: T04.05.
  - Requirements: R11, R13, R14, R24, R17.
  - Allowed paths: `crates/mailweft-core/src/matching.rs`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: exact/likely/ambiguous/unmatched classes. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/04_data-validation/evidence.md`.

- [ ] Task: **T04.07 — Generate paired/unpaired validation reports**

  - Lane: `corpus`; dependencies: T04.06.
  - Requirements: R11, R13, R14, R24, R17.
  - Allowed paths: `crates/mailweft-cli/src/validation.rs`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Denominators and exclusions explicitly stated. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/04_data-validation/evidence.md`.

- [ ] Task: **T04.08 — Run 10k/100k/full corpus benchmarks only after gate**

  - Lane: `corpus`; dependencies: T04.07.
  - Requirements: R11, R13, R14, R24, R17.
  - Allowed paths: `benchmarks/README.md`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Hardware/RSS/time/source SHA and exclusions. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/04_data-validation/evidence.md`.
