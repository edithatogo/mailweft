# Repository contribution metadata — executable task plan

Generated from the canonical task manifest; edit that manifest, then run `python3 scripts/conductor_doctor.py --sync`.
A partial task is not unimplemented and not complete. Evidence and remaining acceptance work are explicit.

- [~] Task: **T00a.01 — Add a scoped pull request template and maintenance evidence**

  - Lane: `core`; dependencies: none.
  - Requirements: R10, R15.
  - Allowed paths: `.github/pull_request_template.md`, `conductor/index.md`, `conductor/setup_state.json`, `conductor/task-manifest.json`, `conductor/tracks.md`, `conductor/tracks/00_foundation/metadata.json`, `conductor/tracks/00_repository_metadata/evidence.md`, `conductor/tracks/00_repository_metadata/index.md`, `conductor/tracks/00_repository_metadata/metadata.json`, `conductor/tracks/00_repository_metadata/plan.md`, `conductor/tracks/00_repository_metadata/spec.md`, `conductor/tracks/00_repository_metadata/validation.json`, `conductor/tracks/01_ingestion/metadata.json`, `conductor/tracks/02_identity/metadata.json`, `conductor/tracks/03_ocel/metadata.json`, `conductor/tracks/04_data-validation/metadata.json`, `conductor/tracks/05_attachments/metadata.json`, `conductor/tracks/05b_email-structure/metadata.json`, `conductor/tracks/06_semantics/metadata.json`, `conductor/tracks/07_research/metadata.json`, `conductor/tracks/08_release/metadata.json`, `conductor/tracks/09_patterns/metadata.json`, `conductor/tracks/10_evidence-incremental/metadata.json`, `conductor/tracks/11_frontier-validation/metadata.json`, `conductor/tracks/12_ecosystem-reuse/metadata.json`, `conductor/tracks/13_graphrag/metadata.json`, `docs/EXECUTION-ORDER.md`.
  - Action: Prepare the repository-local template and dedicated maintenance records; preserve every existing product task and report unavailable checks explicitly.
  - Acceptance: Template and Conductor/document integrity verified; existing README/workflow Rust, actual-output schema and exact-head CI gates remain required before acceptance. Leave this task uncompleted while any required gate is unavailable or unverified.
  - Verification: `python3 scripts/validate_scaffold.py; python3 scripts/conductor_doctor.py --check; python3 -m unittest discover -s scripts -p 'test_conductor_*.py'; git diff --cached --check`.
  - Evidence: `conductor/tracks/00_repository_metadata/evidence.md`.
  - Implemented: The scoped template and maintenance records are prepared. Documentation, canonical task context, eight focused Conductor regression tests and all 45 Python planning tests passed locally in the available Python environment.
  - Remaining: Task acceptance is blocked: Rust 1.99.1 cargo/rustc/rustup are unavailable locally and the first-party bootstrap download timed out. Locked Rust tests, Rust4PM import, formatting, all-feature Clippy, building the CLI, actual-output runtime/selection schema checks and exact-head hosted CI remain required and unverified. All 45 Python planning tests passed in the available Python 3.12.14 environment; the exact requirements-planning.txt environment was not installed or qualified.
