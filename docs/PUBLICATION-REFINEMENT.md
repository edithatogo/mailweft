# Mailweft publication refinement — 8 November 2026

Source baseline: `e1c201b`. Accepted product name: Mailweft. Core outcome: an email-to-OCEL converter written in Rust. This revision preserves the original schema bytes and Conductor gitlink, and keeps the source history.

Changes: corpus-neutral core; Enron canonical verification/example role; explicit exclusion of the unvalidated FOI pipeline from all reference/oracle roles; one-way first-consumer integration; fyi-cli/archive ownership and documented handover conflict; evidence-linked FOI-O contribution/review loop; optional source-constrained GraphRAG design; exact task-DAG refinement and new scope-regression tests.

Planning counts: 53 requirements, 15 tracks, 116 tasks, 11 existing JSON schemas. Application implementation is not_started. Track 13 specifies bounded contracts/qualification, not a claimed GraphRAG implementation. Runtime extraction, provider benchmarks, FOI migration and ontology changes have not been executed.

The original seven scope tests were run before enforcement: one valid profile passed and six invalid profiles incorrectly passed the old validator (recorded RED). The updated validator rejects those profiles. Fresh verification is recorded in docs/BUILD_STATUS.md after checks.
