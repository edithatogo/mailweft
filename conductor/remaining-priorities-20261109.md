# Execution ledger — source receipts and segmentation hardening

Base: f6999713b1c42f1fc7d56d8a16bac4df649f8bf7. Scope: continue T01.01/T01.07 foundations and T05b structural work. Do not claim the entire tracks done.

## Rulings
- The existing contract calls for a SourceEnvelope but no schema or Rust implementation exists. Add a versioned strict contract and a separate convert-evidence API/CLI. Preserve existing OCEL output/identifiers; receipts verify byte hash/length, not caller authority or source completeness.
- Only raw_rfc5322 is admitted to conversion. Lower-fidelity rendered HTML/platform exports are representable metadata but rejected by this conversion boundary. No raw-source reconstruction from FYI pages.
- Local Rust execution is unavailable in this session; exact-commit GitHub Actions is the test environment. Work is isolated on a feature branch, not main.
- The CLI currently allocates whole files before validation. Use a counted limit+1 reader for all input modes, and count actual batch bytes rather than only pre-read metadata.
- Forwarded histories must not become current-author acts. Add explicit origin scope and quote depth; preserve all spans and report skipped parts. No generated confidence, actor identity, HTML rendering, or GraphRAG.

## Ordered tasks
1. Define CLI and zone regression tests; observe their failures against the existing implementation.
2. Add strict source envelope, byte-integrity verification, deterministic content-free receipts and bounded I/O. Test against corruption, malformed metadata and lower-fidelity claims.
3. Strengthen forward/quote/disclaimer zoning and explicit coverage diagnostics. Retain exact/derived source-offset distinctions.
4. Add contract schemas, migration notes, granular track evidence and exact test receipts. Validate default Rust, Rust4PM, format, Clippy and planning on the final commit.

Status: tests specified; implementation not yet performed. Core product remains a Rust email-to-OCEL converter. Enron is an example/verification corpus only; FOI consumer/ontology work remains outside this change.
