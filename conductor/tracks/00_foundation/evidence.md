# Track 00_foundation — reconciled evidence

Canonical acceptance declarations: `../../task-manifest.json`. Current CI must pass before merge.
Earlier checkpoints remain in Git history; initial NOT_STARTED assertions are not current status.

## T00.01 — completed

Resolved direct dependencies and declared SPDX/source licence file hashes inventoried from locked Cargo metadata.

Artifacts: `docs/DEPENDENCIES.md`, `docs/dependency-inventory.json`
Verification: `cargo test --locked --workspace --all-targets --no-fail-fast`; `cargo fmt --all -- --check`; `python3 scripts/conductor_doctor.py --check`
Evidence: https://github.com/edithatogo/mailweft/actions/runs/42590576579; docs/VERIFICATION-0.7.md (final exact-head CI is the acceptance gate)
Remaining: Transitive vulnerabilities and wider candidate evaluation are separate tasks.

## T00.03 — completed

Source/capture/FOI boundaries and private-data exclusions are recorded; representative ignore rules are checked.

Artifacts: `AGENTS.md`, `.gitignore`
Verification: `cargo test --locked --workspace --all-targets --no-fail-fast`; `cargo fmt --all -- --check`; `python3 scripts/conductor_doctor.py --check`
Evidence: https://github.com/edithatogo/mailweft/actions/runs/42590576579; docs/VERIFICATION-0.7.md (final exact-head CI is the acceptance gate)
Remaining: Ignore rules are not a data-loss prevention classifier; deliberately staged material still needs review.

## T00.04 — completed

The two-package Rust workspace exists and builds; no duplicate proposed crate is required.

Artifacts: `Cargo.toml`, `crates/mailweft-core/Cargo.toml`, `crates/mailweft-cli/Cargo.toml`
Verification: `cargo test --locked --workspace --all-targets --no-fail-fast`; `cargo fmt --all -- --check`; `python3 scripts/conductor_doctor.py --check`
Evidence: https://github.com/edithatogo/mailweft/actions/runs/42590576579; docs/VERIFICATION-0.7.md (final exact-head CI is the acceptance gate)
Remaining: Keep the existing mailweft-core/CLI names; do not recreate the original email-ocel scaffold.

## T00.05 — completed

Tested Rust 1.99.1 toolchain and dependency resolution are pinned.

Artifacts: `rust-toolchain.toml`, `Cargo.lock`
Verification: `cargo test --locked --workspace --all-targets --no-fail-fast`; `cargo fmt --all -- --check`; `python3 scripts/conductor_doctor.py --check`
Evidence: https://github.com/edithatogo/mailweft/actions/runs/42590576579; docs/VERIFICATION-0.7.md (final exact-head CI is the acceptance gate)
Remaining: Future upgrades require an explicit qualification change.

## T00.06 — in_progress

Read-only synthetic Rust, contracts, planning and Conductor checks run in CI. User-approved synthetic boundary fuzzing adds shared native invariants, an isolated pinned libFuzzer target, and bounded PR/weekly execution.

Artifacts: `.github/workflows/rust-core.yml`, `.github/workflows/planning-checks.yml`, `.github/workflows/fuzz.yml`, `fuzz/README.md`, `fuzz/Cargo.toml`, `fuzz/Cargo.lock`, `fuzz/run.sh`, `fuzz/fuzz_targets/email_bytes.rs`, `crates/mailweft-core/tests/fuzz_regressions.rs`, `crates/mailweft-core/tests/support/fuzz_contract.rs`
Verification: `cargo test --locked --workspace --all-targets --no-fail-fast`; `cargo fmt --all -- --check`; `python3 scripts/conductor_doctor.py --check`; `cargo +1.99.1 test --locked -p mailweft-core --test fuzz_regressions`; `bash fuzz/run.sh 30`
Evidence: https://github.com/edithatogo/mailweft/actions/runs/42590576579; docs/VERIFICATION-0.7.md (final exact-head CI is the acceptance gate); fuzz/README.md (synthetic source policy, invariant definitions, exact budgets and reproduction); native fuzz_regressions: 2 passed, 0 failed on Rust 1.99.1; campaign/hosted receipts are recorded separately; fuzz/validation.json: native workspace tests/fmt/scoped Clippy and the local ASan campaign passed; local LeakSanitizer unavailable; final hosted CI pending.
Remaining: Dedicated dependency advisory/security scanning, hosted CI on the final fuzz commit and wider resource/release qualification remain open.

## T00.07 — completed

Pinned gitlink, actual checkout, upstream command manifests and generated project/track context are checked.

Artifacts: `scripts/conductor_doctor.py`, `docs/CONDUCTOR-SETUP.md`, `GEMINI.md`
Verification: `cargo test --locked --workspace --all-targets --no-fail-fast`; `cargo fmt --all -- --check`; `python3 scripts/conductor_doctor.py --check`
Evidence: https://github.com/edithatogo/mailweft/actions/runs/42590576579; docs/VERIFICATION-0.7.md (final exact-head CI is the acceptance gate)
Remaining: Host extension registration on the user computer is explicitly unverified, not a repository prerequisite.
