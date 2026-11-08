# Plan — 08: Interoperability, packaging and downstream consumer boundaries

**Execution rule:** One task per agent turn/commit. Complete tasks in numeric order. Update checkboxes only after acceptance and record actual commands/results in `evidence.md`.
**Initial state:** Not started. Any unavailable requirement/tool is BLOCKED, not a pass.

- [ ] **T08.01 — Document stable library API and SemVer mapping**
  - **Edit only:** `docs/API.md` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** No Enron-specific types in public API. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/08_release/evidence.md`, step committed as `T08.01: <short description>`.

- [ ] **T08.02 — Add optional local MBOX input adapter**
  - **Edit only:** `crates/email-ocel-cli/src/adapter_mbox.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** Streaming, byte-exact item provenance. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/08_release/evidence.md`, step committed as `T08.02: <short description>`.

- [ ] **T08.03 — Add native OCEL XML/SQLite adapters where supported**
  - **Edit only:** `crates/email-ocel-core/src/formats.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** Loss ledger and qualified parity tests. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/08_release/evidence.md`, step committed as `T08.03: <short description>`.

- [ ] **T08.04 — Create optional cross-engine reference validation**
  - **Edit only:** `tests/pm4py_optional.md` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** Licensed isolated run, no Python runtime requirement. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/08_release/evidence.md`, step committed as `T08.04: <short description>`.

- [ ] **T08.05 — Document independent CareOps adapter interface only**
  - **Edit only:** `docs/CONSUMERS.md` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** No CareOps dependency. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/08_release/evidence.md`, step committed as `T08.05: <short description>`.

- [ ] **T08.06 — Add macOS arm64, Linux x64, Windows x64 CI matrix**
  - **Edit only:** `.github/workflows/ci.yml` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** All synthetic stages pass or blocked explicitly. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/08_release/evidence.md`, step committed as `T08.06: <short description>`.

- [ ] **T08.07 — Package crate release with notices and docs**
  - **Edit only:** `docs/RELEASE.md` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** No source data included, licence audit pass. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/08_release/evidence.md`, step committed as `T08.07: <short description>`.

- [ ] **T08.08 — Write independent end-to-end acceptance summary**
  - **Edit only:** `docs/ACCEPTANCE.md` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** Every requirement R01-R26 verified or deferred. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/08_release/evidence.md`, step committed as `T08.08: <short description>`.

## Phase completion gate
- [ ] All Txx.yy steps verified or documented explicitly deferred with justification.
- [ ] Run appropriate module integration tests, audit source/data leakage and record results in `evidence.md`.
- [ ] No claim of operational or externally validated acceptance without real receipts.
