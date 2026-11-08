# Plan — 01: Raw EML and maildir input, manifests and parse boundaries

**Execution rule:** One task per agent turn/commit. Complete tasks in numeric order. Update checkboxes only after acceptance and record actual commands/results in `evidence.md`.
**Initial state:** Not started. Any unavailable requirement/tool is BLOCKED, not a pass.

- [ ] **T01.01 — Implement source envelope types and bounded file reader**
  - **Edit only:** `crates/email-ocel-core/src/source.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** Hash byte-exact source content; source only local. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/01_ingestion/evidence.md`, step committed as `T01.01: <short description>`.

- [ ] **T01.02 — Implement maildir traversal and ordered EML discovery**
  - **Edit only:** `crates/email-ocel-cli/src/walk.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** No traversal outside root; reproducible ordering. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/01_ingestion/evidence.md`, step committed as `T01.02: <short description>`.

- [ ] **T01.03 — Integrate mail-parser with error map**
  - **Edit only:** `crates/email-ocel-core/src/parser.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** Malformed samples reject with stable codes. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/01_ingestion/evidence.md`, step committed as `T01.03: <short description>`.

- [ ] **T01.04 — Capture all essential message header and address fields**
  - **Edit only:** `crates/email-ocel-core/src/message.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** Synthetic multipart EML matches expected structure. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/01_ingestion/evidence.md`, step committed as `T01.04: <short description>`.

- [ ] **T01.05 — Separate header time from source observation and source receipt**
  - **Edit only:** `crates/email-ocel-core/src/time.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** No incorrect delivery/sent event. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/01_ingestion/evidence.md`, step committed as `T01.05: <short description>`.

- [ ] **T01.06 — Extract bounded attachment metadata, never execute or fetch**
  - **Edit only:** `crates/email-ocel-core/src/mime.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** No remote fetch and no payload logging. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/01_ingestion/evidence.md`, step committed as `T01.06: <short description>`.

- [ ] **T01.07 — Emit canonical acquisition manifest with complete/partial status**
  - **Edit only:** `crates/email-ocel-core/src/manifest.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** Input=accepted+rejected, hash valid. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/01_ingestion/evidence.md`, step committed as `T01.07: <short description>`.

- [ ] **T01.08 — Add CLI ingest and non-sensitive diagnostic output**
  - **Edit only:** `crates/email-ocel-cli/src/main.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** cargo run ingest fixture works offline. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/01_ingestion/evidence.md`, step committed as `T01.08: <short description>`.

## Phase completion gate
- [ ] All Txx.yy steps verified or documented explicitly deferred with justification.
- [ ] Run appropriate module integration tests, audit source/data leakage and record results in `evidence.md`.
- [ ] No claim of operational or externally validated acceptance without real receipts.
