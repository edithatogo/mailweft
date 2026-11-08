# Technology stack

- Rust stable (version pin/edition chosen and recorded in T00.04; do not guess without qualification).
- Cargo workspace. Core dependency candidates: `mail-parser` (Stalwart), `mail-threading` (RFC references), `process_mining` (Rust4PM), serde/serde_json, hashing crates, clap and bounded file walk. Exact versions/licences pinned and audited by task.
- OCEL 2.0 canonical data structures and direct Rust import/export. Native JSON first, XML/SQLite qualified separately. No mandatory Python, JVM, WSL or CUDA.
- Optional features: EDRM XML adapter, Parquet bulk output, MIME attachment document parsing, local NLP/LLM backends, evidence graph, Microsoft connectors, language bindings.
- Conductor as pinned upstream Git submodule; only project-owned `conductor` context and tracks edited locally.
- Testing: cargo fmt/clippy/test, proptest, cargo-fuzz where practical, cargo-deny/audit and platform matrix; Python standard library only for initial docs validator.

## Optional reuse audit (2026-11-08)
Do not expand the three specialist Rust candidates merely to implement a roadmap. `foi-process` has a real Rust4PM adapter but currently omits attributes; only foi-process remains in the active owned-repository integration scope. Reuse via qualified consumer-owned contracts, never a reverse application dependency. No new runtime dependency is installed by this update. Draft 2020-12 validation via Python jsonschema is development-only; the stdlib planning checker is not a general JSON Schema validator.

## Mailweft and optional GraphRAG
The new repository name is accepted; preserve historical schema IDs. The core is a Rust email-to-OCEL converter. GraphRAG remains optional and has no selected mandatory library. First qualify deterministic graph projection/retrieval over the existing evidence records; Microsoft GraphRAG is a Python reference/comparator outside default production. FOI-O is consumer-owned ontology input/output, not a Mailweft runtime dependency. All source capture/archive activities stay outside Mailweft.
