# Plan — 05: Opt-in text and attachment content extraction

**Execution rule:** One task per agent turn/commit. Complete tasks in numeric order. Update checkboxes only after acceptance and record actual commands/results in `evidence.md`.
**Initial state:** Not started. Any unavailable requirement/tool is BLOCKED, not a pass.

- [ ] **T05.01 — Define explicit content-access classification and size policy**
  - **Edit only:** `contracts/v1/content-policy.md` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** Default denies source content beyond Stage 1. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/05_attachments/evidence.md`, step committed as `T05.01: <short description>`.

- [ ] **T05.02 — Implement body part selection and HTML sanitisation**
  - **Edit only:** `crates/email-ocel-semantics/src/body.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** No remote images/execution, fallback charset safe. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/05_attachments/evidence.md`, step committed as `T05.02: <short description>`.

- [ ] **T05.03 — Implement nested message/rfc822 guard**
  - **Edit only:** `crates/email-ocel-semantics/src/nested.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** Bounded recursion and source refs. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/05_attachments/evidence.md`, step committed as `T05.03: <short description>`.

- [ ] **T05.04 — Implement authored/quoted/forwarded/signature spans**
  - **Edit only:** `crates/email-ocel-semantics/src/segments.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** Offsets point back to original source. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/05_attachments/evidence.md`, step committed as `T05.04: <short description>`.

- [ ] **T05.05 — Add optional document extractor provider interface**
  - **Edit only:** `crates/email-ocel-semantics/src/attachments.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** No mandatory PDF/Office runtime. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/05_attachments/evidence.md`, step committed as `T05.05: <short description>`.

- [ ] **T05.06 — Implement archive bomb, malware and path protections**
  - **Edit only:** `crates/email-ocel-semantics/tests/security.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** Adversarial fixtures reject safely. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/05_attachments/evidence.md`, step committed as `T05.06: <short description>`.

- [ ] **T05.07 — Add resource limits and local-only execution policy**
  - **Edit only:** `crates/email-ocel-semantics/src/policy.rs` (and same-track tests/docs when necessary).
  - **Action:** Implement the named capability against `docs/CONTRACT.md`; start with a failing invented/synthetic test or schema assertion, then minimal fix.
  - **Verify:** No external network by default. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
  - **Done when:** diff reviewed, actual commands and exit codes appended to `conductor/tracks/05_attachments/evidence.md`, step committed as `T05.07: <short description>`.

## Phase completion gate
- [ ] All Txx.yy steps verified or documented explicitly deferred with justification.
- [ ] Run appropriate module integration tests, audit source/data leakage and record results in `evidence.md`.
- [ ] No claim of operational or externally validated acceptance without real receipts.
