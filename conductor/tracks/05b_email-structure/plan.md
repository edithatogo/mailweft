# Email zoning and participant enrichment before speech-act NLP — executable task plan

Generated from the canonical task manifest; edit that manifest, then run `python3 scripts/conductor_doctor.py --sync`.
A partial task is not unimplemented and not complete. Evidence and remaining acceptance work are explicit.

- [~] Task: **T05b.01 — Define nested email zone/participant contracts and Rust serde types**

  - Lane: `enrichment`; dependencies: T05.02.
  - Requirements: R27, R28, R29, R31.
  - Allowed paths: `contracts/v1/body-evidence.schema.json`, `contracts/v1/email-zone.schema.json`, `contracts/v1/intext-participant.schema.json`, `crates/mailweft-core/src/zones.rs`, `crates/mailweft-semantics/src/zones_contract.rs`.
  - Action: Resume from the recorded implementation; implement only remaining acceptance, with a failing regression first.
  - Acceptance: Synthetic JSON fixtures accepted/rejected by Draft 2020-12 and Rust serde; no raw body strings in evidence outputs.
  - Verification: `cargo test --locked --workspace --all-targets --no-fail-fast; cargo fmt --all -- --check; python3 scripts/conductor_doctor.py --check`.
  - Evidence: `conductor/tracks/05b_email-structure/evidence.md`.
  - Implemented: Implementation-specific BodyEvidence separates content/origin and exact/derived provenance.
  - Remaining: The proposed nested email-zone interchange model is not fully implemented.

- [~] Task: **T05b.02 — Implement deterministic body zone baseline on decoded MIME parts**

  - Lane: `enrichment`; dependencies: T05b.03.
  - Requirements: R27, R28, R29, R31.
  - Allowed paths: `crates/mailweft-core/src/zones.rs`, `crates/mailweft-core/tests/zoning_boundaries.rs`, `crates/mailweft-semantics/src/zones.rs`.
  - Action: Resume from the recorded implementation; implement only remaining acceptance, with a failing regression first.
  - Acceptance: Return source-anchored authored, greeting, signoff, quoted, forwarded, signature, disclaimer, inline-header and unknown zones with full ordered coverage and no silent drop. Labels must include machine-generated/unknown and support a separate coarse multi-label genre summary without claiming an observed business event.
  - Verification: `cargo test --locked --workspace --all-targets --no-fail-fast; cargo fmt --all -- --check; python3 scripts/conductor_doctor.py --check`.
  - Evidence: `conductor/tracks/05b_email-structure/evidence.md`.
  - Implemented: Non-destructive deterministic quote/forward/signature/disclaimer baseline and diagnostics exist.
  - Remaining: Nested attribution, multilingual coverage and independent classifier evaluation remain.

- [~] Task: **T05b.03 — Preserve span/offset provenance through decoding and normalization**

  - Lane: `enrichment`; dependencies: T05b.01.
  - Requirements: R27, R28, R29, R31.
  - Allowed paths: `crates/mailweft-core/src/transfer_map.rs`, `crates/mailweft-core/tests/transfer_provenance.rs`, `crates/mailweft-semantics/src/offsets.rs`.
  - Action: Resume from the recorded implementation; implement only remaining acceptance, with a failing regression first.
  - Acceptance: Original MIME part bytes and normalized Unicode body segments can be linked with verified part hash; malformed UTF-8 and HTML alternatives tested.
  - Verification: `cargo test --locked --workspace --all-targets --no-fail-fast; cargo fmt --all -- --check; python3 scripts/conductor_doctor.py --check`.
  - Evidence: `conductor/tracks/05b_email-structure/evidence.md`.
  - Implemented: Base64/QP transfer-unit maps are admitted only after parser-byte agreement; zone offsets remain conservative.
  - Remaining: Charset transformation maps, HTML mappings and wider encoding fuzz qualification remain open.

