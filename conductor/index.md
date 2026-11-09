# Conductor — Mailweft

**An email-to-OCEL converter written in Rust.** Experimental Rust implementation: 0.7.0.

- [Product Definition](./product.md)
- [Product Guidelines](./product-guidelines.md)
- [Tech Stack](./tech-stack.md)
- [Workflow](./workflow.md)
- [Tracks Registry](./tracks.md)
- [Tracks Directory](./tracks/)
- [Task Manifest](./task-manifest.json)
- [Execution Order](../docs/EXECUTION-ORDER.md)
- [Conductor Setup and Verification](../docs/CONDUCTOR-SETUP.md)

Current accounting: 116 tasks in 15 tracks; 14 completed, 12 in progress, 0 blocked, 90 not started.
Next eligible task: **T00.06**. Resume partial work; do not recreate the Rust workspace or restart all tasks.

The task manifest owns status, prerequisites and acceptance evidence. `conductor_doctor.py --sync` regenerates plans/indexes/metadata; `--check` rejects drift. Sync does not change task status or certify correctness.

Enron is verification/example data only. `foi-process` is the first consumer; legacy outputs are not an oracle. GraphRAG/model work is optional and cannot block the core.

Pinned Conductor: `7a5c560a4fdf5297be58594cc37527eb12790272` at `.agents/plugins/conductor`. Repository setup is audited from artifacts and command manifests; host extension registration on the user's computer is not claimed. Do not rerun generic setup over this context.
