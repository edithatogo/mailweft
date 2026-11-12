# Email-OCEL refinement — 8 November 2026

## Delivered in this revision

The raw-email-to-OCEL 2.0 contract is preserved. The project remains a standalone, local/offline Rust library design, with optional non-destructive structural and semantic enrichment. No CareOps, Python, model, cloud, graph or UI dependency is added to the application core.

The plan now contains **47 requirements, 14 tracks, 108 implementation tasks, 30 new detailed task cards and 11 JSON Schema contracts**. All application implementation tasks remain `not_started`. The delivered changes are specifications, contracts, fictional examples and development-only validation tooling.

### Concepts incorporated

Header participant roles remain separate from textual addressees, claimed organisational roles and requested task actors. Mailbox/address objects are not automatically people. Bcc visibility is source-copy-specific. Four independent vocabularies cover message genre, functional text zone, utterance act and conversation interaction. Signatures and quoted history are masked by analysis view rather than deleted.

The optional interaction layer links requests, clarifications, commitments, delegations, withdrawals and reported completions across messages and work items. It never treats a thread as one task or communication as proof of valid authority or actual completion.

### Stronger research direction

A 2026 study already extracts OCEL from textual descriptions using collector/refiner variants. The project therefore does not claim the basic text-to-OCEL idea as novel. Its proposed differentiation is evidence-grounded, uncertainty-aware, incremental reconstruction from imperfect raw communications, evaluated against strong existing methods.

New experiments address source anchoring; alternative actors and event times; quotation/copy dependence; calibrated abstention; inexpensive local inference cascades; correction/retraction replay; controlled perturbations; and the effect of extraction errors on process conclusions. GraphRAG remains optional and must demonstrate value over simpler retrieval. A synthetic workflow generator must retain an independent latent event ledger rather than treating generated prose as truth.

### Repository reuse findings

- `foi-process` is the strongest implementation precedent: Rust evidence revisions/retractions, replay and a real Rust4PM adapter. Its current adapter deliberately exports empty attribute vectors, so it cannot be copied unchanged as an attribute-complete exporter. That is an adapter limitation, not a Rust4PM data-model limitation.
- `fe-reader` and `citeweft` are optional document-evidence candidates with different scopes and preview limitations.
- `sourceright` and `searchright` provide useful verification, review, provenance and retrieval conventions; their scholarly semantics must not become email-domain assumptions.
- `nlp-policy-nz` is Python-based with Rust acceleration; use only as an optional comparator/adapter, never in the Rust-only default runtime.
- `rcagent` is a downstream investigation consumer/methodology reference, not a Rust extraction engine. `kairos` can support independently validated synthetic workflows.
- `process-mappings` currently declares itself a non-canonical scaffold without substantive mappings. Owned Rust4PM/Propel forks were discovered but their deltas were not audited.

Source files and inspected blob hashes are recorded in `repository-reuse-register.json`; primary literature and standards are in `source-register.json` and `RELATED-WORK.md`.

### Execution repair

Exact task-ID dependencies replace shorthand track dependencies. Safe body/zoning work no longer waits for full Enron benchmarking. Source maps precede zoning. Core release does not wait for corpus rights, model experiments, attachment conversion, external adapters or graphs. Documentation-only validators check the task DAG, requirement references, task registration and independent core dependency closure.

The 30 new task cards name the owned function or artifact, allowed paths, prerequisites, two test oracles and verification commands. Legacy tasks retain their existing acceptance criteria and need exact assertions bound to implemented predecessor interfaces before execution. Planned Cargo commands are not evidence that a crate, function or test already exists.

## Verification and limits

See `BUILD_STATUS.md` for actual checks. The original seven schema files are unchanged; four additive record types have positive and negative fictional examples. Schema checks do not prove evidence truth, cross-record identity, inference calibration or scientific performance.

The Conductor Git submodule remains pinned at the inherited commit. Its source checkout and host extension are not initialized/installed by this revision. No raw email corpus was downloaded, no Rust application was implemented or compiled, no model benchmark ran, and no remote GitHub repository was changed.

## Superseded integration scope
The later user instruction restricts active owned-repository integration to `foi-process`. Earlier candidate descriptions above are historical. Current tasks and research coupling are corrected in `FOI-SCOPE-AND-ROBUSTNESS.md`; no unrelated adapter is active.
