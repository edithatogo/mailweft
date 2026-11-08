# Plan — 05b: Structural email zoning and in-text participant enrichment

**Execution rule:** one task per coding-agent invocation/commit. Complete in order, run only targeted tests, record actual evidence, do not mark other tracks done.

- [ ] **T05b.01 — Define nested email zone/participant contracts and Rust serde types**
  - **Depends on:** 05_attachments accepted.
  - **Edit only:** `crates/email-ocel-semantics/src/zones_contract.rs`, `contracts/v1/email-zone.schema.json`, `contracts/v1/intext-participant.schema.json` and directly associated same-track tests.
  - **Action:** write synthetic failing tests/fixtures first; implement only the named feature; use maintained upstream libraries; preserve existing signatures and provenance.
  - **Verify:** Synthetic JSON fixtures accepted/rejected by Draft 2020-12 and Rust serde; no raw body strings in evidence outputs. Run targeted `cargo test`, `cargo fmt --check`, and applicable contract validation. If prerequisites unavailable, record BLOCKED, not PASS.
  - **Done when:** evidence/commands/exit codes in `conductor/tracks/05b_email-structure/evidence.md`, task checked, and a separate commit with `T05b.01: <concise result>`.

- [ ] **T05b.02 — Implement deterministic body zone baseline on decoded MIME parts**
  - **Depends on:** T05b.01 accepted.
  - **Edit only:** `crates/email-ocel-semantics/src/zones.rs` and exact directly associated same-track tests/contracts if required.
  - **Action:** write synthetic failing tests/fixtures first; implement only the named feature; use maintained upstream libraries; preserve existing signatures and provenance.
  - **Verify:** Return source-anchored authored, greeting, signoff, quoted, forwarded, signature, disclaimer, inline-header and unknown zones with full ordered coverage and no silent drop. Run targeted `cargo test`, `cargo fmt --check`, and applicable contract validation. If prerequisites unavailable, record BLOCKED, not PASS.
  - **Done when:** evidence/commands/exit codes in `conductor/tracks/05b_email-structure/evidence.md`, task checked, and a separate commit with `T05b.02: <concise result>`.

- [ ] **T05b.03 — Preserve span/offset provenance through decoding and normalization**
  - **Depends on:** T05b.02 accepted.
  - **Edit only:** `crates/email-ocel-semantics/src/offsets.rs` and exact directly associated same-track tests/contracts if required.
  - **Action:** write synthetic failing tests/fixtures first; implement only the named feature; use maintained upstream libraries; preserve existing signatures and provenance.
  - **Verify:** Original MIME part bytes and normalized Unicode body segments can be linked with verified part hash; malformed UTF-8 and HTML alternatives tested. Run targeted `cargo test`, `cargo fmt --check`, and applicable contract validation. If prerequisites unavailable, record BLOCKED, not PASS.
  - **Done when:** evidence/commands/exit codes in `conductor/tracks/05b_email-structure/evidence.md`, task checked, and a separate commit with `T05b.03: <concise result>`.

- [ ] **T05b.04 — Create separate authored, identity and conversation-context views**
  - **Depends on:** T05b.03 accepted.
  - **Edit only:** `crates/email-ocel-semantics/src/views.rs` and exact directly associated same-track tests/contracts if required.
  - **Action:** write synthetic failing tests/fixtures first; implement only the named feature; use maintained upstream libraries; preserve existing signatures and provenance.
  - **Verify:** Original protected content remains intact; signatures are excluded from speech-act view but contribute role hints only to permitted identity view; no double counting quoted history. Run targeted `cargo test`, `cargo fmt --check`, and applicable contract validation. If prerequisites unavailable, record BLOCKED, not PASS.
  - **Done when:** evidence/commands/exit codes in `conductor/tracks/05b_email-structure/evidence.md`, task checked, and a separate commit with `T05b.04: <concise result>`.

- [ ] **T05b.05 — Detect salutations, vocatives, signatures and sign-off markers**
  - **Depends on:** T05b.04 accepted.
  - **Edit only:** `crates/email-ocel-semantics/src/address_in_text.rs` and exact directly associated same-track tests/contracts if required.
  - **Action:** write synthetic failing tests/fixtures first; implement only the named feature; use maintained upstream libraries; preserve existing signatures and provenance.
  - **Verify:** Detect multi-person greetings, role titles, ambiguous name-only text, short closings, forwarded signatures; emit candidate assertions with calibrated uncertainty, never infer To/Cc headers. Run targeted `cargo test`, `cargo fmt --check`, and applicable contract validation. If prerequisites unavailable, record BLOCKED, not PASS.
  - **Done when:** evidence/commands/exit codes in `conductor/tracks/05b_email-structure/evidence.md`, task checked, and a separate commit with `T05b.05: <concise result>`.

- [ ] **T05b.06 — Reconcile header roles and in-text participant candidates without conflation**
  - **Depends on:** T05b.05 accepted.
  - **Edit only:** `crates/email-ocel-semantics/src/person_link.rs` and exact directly associated same-track tests/contracts if required.
  - **Action:** write synthetic failing tests/fixtures first; implement only the named feature; use maintained upstream libraries; preserve existing signatures and provenance.
  - **Verify:** Explicit Bcc unknown/empty/present distinctions; header roles remain observed claims; in-text names are inferred; task actor is not assigned solely from To field. Run targeted `cargo test`, `cargo fmt --check`, and applicable contract validation. If prerequisites unavailable, record BLOCKED, not PASS.
  - **Done when:** evidence/commands/exit codes in `conductor/tracks/05b_email-structure/evidence.md`, task checked, and a separate commit with `T05b.06: <concise result>`.

- [ ] **T05b.07 — Add synthetic zone and participant benchmark with per-class errors**
  - **Depends on:** T05b.06 accepted.
  - **Edit only:** `crates/email-ocel-semantics/tests/zones_and_participants.rs` and exact directly associated same-track tests/contracts if required.
  - **Action:** write synthetic failing tests/fixtures first; implement only the named feature; use maintained upstream libraries; preserve existing signatures and provenance.
  - **Verify:** F1 and boundary agreement reported for synthetic gold; Bcc privacy, quote attribution, multilingual, false-actor and multiple-recipient adversarial cases covered. Run targeted `cargo test`, `cargo fmt --check`, and applicable contract validation. If prerequisites unavailable, record BLOCKED, not PASS.
  - **Done when:** evidence/commands/exit codes in `conductor/tracks/05b_email-structure/evidence.md`, task checked, and a separate commit with `T05b.07: <concise result>`.

- [ ] **T05b.08 — Document independent Quagga, Zebra, Forge and EPA evaluation protocols**
  - **Depends on:** T05b.07 accepted.
  - **Edit only:** `docs/EMAIL-ZONING-EVALUATION.md` and exact directly associated same-track tests/contracts if required.
  - **Action:** write synthetic failing tests/fixtures first; implement only the named feature; use maintained upstream libraries; preserve existing signatures and provenance.
  - **Verify:** Dataset rights and version gate explicit; benchmark scores marked UNRUN until acquired and evaluated; thread/person-level holdout prevents leakage. Run targeted `cargo test`, `cargo fmt --check`, and applicable contract validation. If prerequisites unavailable, record BLOCKED, not PASS.
  - **Done when:** evidence/commands/exit codes in `conductor/tracks/05b_email-structure/evidence.md`, task checked, and a separate commit with `T05b.08: <concise result>`.

## Track completion
- [ ] Every task passes its specific acceptance or is explicitly blocked; stage 2 must not start while this track is unaccepted.
- [ ] No copyright, patient, staff or real Enron source material in repository.
