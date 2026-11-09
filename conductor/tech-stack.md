# Mailweft technical stack — implemented versus optional

- **Core/runtime:** Rust 2024; `mailweft-core` and `mailweft-cli`. The tested toolchain is pinned in rust-toolchain.toml; dependency resolution is committed in Cargo.lock. No Python runtime.
- **MIME parsing:** Stalwart mail-parser =1.0.1. Mailweft projects parsed observations, bounded source-integrity receipts, optional zoning and transfer-unit evidence. It is not a new MIME decoder.
- **OCEL:** corpus-neutral OCEL 2.0 JSON-shaped projection with explicit E2O/O2O qualifiers. Rust4PM process_mining =0.6.2 is feature-gated independent validation. Complete typed-value, cross-format and algorithm parity remain separate tasks.
- **Ingestion:** explicit supplied files and versioned source envelopes. Selection-level accepted/rejected accounting does not prove whole-mailbox or FYI completeness. Network capture/archive preservation remain external.
- **Validation tooling:** Python only for repository planning and JSON Schema checks of actual Rust outputs. Standard-library Conductor doctor synchronizes context; it does not implement or validate the scientific methods.
- **Optional/future:** maildir traversal, broader threading and legacy addresses, semantic NLP, GraphRAG, vector services, and foi-process integration. These are not mandatory core dependencies.

Use docs/DEPENDENCIES.md for the exact resolved direct dependency/licence inventory. Do not replace pinned versions silently. Keep the converter independent of Enron schemas and FOI-specific semantics.
