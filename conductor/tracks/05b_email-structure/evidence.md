# Track 05b_email-structure — reconciled evidence

Canonical acceptance declarations: `../../task-manifest.json`. Current CI must pass before merge.
Earlier checkpoints remain in Git history; initial NOT_STARTED assertions are not current status.

## T05b.01 — in_progress

Implementation-specific BodyEvidence separates content/origin and exact/derived provenance.

Artifacts: `crates/mailweft-core/src/zones.rs`, `contracts/v1/body-evidence.schema.json`
Verification: `cargo test --locked --workspace --all-targets --no-fail-fast`; `cargo fmt --all -- --check`; `python3 scripts/conductor_doctor.py --check`
Evidence: https://github.com/edithatogo/mailweft/actions/runs/42590576579; docs/VERIFICATION-0.7.md (final exact-head CI is the acceptance gate)
Remaining: The proposed nested email-zone interchange model is not fully implemented.

## T05b.02 — in_progress

Non-destructive deterministic quote/forward/signature/disclaimer baseline and diagnostics exist.

Artifacts: `crates/mailweft-core/src/zones.rs`, `crates/mailweft-core/tests/zoning_boundaries.rs`
Verification: `cargo test --locked --workspace --all-targets --no-fail-fast`; `cargo fmt --all -- --check`; `python3 scripts/conductor_doctor.py --check`
Evidence: https://github.com/edithatogo/mailweft/actions/runs/42590576579; docs/VERIFICATION-0.7.md (final exact-head CI is the acceptance gate)
Remaining: Nested attribution, multilingual coverage and independent classifier evaluation remain.

## T05b.03 — in_progress

Base64/QP transfer-unit maps are admitted only after parser-byte agreement; zone offsets remain conservative.

Artifacts: `crates/mailweft-core/src/transfer_map.rs`, `crates/mailweft-core/tests/transfer_provenance.rs`
Verification: `cargo test --locked --workspace --all-targets --no-fail-fast`; `cargo fmt --all -- --check`; `python3 scripts/conductor_doctor.py --check`
Evidence: https://github.com/edithatogo/mailweft/actions/runs/42590576579; docs/VERIFICATION-0.7.md (final exact-head CI is the acceptance gate)
Remaining: Charset transformation maps, HTML mappings and wider encoding fuzz qualification remain open.
