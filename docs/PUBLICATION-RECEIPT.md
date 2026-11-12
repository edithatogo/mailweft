# Mailweft publication receipt — 8 November 2026

## Published source and preserved history

- Repository: https://github.com/edithatogo/mailweft
- Original planning head: `5adfc5a777040fe33c64c4fb860f8fd071c2e790`.
- Original source tree: `8b99bbfb69b3e0847f524a202cae6af14c7051e1`.
- Clean-host qualification commit: `76d2cf38ce8e7dd97f38c2b70d849f4a2d487f6d`.
- Qualified source tree: `56af9e4fb410d284b39cbcf08c3fcda611653879`.
- History-preserving publication merge: `0217e88be00d475b065612b136798eef5db9d0ad`.
- Bootstrap cleanup completed through `abeafe73c674473f4db7a93c653d87f591af3f5d`; transport files and one-time write-enabled workflows are absent from the current source tree. Historical bootstrap commits remain as provenance.

The seven original planning commits remain ancestors with their original identities. Import validated all transport-part SHA-256 hashes, decompressed history hash, source head/tree identities, Git integrity and the exact qualification patch/tree before a non-forced publication.

## Hosted evidence

The successful import run is [42475907377](https://github.com/edithatogo/mailweft/actions/runs/42475907377), job `128717115328`. Its recorded steps all succeeded: transport/history integrity, actual pinned Conductor checkout, planning/schema verification and remote publication readback.

The pinned Conductor source is a real mode-160000 Git submodule at `.agents/plugins/conductor`, commit `7a5c560a4fdf5297be58594cc37527eb12790272`. The hosted runner initialized and verified it. This is not evidence of installing or registering a host-specific Gemini/agent extension on the user's computer.

Checks executed before publication:

- 53 requirements, 15 tracks, 116 tasks and 38 detailed task cards pass scaffold/DAG/scope validation.
- 17 planning-validator tests pass.
- 11 Draft 2020-12 schemas validate; four fictional positive examples are accepted and 18 negative mutations rejected.
- Git whitespace and clean-worktree checks pass.

The first hosted attempt, run `42475424694`, correctly prevented publication because an optional date-time validator dependency was missing. The correction pins `rfc3339-validator` and `six` and adds a fail-closed checker guard with two regression tests. The failure is retained as evidence, not relabelled as a successful initial run.

The permanent `.github/workflows/planning-checks.yml` uses read-only repository permissions, an immutable checkout action and the same pinned planning dependencies. Its exact-head result must be read from Actions for each subsequent commit; this receipt does not predict future checks.

## Partner coordination

See [PARTNER-HANDOFFS.md](PARTNER-HANDOFFS.md): Mailweft #2, foi-process #153, foi-o #195, and comments on the existing fyi-cli #309 and fyi-archive #378 programmes. Partner responsibilities and ontology changes are requested, not implemented or certified by opening issues.

## Evidence boundary

Mailweft's core outcome remains **an email-to-OCEL converter written in Rust**. This publication delivers contracts, plans, fictional examples and documentation-validation tooling. No Rust application, live-data ingestion, extraction benchmark, NLP model, GraphRAG runtime, FOI migration or ontology promotion has been implemented or validated. Enron is a verification/example corpus only. Existing unvalidated FOI pipeline outputs are not correctness references. No real email corpus or private correspondence was published.
