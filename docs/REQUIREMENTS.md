# Requirements — version 0.1

Priorities: **MUST** for Stage 1 release, **SHOULD** for next increment, **OPTIONAL** for research lanes. Every REQ is covered by a Conductor task and a test or reasoned deferral.

| ID | Priority | Requirement and unambiguous acceptance |
|---|---|---|
| R01 | MUST | Accept local raw .eml + maildir paths; no processed CSV or Parquet prerequisites. |
| R02 | MUST | Parse RFC5322/MIME headers, multipart text, addresses and source IDs; malformed input produces bounded, non-sensitive error. |
| R03 | MUST | Preserve raw hash, source copy/custodian and independently distinguish `header_date`, observed ingestion time and source receipt time. |
| R04 | MUST | Deterministically represent sender/to/cc/bcc, message, source copy, thread, attachment metadata as typed objects with role-qualified E2O edges. |
| R05 | MUST | Thread by standards headers when available; fallback heuristics produce separate lower-confidence links, never 'observed' responses. |
| R06 | MUST | Exact/near duplicate controls across custodians; Message-ID collisions/missing IDs handled, with no silent dropping. |
| R07 | MUST | Output loss-tested OCEL 2.0 JSON and native interchange as available; qualified E2O/O2O and object attribute changes preserved. |
| R08 | MUST | Independently import the result with Rust4PM; run a baseline object-centric algorithm with deterministic, documented outputs. |
| R09 | MUST | Manifest counts, schema version, checksum, source completeness, rejects, duplicate counts and immutable provenance in machine-readable outputs. |
| R10 | MUST | Corpus files, sensitive outputs, embeddings and licence-uncertain source code excluded from Git and CI artifacts. |
| R11 | MUST | Synthetic fixtures and property/fuzz regression for malformed, huge, recursive MIME, dangerous content, clock issues, uncommon legacy addresses and disputed thread links. |
| R12 | MUST | Single-command offline test, format, lint, audit and deterministic fixture generation on supported Rust targets. |
| R13 | MUST | Enron source manifest and rights/personal-information risk ledger, with no downloads or redistributions without permitted source policy. |
| R14 | MUST | Independent matchability study between raw corpus and processed Enron OCEL; no false record-level equivalence. |
| R15 | MUST | Conductor as pinned Git submodule and checked-in context/tracks with exact per-task acceptance. |
| R16 | SHOULD | Extend to optional MBOX, RFC822 forwarded messages, HTML-only bodies and attachments with safe decoding. |
| R17 | SHOULD | Support EDRM XML manifests/attachment companions as optional adapter, not core raw parsing prerequisite. |
| R18 | SHOULD | Configurable pseudonymisation and protected identity vault, purpose-limited access and deletion. |
| R19 | SHOULD | Normalise old Exchange/X.500/IMCEA legacy sender forms, retaining unresolved cases and source evidence. |
| R20 | SHOULD | Add Stage 2 email text segmentation, quote attribution, attachment content extraction in isolated sandbox, classification/redaction before inference. |
| R21 | SHOULD | Extract task, commitment, request, deadline, decision communication, negation, modality, entity references with source spans and abstention. |
| R22 | SHOULD | Compare deterministic rules, conventional NLP, optional local small LLMs with gold adjudication; zero required network egress. |
| R23 | SHOULD | Cross-source entity links and temporal evidence graph behind optional provider; graph/GraphRAG never authoritative. |
| R24 | SHOULD | Benchmark scaling on 10k/100k/full matched corpora, import/export fidelity, memory, time and failure rates. |
| R25 | OPTIONAL | Python/R/Julia bindings, Power BI/Fabric integration, Outlook/Graph connector, CareOps adapter through versioned independent contracts. |
| R26 | OPTIONAL | Rust4PM advanced OC algorithms, graph-assisted retrieval, uncertainty-aware conformance and simulation interoperability. |

## Non-functional targets
- Minimal production dependencies: qualify `mail-parser`, `mail-threading`, `process_mining` and required serialization/CLI crates only; avoid custom parsers where robust upstream exists.
- No unbounded reads: stream at file/record boundaries, configurable max message and attachment sizes, no remote fetching.
- Idempotent replay: same source bytes and mapping version yield same logical IDs and event payloads; manifest generation timestamp is excluded from comparison.
- Security gates: fail closed on unknown scope/rights in production ingest; test fixtures can declare `synthetic`.
- Quality: Rust `fmt`, `clippy -D warnings`, tests, cargo audit/deny and fixture reproducibility; code coverage target to be qualified (do not invent thresholds from an empty project).
