# Mailweft

**An email-to-OCEL converter written in Rust.**

Mailweft converts supplied raw RFC 5322/MIME email into portable OCEL 2.0 communication logs. Source observations remain distinct from header claims, inferred relationships and later semantic interpretations. Enron is verification/example data only, never a required input or product schema.

**Status: experimental Rust 0.7.0.** Implemented: single/batch EML conversion, observable participant roles, claimed header chronology, MIME metadata, conservative reply candidates, source-integrity receipts, selection-level accepted/rejected reconciliation, non-destructive zoning and bounded base64/quoted-printable source-unit maps. No real-corpus, semantic NLP, GraphRAG or live FOI validation is claimed.

## Convert and inspect

```sh
cargo run --locked -p mailweft-cli -- source.eml 2026-11-09T07:00:00Z > source.ocel.json
cargo run --locked -p mailweft-cli -- batch 2026-11-09T07:00:00Z a.eml b.eml > batch.ocel.json
cargo run --locked -p mailweft-cli -- inspect source.eml > body-evidence.json
cargo run --locked -p mailweft-cli -- convert-evidence source.eml source-envelope.json > evidence-bundle.json
cargo run --locked -p mailweft-cli -- reconcile selection.json > reconciliation.json
```

Single/batch modes emit bare OCEL. `convert-evidence` emits `{conversion, receipt}`, with OCEL at `conversion.log`. `reconcile` emits a manifest and optional `log`; partial input produces exit 2 and withholds OCEL unless `--allow-partial` is explicitly supplied. It accounts for supplied files, not all potentially relevant correspondence.

[Source envelopes](docs/SOURCE-ENVELOPE.md) verify byte matching, not sender authenticity, authorization or completeness. [Selection reconciliation](docs/CORPUS-RECONCILIATION.md) records every accepted/rejected slot. [Transfer provenance](docs/TRANSFER-PROVENANCE.md) records verified source-unit relations without inventing exact character offsets. Hashes are pseudonymous and require access controls.

## Conductor — configured repository, explicit host boundary

```sh
git clone --recurse-submodules https://github.com/edithatogo/mailweft.git
cd mailweft
python3 scripts/conductor_doctor.py --check
python3 scripts/conductor_doctor.py --status
```

Conductor v0.4.1 is pinned as a real submodule at `.agents/plugins/conductor` (`7a5c560a4fdf5297be58594cc37527eb12790272`). All track indexes, metadata, plans and task statuses are reconciled. The [canonical task manifest](conductor/task-manifest.json) owns status and exact prerequisites; `conductor_doctor.py --sync` updates derived context without marking work complete. CI rejects stale context. Do not rerun generic setup over this existing project.

See [Conductor setup](docs/CONDUCTOR-SETUP.md) for optional Gemini CLI linking on a development host. Repository checks do not claim the extension was registered on the user's computer. Root [GEMINI.md](GEMINI.md) and [AGENTS.md](AGENTS.md) direct agents to the existing [project index](conductor/index.md) and [next eligible tasks](docs/EXECUTION-ORDER.md).

Current task accounting: **14 completed, 12 in progress, 90 not started**. Partial implementations retain explicit remaining acceptance. Optional research/graphs and downstream work cannot become core-release prerequisites.

## Test

```sh
cargo test --locked --workspace --all-targets --no-fail-fast
cargo test --locked -p mailweft-core --features rust4pm-validation --test rust4pm_interop
cargo fmt --all -- --check
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
python3 -m pip install -r requirements-planning.txt
python3 scripts/validate_scaffold.py
python3 -m unittest discover -s scripts -p 'test_*.py'
cargo build --locked -p mailweft-cli
python3 scripts/validate_runtime_contracts.py
python3 scripts/validate_selection_outputs.py
```

Rust 1.99.1 is pinned in rust-toolchain.toml; Cargo.lock fixes dependency resolution. Python is development-only validation, not the runtime. [Build evidence](docs/BUILD_STATUS.md), [0.7 migration](docs/MIGRATION-0.7.md), [qualification record](docs/VERIFICATION-0.7.md).

## Boundaries and remaining work

Offline Rust core; no required language model, graph store, cloud, UI, FOI module or Enron corpus. Source capture/preservation remain external. `foi-process` is the first consumer and must validate against independent source annotations, not its unvalidated old output. `fyi-cli` captures; the verified archive boundary preserves; `foi-o` reviews ontology proposals. No legal inference or automatic ontology promotion.

Input is bounded and explicit, not a streaming mailbox crawler or race-resistant filesystem sandbox. Generic charset/HTML source maps, full typed OCEL/format roundtrips, broader identity/threading, security advisory/fuzz gates, NLP/GraphRAG and live FOI integration remain tracked. See [contract](docs/CONTRACT.md), [design](docs/DESIGN.md), [FOI boundaries](docs/FOI-BOUNDARIES.md) and [GraphRAG design](docs/GRAPHRAG.md).

## Licence

Original work Apache-2.0. mail-parser 1.0.1 and optional Rust4PM process_mining 0.6.2 retain upstream licences. [Resolved direct dependency inventory](docs/DEPENDENCIES.md) includes exact versions, declared SPDX expressions and available licence-file hashes. No real email corpus or unlicensed upstream implementation is bundled.
