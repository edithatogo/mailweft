# Conductor — Mailweft

Core outcome: **an email-to-OCEL converter written in Rust**. Enron is canonical verification/example data only. foi-process is the first consumer and its old outputs are not validation references.

Read [product](product.md), [scope contract](scope-contract.json), [guidelines](product-guidelines.md), [tech stack](tech-stack.md), [workflow](workflow.md), [execution order](../docs/EXECUTION-ORDER.md), then exactly one eligible task from [tracks](tracks.md).

Current planning: **53 requirements, 15 tracks, 116 tasks**. No Rust application or graph/model benchmark is implemented. All application tasks remain not_started. First task: **T00.01** (qualify current upstream dependencies and licences). T00.04 later creates the real Rust workspace; do not claim Cargo checks ran before it exists.

- [Product scope](../docs/PRODUCT-SCOPE.md)
- [FOI boundaries and ontology feedback](../docs/FOI-BOUNDARIES.md)
- [Optional GraphRAG](../docs/GRAPHRAG.md)
- [Robustness and error injection](../docs/FOI-SCOPE-AND-ROBUSTNESS.md)

Conductor is a real gitlink at `.agents/plugins/conductor`, pinned to `7a5c560a4fdf5297be58594cc37527eb12790272`. Project setup is complete as repository context. Host extension registration and upstream plugin execution are separate and unclaimed. Never rerun setup over the supplied context without review.
