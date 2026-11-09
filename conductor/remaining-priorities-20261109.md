# Execution ledger — source receipts and segmentation hardening

Base: f6999713b1c42f1fc7d56d8a16bac4df649f8bf7. Feature branch: feat/source-envelope-integrity.

## Rulings
- Add a source contract where the product spec required one but no concrete schema existed. `convert-evidence` is additive; preserve original OCEL fields/identities.
- Byte integrity is verifiable; source timestamps, rights/scope, producer identity, source fidelity and coverage remain caller assertions. Never promote them to independent attestation.
- Lower-fidelity FYI pages/exports are representable metadata, not valid input to this raw-email converter. Capture/archive ownership remains external.
- CLI file reads were previously unbounded before parsing. Use counted limits for every mode and actual batch bytes; document remaining filesystem-race limitations.
- Scope work uses existing core/CLI modules rather than creating an extra tiny crate. The implementation-specific body-evidence schema does not replace the broader proposed email-zone schema.
- The session has no local Rust/network checkout; hosted Actions is the executed test environment. No separate reviewer agent is available; conduct and disclose a whole-diff self-review.

## Task ledger
1. Source receipt CLI tests: RED at 427418b (42587746942), GREEN at 190f050 (42588267305); that latter run was still red for zoning.
2. Source envelope/readers: implemented in source.rs; API boundary tests, private-reference exclusion and actual-output schema validation added. Core projection remains unchanged.
3. Zoning: four failures observed before implementation. Add explicit forwarding/quote origin and skipped-part diagnostics; precise encoded-byte maps remain open.
4. Version/contracts: Rust 0.6.0; SourceEnvelope/Receipt 1.0.0; BodyEvidence 1.1.0; Conversion stays 1.3.0. No new runtime dependencies.
5. Qualification: one-time branch-scoped helper formatted Rust, tested default workspace and updated workspace lock entries. It is removed before merge. Full final read-only CI remains the acceptance gate.

## Next granular tasks (not completed here)
- Source manifests: define supplied/accepted/rejected counts and diagnostic records, reconcile a synthetic mixed-quality batch, and verify no failed source becomes an event.
- Transfer mapping: test UTF-8 byte offsets, base64 quartet/whitespace/padding spans and quoted-printable soft-line-break spans separately; report mapping uncertainty rather than fabricate character offsets.
- Filesystem ingestion: qualify a maintained capability-safe reader for descriptor-relative no-follow traversal before claiming protection against hostile path races.
- Zoning: independently annotate bounded forward endings, inline answers after quotations, and multi-line disclaimers; preserve false-positive/false-negative reports.
- Only afterwards add FOI consumer adapters and optional source-constrained graph retrieval. No legacy FOI output is a golden reference.
