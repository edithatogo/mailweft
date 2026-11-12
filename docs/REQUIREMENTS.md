# Requirements — version 0.2 (additive plan refinement)

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
| R25 | OPTIONAL | Python/R/Julia bindings, optional platform connectors and generic consumer exports use versioned independent contracts; foi-process is the only active owned-repository adapter. |
| R26 | OPTIONAL | Rust4PM advanced OC algorithms, graph-assisted retrieval, uncertainty-aware conformance and simulation interoperability. |
| R27 | SHOULD | Model header participants, in-text addressees and task actors as three separate relationship classes; preserve Bcc uncertainty and privacy restrictions, and support Resent headers. |
| R28 | SHOULD | Segment decoded email parts into non-destructive, source-anchored functional zones, with original/semantic/identity/context views and nested quote provenance. |
| R29 | SHOULD | Extract salutations, sign-offs and signature contact/organisational hints without treating them as authoritative identities or deleting their source content. |
| R30 | SHOULD | Apply multi-label utterance-level dialogue/speech acts using historical taxonomies; identify likely actors, reported actions, temporality and negation, preserving abstention. |
| R31 | SHOULD | Benchmark functional zoning, identity linking and task-to-person mapping using permitted, independently annotated corpora (Quagga, Forge, EPA) and synthetic fixtures with no leakage. |
| R32 | MUST | Core release is independent of NLP, attachments content conversion, real-corpus access, graph retrieval, research, UI and sibling repositories; validate dependency closure. |
| R33 | MUST | Keep mailbox/address/group, human identity, header role, text addressee and action responsibility distinct; never infer verified person/authority from address or signature alone. |
| R34 | SHOULD | Classify message genre, body zone, utterance act and conversation interaction as separate versioned vocabularies with explicit unknown/multi-label outputs. |
| R35 | SHOULD | Every exported semantic assertion has a verifiable evidence receipt linking exact source/rendition spans, extractor configuration, attribution and projection decision. |
| R36 | SHOULD | Represent request-response, acknowledgement, commitment, delegation, withdrawal and completion-reported interactions without inferring actual obligation fulfilment or authority. |
| R37 | SHOULD | Keep alternative actor/work-item links and temporal intervals/partial orders; no arbitrary timestamp or most-likely entity is silently made an observed fact. |
| R38 | SHOULD | Append-only interpretation revisions support supersession, retraction, deterministic replay and invalidation of affected derived outputs; raw observations remain immutable within retention policy. |
| R39 | SHOULD | Copied/quoted/forwarded evidence shares dependence lineage; repeated copies or agreeing models do not count as independent corroboration. |
| R40 | SHOULD | Support bounded rules-to-statistical-to-local-model extraction with calibrated abstention, measured risk/coverage, budgets and model/runtime provenance; scores are not assumed probabilities. |
| R41 | MUST | For enabled semantic profiles, metamorphic tests change To/Cc, actor mention, quotation, tense and negation one at a time, asserting expected changes and invariants rather than guessing global accuracy. |
| R42 | SHOULD | Evaluate extraction errors through downstream object/event/edge distortion and process-conclusion sensitivity, not just token F1 or overall accuracy. |
| R43 | SHOULD | Enforce evidence-scoped retrieval, Bcc view restrictions and deletion propagation into indexes/summaries/derived artifacts; preserve coverage-qualified absence claims. |
| R44 | OPTIONAL | Research comparisons include 2026 text-to-OCEL collector/refiner work, object-centric quality classifications and matched-cost ablations; no frontier superiority claim without evidence. |
| R45 | OPTIONAL | The sole active owned-repository consumer is foi-process, which may depend on this library through a separately qualified one-way adapter; no reverse dependency, FOI domain leakage or mandatory Python runtime. |
| R46 | SHOULD | Use bounded incremental processing, content/interpretation cache versioning and impact-based recomputation; optimization must preserve canonical equivalence. |
| R47 | MUST | Task manifest uses exact existing task IDs, acyclic dependencies, requirement coverage and checked test commands; no missing external capability can be reported as passed. |


## Non-functional targets
- Minimal production dependencies: qualify `mail-parser`, `mail-threading`, `process_mining` and required serialization/CLI crates only; avoid custom parsers where robust upstream exists.
- No unbounded reads: stream at file/record boundaries, configurable max message and attachment sizes, no remote fetching.
- Idempotent replay: same source bytes and mapping version yield same logical IDs and event payloads; manifest generation timestamp is excluded from comparison.
- Security gates: fail closed on unknown scope/rights in production ingest; test fixtures can declare `synthetic`.
- Quality: Rust `fmt`, `clippy -D warnings`, tests, cargo audit/deny and fixture reproducibility; code coverage target to be qualified (do not invent thresholds from an empty project).

See [structural NLP design and benchmark bibliography](STRUCTURAL-EMAIL-NLP.md) for R27–R31.

## Mailweft product, consumer and retrieval refinements

| ID | Priority | Requirement and unambiguous acceptance |
|---|---|---|
| R48 | MUST | Product remains a generic Rust raw-email-to-OCEL converter. Enron is verification/example only; core release works offline on non-Enron fictional domains without any corpus access. |
| R49 | MUST | No existing foi-process output is a validation reference, oracle, teacher or behavioral-parity target. Source-independent annotations and synthetic expectations govern acceptance; old code is diagnostic-only. |
| R50 | OPTIONAL | First consumer depends on Mailweft, never the reverse; capture/archive ownership remains fyi-cli/fyi-archive or a verified successor. Preserve raw-versus-rendered fidelity and explicit handover evidence. |
| R51 | OPTIONAL | Validated FOI results emit evidence-linked FOI-O contribution packets with base version, positive/negative examples, uncertainty and review state. No automatic ontology or legal-profile promotion. |
| R52 | OPTIONAL | GraphRAG supports bounded source retrieval and optional generation only; default conversion works without it and Stage 1 output remains unchanged under optional processing or failure. |
| R53 | OPTIONAL | Graph retrieval enforces scope/as-of boundaries before traversal and output, propagates retractions/access changes through caches, distinguishes dependent evidence and measures unsupported answers at matched coverage. |
