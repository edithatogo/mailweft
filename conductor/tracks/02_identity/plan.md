# Plan — 02: Message/custodian identity, duplicates and conversation inference

**Execution rule:** One task per agent turn/commit. Complete tasks in numeric order. Update checkboxes only after acceptance and record actual commands/results in `evidence.md`.
**Initial state:** Not started. Any unavailable requirement/tool is BLOCKED, not a pass.

- [ ] **T02.01 — Implement domain-separated stable identity types**
  - **Edit only:** `crates/email-ocel-core/src/identity.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** Same bytes+config => same ID; no raw identity in output. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/02_identity/evidence.md`, step committed as `T02.01: <short description>`.

- [ ] **T02.02 — Differentiate source copies and logical message candidates**
  - **Edit only:** `crates/email-ocel-core/src/copies.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** Two custodians retain separate copies. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/02_identity/evidence.md`, step committed as `T02.02: <short description>`.

- [ ] **T02.03 — Handle missing and colliding Message-ID values**
  - **Edit only:** `crates/email-ocel-core/src/dedup.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** No silent data loss or overwrite. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/02_identity/evidence.md`, step committed as `T02.03: <short description>`.

- [ ] **T02.04 — Implement explicit To/Cc/Bcc role assertions**
  - **Edit only:** `crates/email-ocel-core/src/parties.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** Roles round-trip without duplicates introduced. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/02_identity/evidence.md`, step committed as `T02.04: <short description>`.

- [ ] **T02.05 — Implement RFC References and In-Reply-To thread graph**
  - **Edit only:** `crates/email-ocel-core/src/threads.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** Truth graph fixtures pass. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/02_identity/evidence.md`, step committed as `T02.05: <short description>`.

- [ ] **T02.06 — Add subject/quote fallback as inferred links only**
  - **Edit only:** `crates/email-ocel-core/src/heuristics.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** Ambiguous collisions abstain, links uncertain. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/02_identity/evidence.md`, step committed as `T02.06: <short description>`.

- [ ] **T02.07 — Implement legacy Exchange/X500/IMCEA adapter**
  - **Edit only:** `crates/email-ocel-core/src/legacy.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** Retain unresolved identity assertions, no false merges. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/02_identity/evidence.md`, step committed as `T02.07: <short description>`.

- [ ] **T02.08 — Add thread graph cycle/conflict and duplication tests**
  - **Edit only:** `crates/email-ocel-core/tests/threading.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** No cycles passed as confirmed thread. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/02_identity/evidence.md`, step committed as `T02.08: <short description>`.

## Phase completion gate
- [ ] All Txx.yy steps verified or documented explicitly deferred with justification.
- [ ] Run appropriate module integration tests, audit source/data leakage and record results in `evidence.md`.
- [ ] No claim of operational or externally validated acceptance without real receipts.
