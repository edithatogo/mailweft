# Track 02_identity — reconciled evidence

Canonical acceptance declarations: `../../task-manifest.json`. Current CI must pass before merge.
Earlier checkpoints remain in Git history; initial NOT_STARTED assertions are not current status.

## T02.01 — completed

Domain-separated deterministic IDs have source-copy and event tests; raw identifiers are not emitted.

Artifacts: `crates/mailweft-core/src/lib.rs`, `crates/mailweft-core/tests/conversion.rs`
Verification: `cargo test --locked --workspace --all-targets --no-fail-fast`; `cargo fmt --all -- --check`; `python3 scripts/conductor_doctor.py --check`
Evidence: https://github.com/edithatogo/mailweft/actions/runs/42590576579; docs/VERIFICATION-0.7.md (final exact-head CI is the acceptance gate)
Remaining: Hashes are pseudonyms, not anonymity; keyed/scoped identity is a separate hardening step.

## T02.02 — completed

Distinct custodians retain source copies while exact-byte message candidates can be shared.

Artifacts: `crates/mailweft-core/src/batch.rs`, `crates/mailweft-core/tests/batch.rs`
Verification: `cargo test --locked --workspace --all-targets --no-fail-fast`; `cargo fmt --all -- --check`; `python3 scripts/conductor_doctor.py --check`
Evidence: https://github.com/edithatogo/mailweft/actions/runs/42590576579; docs/VERIFICATION-0.7.md (final exact-head CI is the acceptance gate)
Remaining: Near-duplicate and content-semantic identity resolution remain separate.

## T02.03 — completed

Missing/colliding Message-ID claims do not overwrite distinct raw messages or force ambiguous parent links.

Artifacts: `crates/mailweft-core/src/header_claims.rs`, `crates/mailweft-core/tests/thread_candidates.rs`
Verification: `cargo test --locked --workspace --all-targets --no-fail-fast`; `cargo fmt --all -- --check`; `python3 scripts/conductor_doctor.py --check`
Evidence: https://github.com/edithatogo/mailweft/actions/runs/42590576579; docs/VERIFICATION-0.7.md (final exact-head CI is the acceptance gate)
Remaining: Full legacy-address and broader threading inference are not included.

## T02.04 — in_progress

Observed participant qualifiers and independent-import checks exist.

Artifacts: `crates/mailweft-core/tests/rust4pm_interop.rs`, `crates/mailweft-core/src/lib.rs`
Verification: `cargo test --locked --workspace --all-targets --no-fail-fast`; `cargo fmt --all -- --check`; `python3 scripts/conductor_doctor.py --check`
Evidence: https://github.com/edithatogo/mailweft/actions/runs/42590576579; docs/VERIFICATION-0.7.md (final exact-head CI is the acceptance gate)
Remaining: Complete role-value roundtrip, Bcc disclosure controls and empty/unknown states need stronger qualification.

## T02.05 — in_progress

Unique observed In-Reply-To targets are labelled candidate edges; missing and ambiguous targets abstain.

Artifacts: `crates/mailweft-core/src/batch.rs`, `crates/mailweft-core/tests/thread_candidates.rs`
Verification: `cargo test --locked --workspace --all-targets --no-fail-fast`; `cargo fmt --all -- --check`; `python3 scripts/conductor_doctor.py --check`
Evidence: https://github.com/edithatogo/mailweft/actions/runs/42590576579; docs/VERIFICATION-0.7.md (final exact-head CI is the acceptance gate)
Remaining: Reference-chain reconstruction, cycles, incomplete sources and externally adjudicated accuracy remain.
