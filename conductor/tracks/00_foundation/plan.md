# Plan — 00: Foundation, legal/source gates and Rust workspace

**Execution rule:** One task per agent turn/commit. Complete tasks in numeric order. Update checkboxes only after acceptance and record actual commands/results in `evidence.md`.
**Initial state:** Not started. Any unavailable requirement/tool is BLOCKED, not a pass.

- [ ] **T00.01 — Inventory dependency licences and record SPDX + exact pinned candidate versions**
  - **Edit only:** `docs/DEPENDENCIES.md` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** Validate upstream source licences; no unlicensed code copied. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/00_foundation/evidence.md`, step committed as `T00.01: <short description>`.

- [ ] **T00.02 — Verify CMU, EDRM and Berti raw/reference rights and write acquisition policy**
  - **Edit only:** `docs/DATASETS.md` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** Separate rights status and permitted uses, no download. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/00_foundation/evidence.md`, step committed as `T00.02: <short description>`.

- [ ] **T00.03 — Record repository boundaries and source asset exclusions**
  - **Edit only:** `AGENTS.md .gitignore` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** No real-data files trackable by default. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/00_foundation/evidence.md`, step committed as `T00.03: <short description>`.

- [ ] **T00.04 — Create single minimal Rust workspace with email-ocel-core and CLI crates**
  - **Edit only:** `Cargo.toml crates/*` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** cargo metadata --no-deps succeeds; avoid unused crates. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/00_foundation/evidence.md`, step committed as `T00.04: <short description>`.

- [ ] **T00.05 — Add rust-toolchain pin and deterministic dependency lock**
  - **Edit only:** `rust-toolchain.toml Cargo.lock` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** Locked build reproducible on chosen toolchain. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/00_foundation/evidence.md`, step committed as `T00.05: <short description>`.

- [ ] **T00.06 — Add CI synthetic lint/test/contract and dependency security scans**
  - **Edit only:** `.github/workflows/ci.yml` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** CI requires no real mailbox/corpus data. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/00_foundation/evidence.md`, step committed as `T00.06: <short description>`.

- [ ] **T00.07 — Verify Conductor submodule and project context integrity**
  - **Edit only:** `scripts/check_conductor.sh` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** gitlink/tag/sha and track files verified. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/00_foundation/evidence.md`, step committed as `T00.07: <short description>`.

## Phase completion gate
- [ ] All Txx.yy steps verified or documented explicitly deferred with justification.
- [ ] Run appropriate module integration tests, audit source/data leakage and record results in `evidence.md`.
- [ ] No claim of operational or externally validated acceptance without real receipts.
