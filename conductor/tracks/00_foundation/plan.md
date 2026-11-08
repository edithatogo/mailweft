# Foundation, legal/source gates and Rust workspace — executable task plan

Read the track spec, AGENTS.md and the task manifest. Each unchecked task is unimplemented. Dependencies are exact task IDs, not whole-track gates.

## [ ] **T00.01 — Inventory dependency licences and record SPDX + exact pinned candidate versions**

- Lane: `core`; dependencies: none.
- Requirements: R10, R12, R13, R15.
- Allowed paths: `docs/DEPENDENCIES.md`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: Validate upstream source licences; no unlicensed code copied. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/00_foundation/evidence.md`.

## [ ] **T00.02 — Verify CMU, EDRM and Berti raw/reference rights and write acquisition policy**

- Lane: `corpus`; dependencies: T00.01.
- Requirements: R10, R12, R13, R15.
- Allowed paths: `docs/DATASETS.md`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: Separate rights status and permitted uses, no download. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/00_foundation/evidence.md`.

## [ ] **T00.03 — Record repository boundaries and source asset exclusions**

- Lane: `core`; dependencies: T00.01.
- Requirements: R10, R12, R13, R15.
- Allowed paths: `AGENTS.md`, `.gitignore`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: No real-data files trackable by default. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/00_foundation/evidence.md`.

## [ ] **T00.04 — Create single minimal Rust workspace with email-ocel-core and CLI crates**

- Lane: `core`; dependencies: T00.01, T00.03.
- Requirements: R10, R12, R13, R15.
- Allowed paths: `Cargo.toml`, `crates/*`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: cargo metadata --no-deps succeeds; avoid unused crates. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED. Planned package root is crates/mailweft; no Enron or FOI-specific default is allowed.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/00_foundation/evidence.md`.

## [ ] **T00.05 — Add rust-toolchain pin and deterministic dependency lock**

- Lane: `core`; dependencies: T00.04.
- Requirements: R10, R12, R13, R15.
- Allowed paths: `rust-toolchain.toml`, `Cargo.lock`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: Locked build reproducible on chosen toolchain. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/00_foundation/evidence.md`.

## [ ] **T00.06 — Add CI synthetic lint/test/contract and dependency security scans**

- Lane: `core`; dependencies: T00.05.
- Requirements: R10, R12, R13, R15.
- Allowed paths: `.github/workflows/ci.yml`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: CI requires no real mailbox/corpus data. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/00_foundation/evidence.md`.

## [ ] **T00.07 — Verify Conductor submodule and project context integrity**

- Lane: `core`; dependencies: T00.06.
- Requirements: R10, R12, R13, R15.
- Allowed paths: `scripts/check_conductor.sh`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: gitlink/tag/sha and track files verified. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/00_foundation/evidence.md`.

## Native checkpoint

See ../../execution/20261108-core-slice.md. Only explicitly checked foundation tasks are complete. Remaining checked-in capability is partial; security issue #4 blocks qualification/release.
