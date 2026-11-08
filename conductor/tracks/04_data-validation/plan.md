# Plan — 04: Synthetic and Enron raw-to-processed validation

**Execution rule:** One task per agent turn/commit. Complete tasks in numeric order. Update checkboxes only after acceptance and record actual commands/results in `evidence.md`.
**Initial state:** Not started. Any unavailable requirement/tool is BLOCKED, not a pass.

- [ ] **T04.01 — Invent version-controlled synthetic MIME corpus generator**
  - **Edit only:** `tools/generate_fixture.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** No real addresses, reproducible deterministic files. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/04_data-validation/evidence.md`, step committed as `T04.01: <short description>`.

- [ ] **T04.02 — Write rights-gated raw corpus manifest template**
  - **Edit only:** `datasets/README.md` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** Requires real source/version/rights and SHA256. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/04_data-validation/evidence.md`, step committed as `T04.02: <short description>`.

- [ ] **T04.03 — Build optional CMU raw maildir reader adapter**
  - **Edit only:** `crates/email-ocel-cli/src/adapter_cmu.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** Local source, no automatic download. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/04_data-validation/evidence.md`, step committed as `T04.03: <short description>`.

- [ ] **T04.04 — Build optional EDRM EML/XML metadata reconciler**
  - **Edit only:** `crates/email-ocel-cli/src/adapter_edrm.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** Correlate source hashes and attachment metadata. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/04_data-validation/evidence.md`, step committed as `T04.04: <short description>`.

- [ ] **T04.05 — Build Berti OCEL reference reader with record scope checks**
  - **Edit only:** `crates/email-ocel-cli/src/adapter_reference.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** No unsupported one-to-one claims. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/04_data-validation/evidence.md`, step committed as `T04.05: <short description>`.

- [ ] **T04.06 — Implement candidate cross-corpus record linkage**
  - **Edit only:** `crates/email-ocel-core/src/matching.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** exact/likely/ambiguous/unmatched classes. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/04_data-validation/evidence.md`, step committed as `T04.06: <short description>`.

- [ ] **T04.07 — Generate paired/unpaired validation reports**
  - **Edit only:** `crates/email-ocel-cli/src/validation.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** Denominators and exclusions explicitly stated. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/04_data-validation/evidence.md`, step committed as `T04.07: <short description>`.

- [ ] **T04.08 — Run 10k/100k/full corpus benchmarks only after gate**
  - **Edit only:** `benchmarks/README.md` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** Hardware/RSS/time/source SHA and exclusions. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/04_data-validation/evidence.md`, step committed as `T04.08: <short description>`.

## Phase completion gate
- [ ] All Txx.yy steps verified or documented explicitly deferred with justification.
- [ ] Run appropriate module integration tests, audit source/data leakage and record results in `evidence.md`.
- [ ] No claim of operational or externally validated acceptance without real receipts.
