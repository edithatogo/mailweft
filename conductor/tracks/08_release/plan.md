# Interoperability, packaging and downstream consumer boundaries implementation plan

**Status:** all implementation tasks not started. **Goal:** the bounded specification in `spec.md`.
**Architecture:** retain independent Rust raw-email -> OCEL 2.0 core; optional lanes do not gate core.
**Tech stack:** qualified Rust dependencies; Python helpers only for development-document validation.
**Spec:** `spec.md`, `docs/CONTRACT.md`, `docs/FRONTIER-DESIGN.md`.

## Global constraints and review focus
No real email/source data in Git; no network/model fallback; no participant-role/authority conflation; preserve source attribution and OCEL attributes/qualifiers; observe exact task prerequisites, not whole-track gates. Test unknowns, malicious data, copied quotations, Unicode spans and missing/ambiguous references where applicable.

The manifest is authoritative for dependencies and lane. Checkboxes are completion records, not approval requests. Planning checks do not mark implementation complete.

- [ ] **T08.01 — Document stable library API and SemVer mapping**
  - Lane: `core`. Depends on: T03.08.
  - Edit only: `docs/API.md`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: No Enron-specific types in public API. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/08_release/evidence.md`. Commit: `T08.01: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.

- [ ] **T08.02 — Add optional local MBOX input adapter**
  - Lane: `integration`. Depends on: T08.01.
  - Edit only: `crates/email-ocel-cli/src/adapter_mbox.rs`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Streaming, byte-exact item provenance. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/08_release/evidence.md`. Commit: `T08.02: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.

- [ ] **T08.03 — Add native OCEL XML/SQLite adapters where supported**
  - Lane: `integration`. Depends on: T08.01.
  - Edit only: `crates/email-ocel-core/src/formats.rs`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Loss ledger and qualified parity tests. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/08_release/evidence.md`. Commit: `T08.03: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.

- [ ] **T08.04 — Create optional cross-engine reference validation**
  - Lane: `integration`. Depends on: T08.01.
  - Edit only: `tests/pm4py_optional.md`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Licensed isolated run, no Python runtime requirement. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/08_release/evidence.md`. Commit: `T08.04: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.

- [ ] **T08.05 — Document generic consumer interface with FOI as sole integration target**
  - Lane: `integration`. Depends on: T08.01.
  - Edit only: `docs/CONSUMERS.md`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: No downstream application dependency; foi-process is the only active owned consumer. Validate generic export and field-loss contracts; runtime checks remain blocked until implementation exists.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/08_release/evidence.md`. Commit: `T08.05: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.

- [ ] **T08.06 — Add macOS arm64, Linux x64, Windows x64 CI matrix**
  - Lane: `core`. Depends on: T08.01, T00.06.
  - Edit only: `.github/workflows/ci.yml`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: All synthetic stages pass or blocked explicitly. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/08_release/evidence.md`. Commit: `T08.06: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.

- [ ] **T08.07 — Package crate release with notices and docs**
  - Lane: `core`. Depends on: T08.01, T08.06, T07.06, T07.07, T04.01.
  - Edit only: `docs/RELEASE.md`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Package the core candidate only after synthetic parser/OCEL fidelity, offline resource/privacy checks and selected platform checks pass. Optional NLP, datasets and adapters do not gate it. Release publication itself needs the user-selected destination and authority.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/08_release/evidence.md`. Commit: `T08.07: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.

- [ ] **T08.08 — Write independent end-to-end acceptance summary**
  - Lane: `core`. Depends on: T08.07.
  - Edit only: `docs/ACCEPTANCE.md`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Record core-only independent synthetic end-to-end acceptance. State separately that all optional lanes and real-corpus benchmarks may remain unrun.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/08_release/evidence.md`. Commit: `T08.08: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.
