# Product
email-ocel is a reusable, independent Rust library and CLI converting raw RFC 5322/MIME emails into fully typed, standards-compliant OCEL 2.0 communication event logs, with optional provenance-bound NLP extraction of business events and objects. Rust4PM is the canonical OCEL engine and a useful consumer; foi-process is the only active owned-repository consumer. It may depend on this library; the library never depends on it.

Primary users: process-mining researchers, forensic analysts and administrative workflow engineers. First release: local EML/maildir to OCEL with source evidence, thread links, human-readable diagnostics and validation against synthetic and independently acquired processed datasets.

## Preserved core and optional differentiation
Raw-first, offline Rust communications-to-OCEL 2.0 stays the independently releasable product. Differentiation is evaluated through source-grounded structural/semantic reconstruction, dependent-evidence handling, incremental interpretation revisions, uncertainty-sensitive process results and adversarial/controlled benchmarks. Research cannot become a hidden baseline dependency. See `docs/FRONTIER-DESIGN.md`.
