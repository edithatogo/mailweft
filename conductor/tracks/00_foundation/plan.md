# Foundation, legal/source gates and Rust workspace — executable task plan

Generated from the canonical task manifest; edit that manifest, then run `python3 scripts/conductor_doctor.py --sync`.
A partial task is not unimplemented and not complete. Evidence and remaining acceptance work are explicit.

- [x] Task: **T00.01 — Inventory dependency licences and record SPDX + exact pinned candidate versions**

  - Lane: `core`; dependencies: none.
  - Requirements: R10, R12, R13, R15.
  - Allowed paths: `docs/DEPENDENCIES.md`, `docs/dependency-inventory.json`.
  - Action: Acceptance recorded. Do not repeat implementation; protect it with the listed regressions.
  - Acceptance: Validate upstream source licences; no unlicensed code copied. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `cargo test --locked --workspace --all-targets --no-fail-fast; cargo fmt --all -- --check; python3 scripts/conductor_doctor.py --check`.
  - Evidence: `conductor/tracks/00_foundation/evidence.md`.
  - Implemented: Resolved direct dependencies and declared SPDX/source licence file hashes inventoried from locked Cargo metadata.
  - Remaining: Transitive vulnerabilities and wider candidate evaluation are separate tasks.

- [ ] Task: **T00.02 — Verify CMU, EDRM and Berti raw/reference rights and write acquisition policy**

  - Lane: `corpus`; dependencies: T00.01.
  - Requirements: R10, R12, R13, R15.
  - Allowed paths: `docs/DATASETS.md`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: Separate rights status and permitted uses, no download. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/00_foundation/evidence.md`.

- [x] Task: **T00.03 — Record repository boundaries and source asset exclusions**

  - Lane: `core`; dependencies: T00.01.
  - Requirements: R10, R12, R13, R15.
  - Allowed paths: `.gitignore`, `AGENTS.md`.
  - Action: Acceptance recorded. Do not repeat implementation; protect it with the listed regressions.
  - Acceptance: No real-data files trackable by default. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `cargo test --locked --workspace --all-targets --no-fail-fast; cargo fmt --all -- --check; python3 scripts/conductor_doctor.py --check`.
  - Evidence: `conductor/tracks/00_foundation/evidence.md`.
  - Implemented: Source/capture/FOI boundaries and private-data exclusions are recorded; representative ignore rules are checked.
  - Remaining: Ignore rules are not a data-loss prevention classifier; deliberately staged material still needs review.

- [x] Task: **T00.04 — Maintain the minimal mailweft-core and CLI Rust workspace**

  - Lane: `core`; dependencies: T00.01, T00.03.
  - Requirements: R10, R12, R13, R15.
  - Allowed paths: `Cargo.toml`, `crates/*`, `crates/mailweft-cli/Cargo.toml`, `crates/mailweft-core/Cargo.toml`.
  - Action: Acceptance recorded. Do not repeat implementation; protect it with the listed regressions.
  - Acceptance: Locked cargo metadata and workspace tests pass for the two existing generic Rust packages; no duplicate proposed crate.
  - Verification: `cargo test --locked --workspace --all-targets --no-fail-fast; cargo fmt --all -- --check; python3 scripts/conductor_doctor.py --check`.
  - Evidence: `conductor/tracks/00_foundation/evidence.md`.
  - Implemented: The two-package Rust workspace exists and builds; no duplicate proposed crate is required.
  - Remaining: Keep the existing mailweft-core/CLI names; do not recreate the original email-ocel scaffold.

- [x] Task: **T00.05 — Add rust-toolchain pin and deterministic dependency lock**

  - Lane: `core`; dependencies: T00.04.
  - Requirements: R10, R12, R13, R15.
  - Allowed paths: `Cargo.lock`, `rust-toolchain.toml`.
  - Action: Acceptance recorded. Do not repeat implementation; protect it with the listed regressions.
  - Acceptance: Locked build reproducible on chosen toolchain. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `cargo test --locked --workspace --all-targets --no-fail-fast; cargo fmt --all -- --check; python3 scripts/conductor_doctor.py --check`.
  - Evidence: `conductor/tracks/00_foundation/evidence.md`.
  - Implemented: Tested Rust 1.99.1 toolchain and dependency resolution are pinned.
  - Remaining: Future upgrades require an explicit qualification change.

- [~] Task: **T00.06 — Add CI synthetic lint/test/contract and dependency security scans**

  - Lane: `core`; dependencies: T00.05.
  - Requirements: R10, R12, R13, R15.
  - Allowed paths: `.github/workflows/ci.yml`, `.github/workflows/planning-checks.yml`, `.github/workflows/rust-core.yml`.
  - Action: Resume from the recorded implementation; implement only remaining acceptance, with a failing regression first.
  - Acceptance: CI requires no real mailbox/corpus data. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `cargo test --locked --workspace --all-targets --no-fail-fast; cargo fmt --all -- --check; python3 scripts/conductor_doctor.py --check`.
  - Evidence: `conductor/tracks/00_foundation/evidence.md`.
  - Implemented: Read-only synthetic Rust, contracts, planning and Conductor checks run in CI.
  - Remaining: Dedicated dependency advisory/security scanning and broader fuzz gates remain open.

- [x] Task: **T00.07 — Verify Conductor submodule and project context integrity**

  - Lane: `core`; dependencies: T00.04, T00.05.
  - Requirements: R10, R12, R13, R15.
  - Allowed paths: `GEMINI.md`, `docs/CONDUCTOR-SETUP.md`, `scripts/check_conductor.sh`, `scripts/conductor_doctor.py`.
  - Action: Acceptance recorded. Do not repeat implementation; protect it with the listed regressions.
  - Acceptance: Conductor doctor validates real pinned checkout, command TOMLs, all track artifacts and synchronized status; no user-host installation claim.
  - Verification: `cargo test --locked --workspace --all-targets --no-fail-fast; cargo fmt --all -- --check; python3 scripts/conductor_doctor.py --check`.
  - Evidence: `conductor/tracks/00_foundation/evidence.md`.
  - Implemented: Pinned gitlink, actual checkout, upstream command manifests and generated project/track context are checked.
  - Remaining: Host extension registration on the user computer is explicitly unverified, not a repository prerequisite.
