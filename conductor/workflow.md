# Mailweft development workflow

1. Read the project index, AGENTS.md, scope contract and the selected track index/spec/plan. Use `python3 scripts/conductor_doctor.py --status`; select an eligible task from the canonical manifest. Task dependencies are exact IDs, never whole-track approval gates.
2. Work on an isolated feature branch. Record significant interface/scope rulings in the execution ledger; preserve raw-source/copy/claim distinctions, FOI ownership and optional-lane isolation.
3. Write a focused failing test, execute it and inspect the failure. Implement the smallest coherent change, then run the targeted tests. Fictional fixtures only; missing tool/data access is BLOCKED, not PASSED.
4. Run locked Rust tests, independent Rust4PM import tests, actual-output schema validation, formatting, all-feature Clippy, planning tests and the Conductor doctor. Failed checks prevent acceptance.
5. Update only the tasks whose acceptance was actually met. For partial work use `in_progress`, with precise implemented and remaining statements. Store file/test/CI evidence in the task manifest. Preserve requirement IDs; amend interface paths to actual code rather than duplicating modules solely to match obsolete proposed names.
6. Run `python3 scripts/conductor_doctor.py --sync`, inspect changes, then `--check`. This updates plan checkboxes, metadata, indexes, evidence summaries and next-task hints. Synchronization never changes task status.
7. Review the full diff and residual risks. Push the feature branch, require exact-head CI, merge only with authorization, and verify post-merge CI. No force push or branch protection bypass.

Conductor's upstream plugin is read-only and pinned. The project is already configured: generic setup must not overwrite these files. Host registration is separate and unverified here. Human approval is reserved for real-data rights/access, external effects outside the requested work, destructive operations and ontology/legal promotion—not repeated permission to perform already approved implementation tasks.
