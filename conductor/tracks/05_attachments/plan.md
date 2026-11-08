# Opt-in text and attachment content extraction — executable task plan

Read the track spec, AGENTS.md and the task manifest. Each unchecked task is unimplemented. Dependencies are exact task IDs, not whole-track gates.

## [ ] **T05.01 — Define explicit content-access classification and size policy**

- Lane: `enrichment`; dependencies: T01.06.
- Requirements: R16, R20.
- Allowed paths: `contracts/v1/content-policy.md`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: Default denies source content beyond Stage 1. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/05_attachments/evidence.md`.

## [ ] **T05.02 — Scaffold optional semantics crate; implement body part selection and HTML sanitisation**

- Lane: `enrichment`; dependencies: T05.01.
- Requirements: R16, R20.
- Allowed paths: `Cargo.toml`, `crates/mailweft-semantics/Cargo.toml`, `crates/mailweft-semantics/src/lib.rs`, `crates/mailweft-semantics/src/body.rs`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: No remote images/execution, fallback charset safe. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/05_attachments/evidence.md`.

## [ ] **T05.03 — Implement nested message/rfc822 guard**

- Lane: `enrichment`; dependencies: T05.02.
- Requirements: R16, R20.
- Allowed paths: `crates/mailweft-semantics/src/nested.rs`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: Bounded recursion and source refs. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/05_attachments/evidence.md`.

## [ ] **T05.04 — Expose decoded body-part boundary for the single 05b zoning implementation**

- Lane: `enrichment`; dependencies: T05.02, T05.03.
- Requirements: R16, R20.
- Allowed paths: `crates/mailweft-semantics/src/segments.rs`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: Expose only a typed decoded-part boundary; do not implement a second signature/quote classifier. Track 05b owns zoning.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/05_attachments/evidence.md`.

## [ ] **T05.05 — Add optional document extractor provider interface**

- Lane: `enrichment`; dependencies: T05.04.
- Requirements: R16, R20.
- Allowed paths: `crates/mailweft-semantics/src/attachments.rs`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: No mandatory PDF/Office runtime. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/05_attachments/evidence.md`.

## [ ] **T05.06 — Implement archive bomb, malware and path protections**

- Lane: `enrichment`; dependencies: T05.05.
- Requirements: R16, R20.
- Allowed paths: `crates/mailweft-semantics/tests/security.rs`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: Adversarial fixtures reject safely. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/05_attachments/evidence.md`.

## [ ] **T05.07 — Add resource limits and local-only execution policy**

- Lane: `enrichment`; dependencies: T05.06.
- Requirements: R16, R20.
- Allowed paths: `crates/mailweft-semantics/src/policy.rs`.
- Action: Write or modify the named capability; first create a narrowly scoped synthetic test or document assertion, then implement the minimal change.
- Acceptance: No external network by default. Run targeted `cargo test` and `cargo fmt --check` once Cargo exists; until then verify document syntax and mark runtime checks BLOCKED.
- Verification: `Run targeted cargo test + cargo fmt when rust workspace and toolchain exist; for docs only run scripts/validate_scaffold.py; missing prerequisites => blocked`.
- Evidence: `conductor/tracks/05_attachments/evidence.md`.
