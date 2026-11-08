# Plan — 03: Canonical OCEL 2.0 projection and Rust4PM verification

**Execution rule:** One task per agent turn/commit. Complete tasks in numeric order. Update checkboxes only after acceptance and record actual commands/results in `evidence.md`.
**Initial state:** Not started. Any unavailable requirement/tool is BLOCKED, not a pass.

- [ ] **T03.01 — Lock Rust4PM process_mining API and audit supported formats**
  - **Edit only:** `docs/OCEL_COMPAT.md` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** Exact version and capabilities documented. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/03_ocel/evidence.md`, step committed as `T03.01: <short description>`.

- [ ] **T03.02 — Define type and qualifier vocabulary in mapping contract**
  - **Edit only:** `contracts/v1/ocel-mapping.md` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** Typed E2O/O2O roles precise. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/03_ocel/evidence.md`, step committed as `T03.02: <short description>`.

- [ ] **T03.03 — Map Stage 1 message/source/participant objects and events**
  - **Edit only:** `crates/email-ocel-core/src/ocel.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** One source copy not falsely equal message delivered. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/03_ocel/evidence.md`, step committed as `T03.03: <short description>`.

- [ ] **T03.04 — Produce full OCEL 2.0 native JSON using Rust4PM**
  - **Edit only:** `crates/email-ocel-cli/src/export.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** Independent import succeeds. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/03_ocel/evidence.md`, step committed as `T03.04: <short description>`.

- [ ] **T03.05 — Round-trip variable object attributes + typed values**
  - **Edit only:** `crates/email-ocel-core/tests/ocel_roundtrip.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** No dropped type/change history. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/03_ocel/evidence.md`, step committed as `T03.05: <short description>`.

- [ ] **T03.06 — Round-trip E2O/O2O qualifiers and multiplicity**
  - **Edit only:** `crates/email-ocel-core/tests/ocel_relations.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** Equality of relation count and qualifiers. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/03_ocel/evidence.md`, step committed as `T03.06: <short description>`.

- [ ] **T03.07 — Run Rust4PM basic object-centric mining smoke test**
  - **Edit only:** `crates/email-ocel-core/tests/mining.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** Algorithm runs against emitted fixture. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/03_ocel/evidence.md`, step committed as `T03.07: <short description>`.

- [ ] **T03.08 — Add loss ledger and format compatibility matrix**
  - **Edit only:** `docs/OCEL_COMPAT.md` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** Every unsupported field reported, not erased. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/03_ocel/evidence.md`, step committed as `T03.08: <short description>`.

## Phase completion gate
- [ ] All Txx.yy steps verified or documented explicitly deferred with justification.
- [ ] Run appropriate module integration tests, audit source/data leakage and record results in `evidence.md`.
- [ ] No claim of operational or externally validated acceptance without real receipts.
