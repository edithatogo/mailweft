# Conductor — email-ocel

This is a Conductor-compatible, noninteractive repository context. It was prepared before running the interactive Conductor plugin to let a basic coding model implement tasks without inventing requirements.

Read in order: [product](product.md), [guidelines](product-guidelines.md), [tech stack](tech-stack.md), [workflow](workflow.md), [tracks](tracks.md), then exactly one track's spec and plan.

**Plugin:** `.agents/plugins/conductor` is a real Git submodule entry pinned to verified upstream `conductor-v0.4.1` commit `7a5c560a4fdf5297be58594cc37527eb12790272`. Clone/update on a networked machine using `git submodule update --init --recursive`. The project-local setup files are already populated. Do not run automated setup over them without review.

All tracks start **not implemented**. This is a delivery plan and contracts scaffold, not a working parser or process-mining engine. Execution is sequential by dependency unless plans explicitly permit parallel work.
