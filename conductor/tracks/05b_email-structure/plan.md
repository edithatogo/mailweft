# Email zoning and participant enrichment before speech-act NLP implementation plan

**Status:** all implementation tasks not started. **Goal:** the bounded specification in `spec.md`.
**Architecture:** retain independent Rust raw-email -> OCEL 2.0 core; optional lanes do not gate core.
**Tech stack:** qualified Rust dependencies; Python helpers only for development-document validation.
**Spec:** `spec.md`, `docs/CONTRACT.md`, `docs/FRONTIER-DESIGN.md`.

## Global constraints and review focus
No real email/source data in Git; no network/model fallback; no participant-role/authority conflation; preserve source attribution and OCEL attributes/qualifiers; observe exact task prerequisites, not whole-track gates. Test unknowns, malicious data, copied quotations, Unicode spans and missing/ambiguous references where applicable.

The manifest is authoritative for dependencies and lane. Checkboxes are completion records, not approval requests. Planning checks do not mark implementation complete.

- [ ] **T05b.01 — Define nested email zone/participant contracts and Rust serde types**
  - Lane: `enrichment`. Depends on: T05.02.
  - Edit only: `crates/email-ocel-semantics/src/zones_contract.rs`, `contracts/v1/email-zone.schema.json`, `contracts/v1/intext-participant.schema.json`.
  - Action: Write a failing deterministic synthetic fixture, implement only the named function, and log exact test evidence; no real email content.
  - Acceptance: Synthetic JSON fixtures accepted/rejected by Draft 2020-12 and Rust serde; no raw body strings in evidence outputs.
  - Verification: `Run targeted cargo tests/fmt where workspace exists; otherwise mark BLOCKED and run scripts/validate_scaffold.py only for documentation`.
  - Evidence: `conductor/tracks/05b_email-structure/evidence.md`. Commit: `T05b.01: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.

- [ ] **T05b.02 — Implement deterministic body zone baseline on decoded MIME parts**
  - Lane: `enrichment`. Depends on: T05b.03.
  - Edit only: `crates/email-ocel-semantics/src/zones.rs`.
  - Action: Write a failing deterministic synthetic fixture, implement only the named function, and log exact test evidence; no real email content.
  - Acceptance: Return source-anchored authored, greeting, signoff, quoted, forwarded, signature, disclaimer, inline-header and unknown zones with full ordered coverage and no silent drop. Labels must include machine-generated/unknown and support a separate coarse multi-label genre summary without claiming an observed business event.
  - Verification: `Run targeted cargo tests/fmt where workspace exists; otherwise mark BLOCKED and run scripts/validate_scaffold.py only for documentation`.
  - Evidence: `conductor/tracks/05b_email-structure/evidence.md`. Commit: `T05b.02: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.

- [ ] **T05b.03 — Preserve span/offset provenance through decoding and normalization**
  - Lane: `enrichment`. Depends on: T05b.01.
  - Edit only: `crates/email-ocel-semantics/src/offsets.rs`.
  - Action: Write a failing deterministic synthetic fixture, implement only the named function, and log exact test evidence; no real email content.
  - Acceptance: Original MIME part bytes and normalized Unicode body segments can be linked with verified part hash; malformed UTF-8 and HTML alternatives tested.
  - Verification: `Run targeted cargo tests/fmt where workspace exists; otherwise mark BLOCKED and run scripts/validate_scaffold.py only for documentation`.
  - Evidence: `conductor/tracks/05b_email-structure/evidence.md`. Commit: `T05b.03: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.

- [ ] **T05b.04 — Create separate authored, identity and conversation-context views**
  - Lane: `enrichment`. Depends on: T05b.02.
  - Edit only: `crates/email-ocel-semantics/src/views.rs`.
  - Action: Write a failing deterministic synthetic fixture, implement only the named function, and log exact test evidence; no real email content.
  - Acceptance: Original protected content remains intact; signatures are excluded from speech-act view but contribute role hints only to permitted identity view; no double counting quoted history.
  - Verification: `Run targeted cargo tests/fmt where workspace exists; otherwise mark BLOCKED and run scripts/validate_scaffold.py only for documentation`.
  - Evidence: `conductor/tracks/05b_email-structure/evidence.md`. Commit: `T05b.04: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.

- [ ] **T05b.05 — Detect salutations, vocatives, signatures and sign-off markers**
  - Lane: `enrichment`. Depends on: T05b.04.
  - Edit only: `crates/email-ocel-semantics/src/address_in_text.rs`.
  - Action: Write a failing deterministic synthetic fixture, implement only the named function, and log exact test evidence; no real email content.
  - Acceptance: Detect multi-person greetings, role titles, ambiguous name-only text, short closings, forwarded signatures; emit candidate assertions with calibrated uncertainty, never infer To/Cc headers.
  - Verification: `Run targeted cargo tests/fmt where workspace exists; otherwise mark BLOCKED and run scripts/validate_scaffold.py only for documentation`.
  - Evidence: `conductor/tracks/05b_email-structure/evidence.md`. Commit: `T05b.05: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.

- [ ] **T05b.06 — Reconcile header roles and in-text participant candidates without conflation**
  - Lane: `enrichment`. Depends on: T05b.05.
  - Edit only: `crates/email-ocel-semantics/src/person_link.rs`.
  - Action: Write a failing deterministic synthetic fixture, implement only the named function, and log exact test evidence; no real email content.
  - Acceptance: Explicit Bcc unknown/empty/present distinctions; header roles remain observed claims; in-text names are inferred; task actor is not assigned solely from To field.
  - Verification: `Run targeted cargo tests/fmt where workspace exists; otherwise mark BLOCKED and run scripts/validate_scaffold.py only for documentation`.
  - Evidence: `conductor/tracks/05b_email-structure/evidence.md`. Commit: `T05b.06: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.

- [ ] **T05b.07 — Add synthetic zone and participant benchmark with per-class errors**
  - Lane: `enrichment`. Depends on: T05b.06.
  - Edit only: `crates/email-ocel-semantics/tests/zones_and_participants.rs`.
  - Action: Write a failing deterministic synthetic fixture, implement only the named function, and log exact test evidence; no real email content.
  - Acceptance: F1 and boundary agreement reported for synthetic gold; Bcc privacy, quote attribution, multilingual, false-actor and multiple-recipient adversarial cases covered.
  - Verification: `Run targeted cargo tests/fmt where workspace exists; otherwise mark BLOCKED and run scripts/validate_scaffold.py only for documentation`.
  - Evidence: `conductor/tracks/05b_email-structure/evidence.md`. Commit: `T05b.07: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.

- [ ] **T05b.08 — Document independent Quagga, Zebra, Forge and EPA evaluation protocols**
  - Lane: `enrichment`. Depends on: T05b.07.
  - Edit only: `docs/EMAIL-ZONING-EVALUATION.md`.
  - Action: Write a failing deterministic synthetic fixture, implement only the named function, and log exact test evidence; no real email content.
  - Acceptance: Dataset rights and version gate explicit; benchmark scores marked UNRUN until acquired and evaluated; thread/person-level holdout prevents leakage.
  - Verification: `Run targeted cargo tests/fmt where workspace exists; otherwise mark BLOCKED and run scripts/validate_scaffold.py only for documentation`.
  - Evidence: `conductor/tracks/05b_email-structure/evidence.md`. Commit: `T05b.08: <concise result>`.
  - Before implementing, bind the acceptance to exact synthetic assertions and the actual predecessor interfaces; do not invent unqualified library APIs.
