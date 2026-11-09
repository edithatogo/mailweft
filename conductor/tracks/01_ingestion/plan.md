# Raw EML and maildir input, manifests and parse boundaries — executable task plan

Generated from the canonical task manifest; edit that manifest, then run `python3 scripts/conductor_doctor.py --sync`.
A partial task is not unimplemented and not complete. Evidence and remaining acceptance work are explicit.

- [x] Task: **T01.01 — Implement source envelope types and bounded file reader**

  - Lane: `core`; dependencies: T00.05, T00.03.
  - Requirements: R01, R02, R03, R09, R11.
  - Allowed paths: `crates/mailweft-core/src/source.rs`, `crates/mailweft-core/tests/source_integrity.rs`.
  - Action: Acceptance recorded. Do not repeat implementation; protect it with the listed regressions.
  - Acceptance: Hash byte-exact source content; source only local. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `cargo test --locked --workspace --all-targets --no-fail-fast; cargo fmt --all -- --check; python3 scripts/conductor_doctor.py --check`.
  - Evidence: `conductor/tracks/01_ingestion/evidence.md`.
  - Implemented: Versioned source envelopes, byte integrity receipts and counted input readers are implemented.
  - Remaining: Rights/time/coverage remain caller assertions; hostile path races remain out of scope.

- [ ] Task: **T01.02 — Implement maildir traversal and ordered EML discovery**

  - Lane: `core`; dependencies: T01.01.
  - Requirements: R01, R02, R03, R09, R11.
  - Allowed paths: `crates/mailweft-cli/src/walk.rs`.
  - Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
  - Acceptance: No traversal outside root; reproducible ordering. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
  - Evidence: `conductor/tracks/01_ingestion/evidence.md`.

- [x] Task: **T01.03 — Integrate mail-parser with error map**

  - Lane: `core`; dependencies: T01.01.
  - Requirements: R01, R02, R03, R09, R11, R48.
  - Allowed paths: `crates/mailweft-core/src/lib.rs`, `crates/mailweft-core/src/parser.rs`, `crates/mailweft-core/src/source.rs`.
  - Action: Acceptance recorded. Do not repeat implementation; protect it with the listed regressions.
  - Acceptance: Malformed samples reject with stable codes. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `cargo test --locked --workspace --all-targets --no-fail-fast; cargo fmt --all -- --check; python3 scripts/conductor_doctor.py --check`.
  - Evidence: `conductor/tracks/01_ingestion/evidence.md`.
  - Implemented: Pinned MIME parser is integrated with stable conversion/source diagnostics and negative fixtures.
  - Remaining: This task does not certify every malformed MIME edge case or full parser fuzzing.

- [~] Task: **T01.04 — Capture all essential message header and address fields**

  - Lane: `core`; dependencies: T01.03.
  - Requirements: R01, R02, R03, R09, R11.
  - Allowed paths: `crates/mailweft-core/src/header_claims.rs`, `crates/mailweft-core/src/lib.rs`, `crates/mailweft-core/src/message.rs`.
  - Action: Resume from the recorded implementation; implement only remaining acceptance, with a failing regression first.
  - Acceptance: Synthetic multipart EML matches expected structure. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `cargo test --locked --workspace --all-targets --no-fail-fast; cargo fmt --all -- --check; python3 scripts/conductor_doctor.py --check`.
  - Evidence: `conductor/tracks/01_ingestion/evidence.md`.
  - Implemented: From/Sender/To/Cc/Bcc/Reply-To and claimed Date/message identifiers are projected.
  - Remaining: Resent/envelope evidence, unresolved groups, full address edge cases and comprehensive source spans remain.

- [x] Task: **T01.05 — Separate header time from source observation and source receipt**

  - Lane: `core`; dependencies: T01.03.
  - Requirements: R01, R02, R03, R09, R11.
  - Allowed paths: `crates/mailweft-core/src/header_claims.rs`, `crates/mailweft-core/src/time.rs`, `crates/mailweft-core/tests/header_claims.rs`.
  - Action: Acceptance recorded. Do not repeat implementation; protect it with the listed regressions.
  - Acceptance: No incorrect delivery/sent event. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `cargo test --locked --workspace --all-targets --no-fail-fast; cargo fmt --all -- --check; python3 scripts/conductor_doctor.py --check`.
  - Evidence: `conductor/tracks/01_ingestion/evidence.md`.
  - Implemented: Claimed header Date and caller observation time remain distinct; no fabricated delivery event.
  - Remaining: Independent verified transport timestamps require separate source evidence.

- [x] Task: **T01.06 — Extract bounded attachment metadata, never execute or fetch**

  - Lane: `core`; dependencies: T01.05.
  - Requirements: R01, R02, R03, R09, R11.
  - Allowed paths: `crates/mailweft-core/src/attachments.rs`, `crates/mailweft-core/src/mime.rs`, `crates/mailweft-core/tests/attachment_metadata.rs`.
  - Action: Acceptance recorded. Do not repeat implementation; protect it with the listed regressions.
  - Acceptance: No remote fetch and no payload logging. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - Verification: `cargo test --locked --workspace --all-targets --no-fail-fast; cargo fmt --all -- --check; python3 scripts/conductor_doctor.py --check`.
  - Evidence: `conductor/tracks/01_ingestion/evidence.md`.
  - Implemented: Bounded source-derived attachment metadata is emitted without fetching/executing/logging payloads.
  - Remaining: Content extraction, MIME adversarial corpus validation and content verification are separate work.

- [x] Task: **T01.07 — Emit canonical acquisition manifest with complete/partial status**

  - Lane: `core`; dependencies: T01.06.
  - Requirements: R01, R02, R03, R09, R11.
  - Allowed paths: `crates/mailweft-cli/tests/corpus_reconciliation.rs`, `crates/mailweft-core/src/corpus.rs`, `crates/mailweft-core/src/manifest.rs`.
  - Action: Acceptance recorded. Do not repeat implementation; protect it with the listed regressions.
  - Acceptance: Supplied equals accepted plus rejected; deterministic manifest and explicit partial projection; no rejected source becomes an event.
  - Verification: `cargo test --locked --workspace --all-targets --no-fail-fast; cargo fmt --all -- --check; python3 scripts/conductor_doctor.py --check`.
  - Evidence: `conductor/tracks/01_ingestion/evidence.md`.
  - Implemented: Each supplied slot has an accepted/rejected outcome, deterministic manifest digest and explicit partial/withheld projection.
  - Remaining: Completeness covers only the supplied selection; automatic discovery, streaming and archival attestation remain separate.

- [x] Task: **T01.08 — Expose supplied-email and selection-reconciliation CLI with non-sensitive diagnostics**

  - Lane: `core`; dependencies: T01.07.
  - Requirements: R01, R02, R03, R09, R11.
  - Allowed paths: `crates/mailweft-cli/src/main.rs`, `crates/mailweft-cli/src/reconcile.rs`.
  - Action: Acceptance recorded. Do not repeat implementation; protect it with the listed regressions.
  - Acceptance: Single/batch/convert-evidence/reconcile fictional fixtures run offline with documented exit codes and bounded input.
  - Verification: `cargo test --locked --workspace --all-targets --no-fail-fast; cargo fmt --all -- --check; python3 scripts/conductor_doctor.py --check`.
  - Evidence: `conductor/tracks/01_ingestion/evidence.md`.
  - Implemented: Offline CLI covers supplied single/batch emails, source verification and selection reconciliation with non-sensitive diagnostics.
  - Remaining: The command is reconcile rather than the originally proposed ingest; no acquisition/recursion is implied.
