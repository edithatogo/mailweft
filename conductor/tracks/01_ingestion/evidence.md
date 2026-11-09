# Track 01_ingestion — reconciled evidence

Canonical acceptance declarations: `../../task-manifest.json`. Current CI must pass before merge.
Earlier checkpoints remain in Git history; initial NOT_STARTED assertions are not current status.

## T01.01 — completed

Versioned source envelopes, byte integrity receipts and counted input readers are implemented.

Artifacts: `crates/mailweft-core/src/source.rs`, `crates/mailweft-core/tests/source_integrity.rs`
Verification: `cargo test --locked --workspace --all-targets --no-fail-fast`; `cargo fmt --all -- --check`; `python3 scripts/conductor_doctor.py --check`
Evidence: https://github.com/edithatogo/mailweft/actions/runs/42590576579; docs/VERIFICATION-0.7.md (final exact-head CI is the acceptance gate)
Remaining: Rights/time/coverage remain caller assertions; hostile path races remain out of scope.

## T01.03 — completed

Pinned MIME parser is integrated with stable conversion/source diagnostics and negative fixtures.

Artifacts: `crates/mailweft-core/src/lib.rs`, `crates/mailweft-core/src/source.rs`
Verification: `cargo test --locked --workspace --all-targets --no-fail-fast`; `cargo fmt --all -- --check`; `python3 scripts/conductor_doctor.py --check`
Evidence: https://github.com/edithatogo/mailweft/actions/runs/42590576579; docs/VERIFICATION-0.7.md (final exact-head CI is the acceptance gate)
Remaining: This task does not certify every malformed MIME edge case or full parser fuzzing.

## T01.04 — in_progress

From/Sender/To/Cc/Bcc/Reply-To and claimed Date/message identifiers are projected.

Artifacts: `crates/mailweft-core/src/header_claims.rs`, `crates/mailweft-core/src/lib.rs`
Verification: `cargo test --locked --workspace --all-targets --no-fail-fast`; `cargo fmt --all -- --check`; `python3 scripts/conductor_doctor.py --check`
Evidence: https://github.com/edithatogo/mailweft/actions/runs/42590576579; docs/VERIFICATION-0.7.md (final exact-head CI is the acceptance gate)
Remaining: Resent/envelope evidence, unresolved groups, full address edge cases and comprehensive source spans remain.

## T01.05 — completed

Claimed header Date and caller observation time remain distinct; no fabricated delivery event.

Artifacts: `crates/mailweft-core/src/header_claims.rs`, `crates/mailweft-core/tests/header_claims.rs`
Verification: `cargo test --locked --workspace --all-targets --no-fail-fast`; `cargo fmt --all -- --check`; `python3 scripts/conductor_doctor.py --check`
Evidence: https://github.com/edithatogo/mailweft/actions/runs/42590576579; docs/VERIFICATION-0.7.md (final exact-head CI is the acceptance gate)
Remaining: Independent verified transport timestamps require separate source evidence.

## T01.06 — completed

Bounded source-derived attachment metadata is emitted without fetching/executing/logging payloads.

Artifacts: `crates/mailweft-core/src/attachments.rs`, `crates/mailweft-core/tests/attachment_metadata.rs`
Verification: `cargo test --locked --workspace --all-targets --no-fail-fast`; `cargo fmt --all -- --check`; `python3 scripts/conductor_doctor.py --check`
Evidence: https://github.com/edithatogo/mailweft/actions/runs/42590576579; docs/VERIFICATION-0.7.md (final exact-head CI is the acceptance gate)
Remaining: Content extraction, MIME adversarial corpus validation and content verification are separate work.

## T01.07 — completed

Each supplied slot has an accepted/rejected outcome, deterministic manifest digest and explicit partial/withheld projection.

Artifacts: `crates/mailweft-core/src/corpus.rs`, `crates/mailweft-cli/tests/corpus_reconciliation.rs`
Verification: `cargo test --locked --workspace --all-targets --no-fail-fast`; `cargo fmt --all -- --check`; `python3 scripts/conductor_doctor.py --check`
Evidence: https://github.com/edithatogo/mailweft/actions/runs/42590576579; docs/VERIFICATION-0.7.md (final exact-head CI is the acceptance gate)
Remaining: Completeness covers only the supplied selection; automatic discovery, streaming and archival attestation remain separate.

## T01.08 — completed

Offline CLI covers supplied single/batch emails, source verification and selection reconciliation with non-sensitive diagnostics.

Artifacts: `crates/mailweft-cli/src/main.rs`, `crates/mailweft-cli/src/reconcile.rs`
Verification: `cargo test --locked --workspace --all-targets --no-fail-fast`; `cargo fmt --all -- --check`; `python3 scripts/conductor_doctor.py --check`
Evidence: https://github.com/edithatogo/mailweft/actions/runs/42590576579; docs/VERIFICATION-0.7.md (final exact-head CI is the acceptance gate)
Remaining: The command is reconcile rather than the originally proposed ingest; no acquisition/recursion is implied.
