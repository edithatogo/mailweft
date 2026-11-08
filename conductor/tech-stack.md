# Technology stack

- Rust stable (version pin/edition chosen and recorded in T00.04; do not guess without qualification).
- Cargo workspace. Core dependency candidates: `mail-parser` (Stalwart), `mail-threading` (RFC references), `process_mining` (Rust4PM), serde/serde_json, hashing crates, clap and bounded file walk. Exact versions/licences pinned and audited by task.
- OCEL 2.0 canonical data structures and direct Rust import/export. Native JSON first, XML/SQLite qualified separately. No mandatory Python, JVM, WSL or CUDA.
- Optional features: EDRM XML adapter, Parquet bulk output, MIME attachment document parsing, local NLP/LLM backends, evidence graph, Microsoft connectors, language bindings.
- Conductor as pinned upstream Git submodule; only project-owned `conductor` context and tracks edited locally.
- Testing: cargo fmt/clippy/test, proptest, cargo-fuzz where practical, cargo-deny/audit and platform matrix; Python standard library only for initial docs validator.
