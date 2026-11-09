# Track 03_ocel — reconciled evidence

Canonical acceptance declarations: `../../task-manifest.json`. Current CI must pass before merge.
Earlier checkpoints remain in Git history; initial NOT_STARTED assertions are not current status.

## T03.01 — in_progress

Rust4PM 0.6.2 is pinned and independently imports the emitted JSON fixtures.

Artifacts: `crates/mailweft-core/Cargo.toml`, `crates/mailweft-core/tests/rust4pm_interop.rs`
Verification: `cargo test --locked --workspace --all-targets --no-fail-fast`; `cargo fmt --all -- --check`; `python3 scripts/conductor_doctor.py --check`
Evidence: https://github.com/edithatogo/mailweft/actions/runs/42590576579; docs/VERIFICATION-0.7.md (final exact-head CI is the acceptance gate)
Remaining: Full supported-format/API compatibility audit remains open.

## T03.03 — in_progress

Stage 1 source-copy, message, address and attachment objects/events are projected.

Artifacts: `crates/mailweft-core/src/lib.rs`
Verification: `cargo test --locked --workspace --all-targets --no-fail-fast`; `cargo fmt --all -- --check`; `python3 scripts/conductor_doctor.py --check`
Evidence: https://github.com/edithatogo/mailweft/actions/runs/42590576579; docs/VERIFICATION-0.7.md (final exact-head CI is the acceptance gate)
Remaining: The wider mapping contract and complete typed-value parity are not yet accepted.

## T03.04 — in_progress

Portable JSON projection imports through Rust4PM without a mandatory Rust4PM runtime dependency.

Artifacts: `crates/mailweft-core/src/lib.rs`, `crates/mailweft-core/tests/rust4pm_interop.rs`
Verification: `cargo test --locked --workspace --all-targets --no-fail-fast`; `cargo fmt --all -- --check`; `python3 scripts/conductor_doctor.py --check`
Evidence: https://github.com/edithatogo/mailweft/actions/runs/42590576579; docs/VERIFICATION-0.7.md (final exact-head CI is the acceptance gate)
Remaining: Full OCEL typed-value/export parity and the canonical-representation decision require resolution.

## T03.05 — in_progress

Time-qualified attachment attribute histories are merged and imported.

Artifacts: `crates/mailweft-core/src/batch.rs`, `crates/mailweft-core/tests/attachment_metadata.rs`
Verification: `cargo test --locked --workspace --all-targets --no-fail-fast`; `cargo fmt --all -- --check`; `python3 scripts/conductor_doctor.py --check`
Evidence: https://github.com/edithatogo/mailweft/actions/runs/42590576579; docs/VERIFICATION-0.7.md (final exact-head CI is the acceptance gate)
Remaining: Current attributes are strings; heterogeneous primitive values and full roundtrips remain.

## T03.06 — in_progress

Observed and candidate E2O/O2O relationships have independent-import regression tests.

Artifacts: `crates/mailweft-core/tests/rust4pm_interop.rs`
Verification: `cargo test --locked --workspace --all-targets --no-fail-fast`; `cargo fmt --all -- --check`; `python3 scripts/conductor_doctor.py --check`
Evidence: https://github.com/edithatogo/mailweft/actions/runs/42590576579; docs/VERIFICATION-0.7.md (final exact-head CI is the acceptance gate)
Remaining: Complete value/qualifier/multiplicity roundtrip equality remains to be tested.
