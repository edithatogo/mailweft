# Conductor — email-ocel

This is a Conductor-compatible, noninteractive repository context. It was prepared before running the interactive Conductor plugin to let a basic coding model implement tasks without inventing requirements.

Read in order: [product](product.md), [guidelines](product-guidelines.md), [tech stack](tech-stack.md), [workflow](workflow.md), [tracks](tracks.md), then exactly one track's spec and plan.

**Plugin:** `.agents/plugins/conductor` is a real Git submodule entry pinned to verified upstream `conductor-v0.4.1` commit `7a5c560a4fdf5297be58594cc37527eb12790272`. Clone/update on a networked machine using `git submodule update --init --recursive`. The project-local setup files are already populated. Do not run automated setup over them without review.

All tracks start **not implemented**. This is a delivery plan and contracts scaffold, not a working parser or process-mining engine. Execution is sequential by dependency unless plans explicitly permit parallel work.

## Refinement 0.2: patterns, evidence and frontier evaluation

- [Extension design](../docs/FRONTIER-DESIGN.md)
- [Research comparison and metamorphic evaluation](../docs/FRONTIER-EVALUATION.md)
- [Repository reuse and limitations](../docs/REPOSITORY-REUSE.md)
- [Execution order](../docs/EXECUTION-ORDER.md)

The original raw-email-to-OCEL 2.0 core is preserved. New types are additive and advanced lanes remain optional. Planning now includes 47 requirements, 14 tracks and 108 tasks (30 new detailed task cards). No Rust functionality has been implemented by this update.
