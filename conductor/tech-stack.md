# Technology stack

- Rust stable (version pin/edition chosen and recorded in T00.04; do not guess without qualification).
- Cargo workspace. Core dependency candidates: `mail-parser` (Stalwart), `mail-threading` (RFC references), `process_mining` (Rust4PM), serde/serde_json, hashing crates, clap and bounded file walk. Exact versions/licences pinned and audited by task.
- OCEL 2.0 canonical data structures and direct Rust import/export. Native JSON first, XML/SQLite qualified separately. No mandatory Python, JVM, WSL or CUDA.
- Optional features: EDRM XML adapter, Parquet bulk output, MIME attachment document parsing, local NLP/LLM backends, evidence graph, Microsoft connectors, language bindings.
- Conductor as pinned upstream Git submodule; only project-owned `conductor` context and tracks edited locally.
- Testing: cargo fmt/clippy/test, proptest, cargo-fuzz where practical, cargo-deny/audit and platform matrix; Python standard library only for initial docs validator.

## Optional reuse audit (2026-11-08)
Do not expand the three specialist Rust candidates merely to implement a roadmap. `foi-process` has a real Rust4PM adapter but currently omits attributes; `nlp-policy-nz` has a Python runtime; `fe-reader`/CiteWeft are previews with bounded capabilities. Reuse via qualified optional contracts. No new runtime dependency is installed by this update. Draft 2020-12 validation via Python jsonschema is development-only; the stdlib planning checker is not a general JSON Schema validator.
