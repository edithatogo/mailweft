# Track 05b — Functional email zoning and in-text participant assertions

**Status:** Not started. **Priority:** immediately after authorised text parsing and before Stage 2 speech-act NLP. **Dependencies:** 05_attachments completed, contracts and synthetic tests valid.

## User story
As a process-reconstruction developer, I need a reliable, source-anchored way to distinguish who received a message, who it addresses in prose, and which content is newly authored versus quoted or boilerplate, before attempting NLP business event extraction.

## Design constraints
- Keep Stage 1 transport/header-role evidence independent of Stage 1.5 inferred in-text mentions.
- No deletion of original content; controlled views only. Precise decoded-body offsets and evidence hashes; parent/child/overlap support.
- Bcc omission means unknown visibility, not a known absence of blind recipients. A copied recipient is not necessarily informational or an assigned actor.
- Signature can contain valid identity or organisation hints even when omitted from action-extraction input.
- No email/attachment payloads committed, sent to an external model or logged. No activity inference from quoted material without attribution.
- Prefer maintained dependencies; do not bundle Quagga GPL implementation into Apache-licensed runtime. External data rights must be reviewed.

## Inputs/outputs
Input: decoded MIME part and approved content reference, observed message/header roles. Output: versioned zone spans and in-text participant assertion records, plus distinct semantic/identity/context projections. Detailed contracts in `contracts/v1/email-zone.schema.json`, `contracts/v1/intext-participant.schema.json` and `docs/STRUCTURAL-EMAIL-NLP.md`.

## Acceptance
Eight small independently tested tasks, no false OCEL observed events, exact raw-source reference mapping, a reproducible synthetic evaluation and an external benchmark protocol with data rights explicitly deferred.