- [ ] Task: **T05b.04 — Create separate authored, identity and conversation-context views**

  - Lane: `enrichment`; dependencies: T05b.02.
  - Requirements: R27, R28, R29, R31.
  - Allowed paths: `crates/mailweft-semantics/src/views.rs`.
  - Action: Write a failing deterministic synthetic fixture, implement only the named function, and log exact test evidence; no real email content.
  - Acceptance: Original protected content remains intact; signatures are excluded from speech-act view but contribute role hints only to permitted identity view; no double counting quoted history.
  - Verification: `Run targeted cargo tests/fmt where workspace exists; otherwise mark BLOCKED and run scripts/validate_scaffold.py only for documentation`.
  - Evidence: `conductor/tracks/05b_email-structure/evidence.md`.

- [ ] Task: **T05b.05 — Detect salutations, vocatives, signatures and sign-off markers**

  - Lane: `enrichment`; dependencies: T05b.04.
  - Requirements: R27, R28, R29, R31.
  - Allowed paths: `crates/mailweft-semantics/src/address_in_text.rs`.
  - Action: Write a failing deterministic synthetic fixture, implement only the named function, and log exact test evidence; no real email content.
  - Acceptance: Detect multi-person greetings, role titles, ambiguous name-only text, short closings, forwarded signatures; emit candidate assertions with calibrated uncertainty, never infer To/Cc headers.
  - Verification: `Run targeted cargo tests/fmt where workspace exists; otherwise mark BLOCKED and run scripts/validate_scaffold.py only for documentation`.
  - Evidence: `conductor/tracks/05b_email-structure/evidence.md`.

- [ ] Task: **T05b.06 — Reconcile header roles and in-text participant candidates without conflation**

  - Lane: `enrichment`; dependencies: T05b.05.
  - Requirements: R27, R28, R29, R31.
  - Allowed paths: `crates/mailweft-semantics/src/person_link.rs`.
  - Action: Write a failing deterministic synthetic fixture, implement only the named function, and log exact test evidence; no real email content.
  - Acceptance: Explicit Bcc unknown/empty/present distinctions; header roles remain observed claims; in-text names are inferred; task actor is not assigned solely from To field.
  - Verification: `Run targeted cargo tests/fmt where workspace exists; otherwise mark BLOCKED and run scripts/validate_scaffold.py only for documentation`.
  - Evidence: `conductor/tracks/05b_email-structure/evidence.md`.

- [ ] Task: **T05b.07 — Add synthetic zone and participant benchmark with per-class errors**

  - Lane: `enrichment`; dependencies: T05b.06.
  - Requirements: R27, R28, R29, R31.
  - Allowed paths: `crates/mailweft-semantics/tests/zones_and_participants.rs`.
  - Action: Write a failing deterministic synthetic fixture, implement only the named function, and log exact test evidence; no real email content.
  - Acceptance: F1 and boundary agreement reported for synthetic gold; Bcc privacy, quote attribution, multilingual, false-actor and multiple-recipient adversarial cases covered.
  - Verification: `Run targeted cargo tests/fmt where workspace exists; otherwise mark BLOCKED and run scripts/validate_scaffold.py only for documentation`.
  - Evidence: `conductor/tracks/05b_email-structure/evidence.md`.

- [ ] Task: **T05b.08 — Document independent Quagga, Zebra, Forge and EPA evaluation protocols**

  - Lane: `enrichment`; dependencies: T05b.07.
  - Requirements: R27, R28, R29, R31.
  - Allowed paths: `docs/EMAIL-ZONING-EVALUATION.md`.
  - Action: Write a failing deterministic synthetic fixture, implement only the named function, and log exact test evidence; no real email content.
  - Acceptance: Dataset rights and version gate explicit; benchmark scores marked UNRUN until acquired and evaluated; thread/person-level holdout prevents leakage.
  - Verification: `Run targeted cargo tests/fmt where workspace exists; otherwise mark BLOCKED and run scripts/validate_scaffold.py only for documentation`.
  - Evidence: `conductor/tracks/05b_email-structure/evidence.md`.
