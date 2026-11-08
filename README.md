# Mailweft

**An email-to-OCEL converter written in Rust.**

Mailweft is a standalone, corpus-neutral Rust library and CLI. It converts supplied raw RFC 5322/MIME messages into evidence-linked OCEL 2.0 communication logs using Rust4PM's full native data structures. Enron is a verification/example corpus, not a required input or the product domain.

## Current implementation checkpoint

The first native slice is on `feat/raw-email-ocel` / [PR #3](https://github.com/edithatogo/mailweft/pull/3). It includes bounded EML parsing, From/Sender/To/Cc/Bcc/Reply-To roles, explicit observation and header-asserted chronology, scoped keyed pseudonyms, exact-copy lineage, MIME attachment metadata, JSON OCEL projection and a local-only CLI.

**Development preview, not a production release.** The dependency advisory gate is blocked by Rust4PM's transitive quick-xml dependency: [issue #4](https://github.com/edithatogo/mailweft/issues/4). Native tests passing does not waive that gate. No real corpus, model, FOI migration or GraphRAG runtime has been evaluated. See [implemented scope and limitations](docs/CORE-CHECKPOINT.md) and [execution evidence](conductor/execution/20261108-core-slice.md).

## Try a fictional example

The current implementation requires the pinned Rust toolchain and registry access for its first dependency build. Conversion itself needs no network. The commands below target the implementation branch while the PR is unmerged:

```sh
git clone --branch feat/raw-email-ocel --recurse-submodules https://github.com/edithatogo/mailweft.git
cd mailweft
cargo test --workspace --all-targets --locked

# Unix example: keep this private key outside the repository.
(umask 077; openssl rand 32 > /tmp/mailweft-example.key)
cargo run --locked -p mailweft-cli -- convert \
  --synthetic --scope fictional-example \
  --key-file /tmp/mailweft-example.key \
  --observed-at 2026-11-08T00:00:00Z --view both \
  fixtures/synthetic/basic.eml
```

The default output bundles `ocel`, pseudonymous message evidence and a receipt. `--ocel-only` emits standalone OCEL JSON. The CLI currently accepts individual regular files, not directories or archives. `--observed-at` is explicitly supplied source-observation metadata, never a guessed sent or delivery date. Choose `observed`, `header-asserted` or `both` deliberately; these are distinct clocks.

`--synthetic` declares fictional fixtures. Independently authorised real local processing uses `--authorised-local`; the flag does not obtain consent or organisational approval. Output is pseudonymous, not anonymous or automatically publishable. Use a trusted, quiescent local source directory. See the limitations before processing any non-fictional material.

## Core contract

- Offline Rust-only production code; no required Python, LLM, graph database, cloud, UI, FOI project or Enron corpus.
- Raw source stays unchanged; source hashes and observed/claimed times remain distinct.
- Header recipients are not task assignees; an address is not a verified person; missing Bcc is not proof of no blind recipients.
- Native OCEL event/object attributes and qualified relationships are retained. Unimplemented/lossy paths are rejected or explicitly diagnosed.
- Optional interpretations cannot overwrite observations. Text/attachment semantic extraction, GraphRAG and ontology feedback remain separate future work.

## First consumer and ownership

`foi-process` will depend on Mailweft. Its existing unvalidated output is not a correctness reference, teacher or migration oracle. Independent source annotation and fictional fixtures govern acceptance.

`fyi-cli` retains acquisition; the `fyi-archive` boundary retains preservation/manifests/distribution, or an explicitly verified successor. `foi-process` owns pinned FOI mappings and sends reviewed evidence-backed candidate improvements to `foi-o`. See [FOI boundaries](docs/FOI-BOUNDARIES.md). No partner runtime was changed in this slice.

## Conductor and development

- [Contract](docs/CONTRACT.md), [requirements](docs/REQUIREMENTS.md), [design](docs/DESIGN.md)
- [Conductor index](conductor/index.md), [tracks](conductor/tracks.md), [task manifest](conductor/task-manifest.json), [execution order](docs/EXECUTION-ORDER.md)
- [Dependency qualification](docs/DEPENDENCIES.md), [GraphRAG design](docs/GRAPHRAG.md), [FOI/error-injection protocol](docs/FOI-SCOPE-AND-ROBUSTNESS.md)

Gemini Conductor is a real submodule at `.agents/plugins/conductor`, pinned to `7a5c560a4fdf5297be58594cc37527eb12790272`. Existing context/plans must not be overwritten by setup. Host-specific extension registration is separate.

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
python3 scripts/validate_scaffold.py
python3 -m unittest discover -s scripts -p 'test_*.py'
python3 scripts/validate_contract_examples.py
```

Python checks are development-only contract tooling, not an application runtime. CI checks the real CLI output against independent evidence schemas and tests Linux, macOS and Windows. Read exact-head CI results rather than assuming every branch is green.

Original work is Apache-2.0. Upstream software/data retain their rights. No real correspondence, dataset archive, private key or unlicensed source implementation is bundled. Existing v1 schema identifiers retain their stable email-ocel aliases.
