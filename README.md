# Mailweft

**An email-to-OCEL converter written in Rust.**

Mailweft is a standalone, corpus-neutral library and CLI: raw RFC 5322/MIME email -> typed, evidence-linked OCEL 2.0 communication logs. Optional structural NLP, semantic enrichment and GraphRAG build on that output without replacing the converter or rewriting its observations.

**Status:** an experimental offline Rust EML-to-OCEL 2.0 implementation now builds, with single-file and bounded multi-EML batch conversion, synthetic tests and independent Rust4PM import acceptance. Its published roadmap remains much broader. No real dataset benchmark, semantic NLP, GraphRAG, live FOI migration or product release has been validated.

## Core contract
- Local EML input (maildir planned); no processed-data prerequisite. Preserve source hashes, actual participant roles, claimed versus observed times, duplicate/copy identity and completeness.
- Native OCEL 2.0 JSON with typed attributes, event/object relationship qualifiers and object histories; report every lossy or unsupported mapping.
- Offline Rust-only production core; no required Python, LLM, graph database, cloud, UI, FOI project or Enron corpus.
- Enron is a **canonical verification and example dataset**, not the subject or schema of Mailweft. Dataset-specific recipes stay outside the core; no real email data is committed.
- Original source stays immutable. Optional interpretations retain evidence, attribution, uncertainty, correction history and review state.

## Try the offline Rust converter

```sh
cargo run --locked -p mailweft-cli -- source.eml 2026-11-09T01:02:03Z > single.ocel.json
cargo run --locked -p mailweft-cli -- batch 2026-11-09T01:02:03Z a.eml b.eml > batch.ocel.json
cargo test --locked --workspace --all-targets
cargo test --locked -p mailweft-core --features rust4pm-validation --test rust4pm_interop
```

The required timestamp is a caller-supplied acquisition observation, not proof of sending or delivery. Public output uses hashed participant identifiers and explicit qualifier relationships. Batch is limited to 1,024 explicit EML paths and 256 MiB input; large-corpus streaming, full source receipts and many fields remain planned. [Implementation limitations](docs/BATCH-IMPLEMENTATION.md).

## First consumer and ownership
`foi-process` will depend on Mailweft. Its existing unvalidated output is **not** a reference or correctness oracle. Independently adjudicated source evidence and synthetic fixtures govern migration.

`fyi-cli` retains acquisition/capture; the `fyi-archive` boundary retains archival manifests/provenance/distribution (or a verified successor). `foi-process` maps generic results into a pinned FOI-O profile and sends evidence-linked ontology improvement candidates to `foi-o`. No automatic ontology changes or legal conclusions are inferred. See [FOI boundaries and handover ambiguity](docs/FOI-BOUNDARIES.md).

## Start here
- [Product scope](docs/PRODUCT-SCOPE.md), [contract](docs/CONTRACT.md), [requirements](docs/REQUIREMENTS.md), [design](docs/DESIGN.md)
- [Conductor index](conductor/index.md), [tracks](conductor/tracks.md), [task manifest](conductor/task-manifest.json), [execution order](docs/EXECUTION-ORDER.md)
- [GraphRAG design](docs/GRAPHRAG.md), [FOI and error-injection protocol](docs/FOI-SCOPE-AND-ROBUSTNESS.md)
- [Dataset verification](docs/DATASETS.md), [validation](docs/VALIDATION.md), [planning status](docs/BUILD_STATUS.md)

## Conductor
Gemini Conductor is a real Git submodule at `.agents/plugins/conductor`, pinned to commit `7a5c560a4fdf5297be58594cc37527eb12790272` (`conductor-v0.4.1`). Project context and task plans are already populated; do not run setup that overwrites them.

```sh
git clone --recurse-submodules https://github.com/edithatogo/mailweft.git
cd mailweft
python3 scripts/validate_scaffold.py
python3 -m unittest discover -s scripts -p 'test_*.py'
```

Python is only an initial planning-validation helper, not the proposed application runtime. Registering a host-specific Conductor extension is a separate optional action; the submodule does not prove it is installed in the user's agent host. See [naming and submodule details](docs/NAME-MIGRATION.md).

## Agent execution
Read `AGENTS.md` and the single eligible task card; implement only that task, write the specified failing tests first, record real exit codes, and leave unavailable execution blocked. The core lane is independently releasable; dataset rights, human annotation, research, graphs and downstream consumers are not core release prerequisites.

## Licence and references
Original work: Apache-2.0. Upstream dependencies and datasets retain separate licences and rights. `mail-parser` 1.0.1 is the qualified initial parser and Rust4PM `process_mining` 0.6.2 is pinned for an optional import-validation feature. `mail-threading` remains a future candidate. No upstream unlicensed implementation or email corpus is bundled. Existing version-1 schema identifiers retain the earlier email-ocel namespace as stable aliases.
