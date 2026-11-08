# Core converter execution ledger — 2026-11-08

Baseline: 14378d44f581d4dc0fddcb92c5d70b82439c90b4. Branch: feat/raw-email-ocel.

## Rulings
- Execute the accepted core+CLI architecture as two crates, mailweft-core and mailweft-cli; the planned seven logical components are modules, not seven packages.
- The local work environment has neither Rust nor outbound DNS. Execute real red/green tests in an isolated GitHub-hosted branch; do not claim local compilation.
- Expand older generic task assertions into the checked-in conversion tests. This bounded vertical slice contributes to T00, T01, T02 and T03 but does not close unfinished threading, legacy address, alternative-format, fuzz or release requirements.
- Stage 1 output preserves header-role assertions, observation versus claimed chronology, exact-copy lineage and typed attributes. Participant pseudonyms use a caller-supplied 32-byte key scoped to the dataset; pseudonymisation is not anonymity.
- No source content, addresses, subjects, private source paths, model prompts, or key material may be serialized. Header names/roles and protected hashes are permitted.
- Enron remains examples/verification only. No external corpus, FOI pipeline, GraphRAG or ontology change is included.

## Red checkpoint
Frozen convert(inputs, config) interface and 19 independently specified fictional tests. The first checkpoint returns NOT_IMPLEMENTED. Record real assertion failures after successful compilation, not a missing-tool or compiler failure. Bootstrap lock workflow may write only generated Cargo.lock and rustfmt changes to this isolated branch; retire it before review.

## Intended green slice
Bounded byte parsing, participant-role extraction, conservative identity, raw-byte provenance, deterministic full native OCEL projection, explicit chronology views, receipt and local CLI. Unsupported features remain visible. No checkbox is completed before its actual acceptance is met.
