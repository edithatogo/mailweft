# Raw EML and maildir input, manifests and parse boundaries — executable task plan

Read the track spec, AGENTS.md and the task manifest. Each unchecked task is unimplemented. Dependencies are exact task IDs, not whole-track gates.

## [ ] **T01.01 — Implement source envelope types and bounded file reader**

- Lane: `core`; dependencies: T00.05, T00.03.
- Requirements: R01, R02, R03, R09, R11.
- Allowed paths: `crates/mailweft-core/src/source.rs`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: Hash byte-exact source content; source only local. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/01_ingestion/evidence.md`.

## [ ] **T01.02 — Implement maildir traversal and ordered EML discovery**

- Lane: `core`; dependencies: T01.01.
- Requirements: R01, R02, R03, R09, R11.
- Allowed paths: `crates/mailweft-cli/src/walk.rs`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: No traversal outside root; reproducible ordering. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/01_ingestion/evidence.md`.

## [ ] **T01.03 — Integrate mail-parser with error map**

- Lane: `core`; dependencies: T01.02.
- Requirements: R01, R02, R03, R09, R11, R48.
- Allowed paths: `crates/mailweft-core/src/parser.rs`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: Malformed samples reject with stable codes. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/01_ingestion/evidence.md`.

## [ ] **T01.04 — Capture all essential message header and address fields**

- Lane: `core`; dependencies: T01.03.
- Requirements: R01, R02, R03, R09, R11.
- Allowed paths: `crates/mailweft-core/src/message.rs`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: Synthetic multipart EML matches expected structure. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/01_ingestion/evidence.md`.

## [ ] **T01.05 — Separate header time from source observation and source receipt**

- Lane: `core`; dependencies: T01.04.
- Requirements: R01, R02, R03, R09, R11.
- Allowed paths: `crates/mailweft-core/src/time.rs`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: No incorrect delivery/sent event. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/01_ingestion/evidence.md`.

## [ ] **T01.06 — Extract bounded attachment metadata, never execute or fetch**

- Lane: `core`; dependencies: T01.05.
- Requirements: R01, R02, R03, R09, R11.
- Allowed paths: `crates/mailweft-core/src/mime.rs`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: No remote fetch and no payload logging. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/01_ingestion/evidence.md`.

## [ ] **T01.07 — Emit canonical acquisition manifest with complete/partial status**

- Lane: `core`; dependencies: T01.06.
- Requirements: R01, R02, R03, R09, R11.
- Allowed paths: `crates/mailweft-core/src/manifest.rs`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: Input=accepted+rejected, hash valid. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/01_ingestion/evidence.md`.

## [ ] **T01.08 — Add CLI ingest and non-sensitive diagnostic output**

- Lane: `core`; dependencies: T01.07.
- Requirements: R01, R02, R03, R09, R11.
- Allowed paths: `crates/mailweft-cli/src/main.rs`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: cargo run ingest fixture works offline. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/01_ingestion/evidence.md`.
