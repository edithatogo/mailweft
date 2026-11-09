# Mailweft

**An email-to-OCEL converter written in Rust.**

Mailweft is a standalone, corpus-neutral library and CLI: supplied raw RFC 5322/MIME email becomes typed OCEL 2.0 communication logs. Optional source inspection and later NLP/GraphRAG do not replace the converter or rewrite its observations.

**Status: experimental Rust 0.6.0.** Implemented: single-file and bounded batch conversion; participant qualifiers; claimed Date/Message-ID/reply references; MIME attachment metadata; conservative reply candidates; source-envelope byte verification and receipts; non-destructive plaintext zones with source offsets and explicit origin/skip diagnostics. No real-corpus benchmark, semantic NLP, GraphRAG, live FOI migration or product release is certified.

## Try it

```sh
# Existing modes continue to produce bare OCEL 2.0 JSON.
cargo run --locked -p mailweft-cli -- source.eml 2026-11-09T07:00:00Z > source.ocel.json
cargo run --locked -p mailweft-cli -- batch 2026-11-09T07:00:00Z a.eml b.eml > batch.ocel.json

# Separate source inspection and integrity-bound conversion.
cargo run --locked -p mailweft-cli -- inspect source.eml > body-evidence.json
cargo run --locked -p mailweft-cli -- convert-evidence source.eml source-envelope.json > evidence-bundle.json

cargo test --locked --workspace --all-targets
cargo test --locked -p mailweft-core --features rust4pm-validation --test rust4pm_interop
```

`convert-evidence` requires a caller-supplied envelope matching the actual bytes. It returns `{conversion, receipt}`; the OCEL is `conversion.log`. Receipt assurance is **byte integrity**, not sender authenticity, trusted time, access authorization or proven source completeness. See [source-envelope contract](docs/SOURCE-ENVELOPE.md) and [0.6 migration](docs/MIGRATION-0.6.md).

## Core boundaries

- Local EML input; no processed dataset prerequisite or automatic network acquisition. Enron is a canonical **verification/example corpus**, never required input or the library's domain.
- Offline Rust runtime: no required Python, language model, graph database, Microsoft tenant, UI or FOI module. Python scripts are development-only contract checks.
- Distinguish observed source copies, claimed header times, address roles and unverified reply candidates. To/Cc does not assign tasks; a Date or Message-ID does not prove delivery.
- Input reads are counted and bounded: 16 MiB per EML, 64 KiB per metadata envelope; batch 1,024 files/256 MiB actual input. Visible final-component symlinks and non-regular files are rejected. Hostile directory races/root confinement remain unqualified.
- Content remains external and immutable. OCEL/receipts/zone sidecars exclude bodies and private source locators, but hashes and metadata are **pseudonymous, not anonymous**. Keep access controls.
- Plaintext zones preserve every accepted line; transformed offsets remain `derived`, and omitted parts have diagnostics. Labels/origin are rule hypotheses, not verified authorship or business acts.

See [batch limits](docs/BATCH-IMPLEMENTATION.md), [header claims](docs/CLAIMED-HEADER-CHRONOLOGY.md), [attachment metadata](docs/ATTACHMENT-METADATA.md), [source zoning](docs/SOURCE-PROVENANCE-AND-ZONING.md) and [thread candidates](docs/THREAD-CANDIDATES.md). Earlier dated specifications describe their implementation versions; the 0.6 migration governs new receipts/zoning.

## First consumer

`foi-process` will depend on Mailweft. Its existing unvalidated output is **not** an oracle, teacher or analytical equality target. Independent source annotations and synthetic fixtures govern acceptance.

`fyi-cli` captures; the `fyi-archive` boundary or its explicitly verified successor preserves sources/manifests. `foi-process` applies versioned FOI mappings and contributes reviewed evidence to `foi-o`. Mailweft performs no legal interpretation or automatic ontology promotion. See [FOI boundaries](docs/FOI-BOUNDARIES.md).

## Conductor and execution

Gemini Conductor is a real pinned Git submodule at `.agents/plugins/conductor`, commit `7a5c560a4fdf5297be58594cc37527eb12790272`. Do not overwrite project context by rerunning setup. Host-specific extension registration is separate.

```sh
git clone --recurse-submodules https://github.com/edithatogo/mailweft.git
cd mailweft
python3 scripts/validate_scaffold.py
python3 -m unittest discover -s scripts -p 'test_*.py'
```

Start with [AGENTS.md](AGENTS.md), [product contract](docs/CONTRACT.md), [requirements](docs/REQUIREMENTS.md), [design](docs/DESIGN.md), [track registry](conductor/tracks.md), [task manifest](conductor/task-manifest.json), and [execution order](docs/EXECUTION-ORDER.md). The [current execution ledger](conductor/remaining-priorities-20261109.md) records partial task coverage and remaining source/segmentation work. Working slices do not mark whole tracks complete. Optional research, GraphRAG and downstream integration cannot block a core release.

## Licence and qualification

Original work: Apache-2.0. `mail-parser` 1.0.1 is pinned; Rust4PM `process_mining` 0.6.2 is an optional independent validation provider. Cargo.lock pins dependency resolution. No new production dependency was added for receipts/zoning. No real email corpus or unlicensed implementation is bundled. Existing schema IDs remain stable; new envelope/receipt/body schemas are additive. See [dataset rights](docs/DATASETS.md), [validation](docs/VALIDATION.md) and [GraphRAG design](docs/GRAPHRAG.md).
