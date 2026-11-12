# Interoperability, packaging and downstream consumer boundaries — executable task plan

Generated from the canonical task manifest; edit that manifest, then run `python3 scripts/conductor_doctor.py --sync`.
A partial task is not unimplemented and not complete. Evidence and remaining acceptance work are explicit.

- [ ] Task: **T08.01 — Document stable library API and SemVer mapping**

  - Lane: `core`; dependencies: T03.08.
  - Requirements: R12, R16, R25.
  - Allowed paths: `docs/API.md`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: No Enron-specific types in public API. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/08_release/evidence.md`.

- [ ] Task: **T08.02 — Add optional local MBOX input adapter**

  - Lane: `integration`; dependencies: T08.01.
  - Requirements: R12, R16, R25.
  - Allowed paths: `crates/mailweft-cli/src/adapter_mbox.rs`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Streaming, byte-exact item provenance. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/08_release/evidence.md`.

- [ ] Task: **T08.03 — Add native OCEL XML/SQLite adapters where supported**

  - Lane: `integration`; dependencies: T08.01.
  - Requirements: R12, R16, R25.
  - Allowed paths: `crates/mailweft-core/src/formats.rs`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Loss ledger and qualified parity tests. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/08_release/evidence.md`.

- [ ] Task: **T08.04 — Create optional cross-engine reference validation**

  - Lane: `integration`; dependencies: T08.01.
  - Requirements: R12, R16, R25.
  - Allowed paths: `tests/pm4py_optional.md`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Licensed isolated run, no Python runtime requirement. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/08_release/evidence.md`.

- [ ] Task: **T08.05 — Document generic consumer interface with FOI as sole integration target**

  - Lane: `integration`; dependencies: T08.01.
  - Requirements: R12, R16, R25.
  - Allowed paths: `docs/CONSUMERS.md`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: No downstream application dependency; foi-process is the only active owned consumer. Validate the generic export contract and explicit field-loss report. Runtime checks remain blocked until implementation exists.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/08_release/evidence.md`.

- [ ] Task: **T08.06 — Add macOS arm64, Linux x64, Windows x64 CI matrix**

  - Lane: `core`; dependencies: T08.01, T00.06.
  - Requirements: R12, R16, R25.
  - Allowed paths: `.github/workflows/ci.yml`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: All synthetic stages pass or blocked explicitly. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/08_release/evidence.md`.

- [ ] Task: **T08.07 — Package crate release with notices and docs**

  - Lane: `core`; dependencies: T08.01, T08.06, T07.06, T07.07, T04.01.
  - Requirements: R12, R16, R25, R32, R47.
  - Allowed paths: `docs/RELEASE.md`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Package the core candidate only after synthetic parser/OCEL fidelity, offline resource/privacy checks and selected platform checks pass. Optional NLP, datasets and adapters do not gate it. Release publication itself needs the user-selected destination and authority.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/08_release/evidence.md`.

- [ ] Task: **T08.08 — Write independent end-to-end acceptance summary**

  - Lane: `core`; dependencies: T08.07.
  - Requirements: R12, R16, R25, R48.
  - Allowed paths: `docs/ACCEPTANCE.md`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Record core-only independent synthetic end-to-end acceptance. State separately that all optional lanes and real-corpus benchmarks may remain unrun.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/08_release/evidence.md`.
