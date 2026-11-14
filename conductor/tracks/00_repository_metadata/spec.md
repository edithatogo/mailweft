# Repository contribution metadata

## Assignment and purpose

The repository owner requested a suitable repository-local pull request template.
This isolated documentation task is assigned to the current repository-maintenance
coordinator on branch `docs/pr-template-20261115`, from base `14b3de557a7654708f6066f70f4d8bc1e3d1e564`.
Its identity is **T00a.01**. It is independent of the converter implementation
tasks; existing task IDs, dependencies, statuses and acceptance remain unchanged.

## Required result

Add `.github/pull_request_template.md` with proportionate prompts for purpose,
related work, actual validation, impact, documentation and author privacy review.
Use no real mail, patient/personnel information, source messages or embeddings.
Retain current contributor requirements and require honest unavailable-check reports.

The canonical task manifest and its existing renderer own the track records.
The reconciliation timestamp updates existing generated metadata timestamps only;
that generated change does not accept or reopen any existing task.

## Owned files

- `.github/pull_request_template.md`
- `conductor/index.md`
- `conductor/setup_state.json`
- `conductor/task-manifest.json`
- `conductor/tracks.md`
- `conductor/tracks/00_foundation/metadata.json`
- `conductor/tracks/00_repository_metadata/evidence.md`
- `conductor/tracks/00_repository_metadata/index.md`
- `conductor/tracks/00_repository_metadata/metadata.json`
- `conductor/tracks/00_repository_metadata/plan.md`
- `conductor/tracks/00_repository_metadata/spec.md`
- `conductor/tracks/00_repository_metadata/validation.json`
- `conductor/tracks/01_ingestion/metadata.json`
- `conductor/tracks/02_identity/metadata.json`
- `conductor/tracks/03_ocel/metadata.json`
- `conductor/tracks/04_data-validation/metadata.json`
- `conductor/tracks/05_attachments/metadata.json`
- `conductor/tracks/05b_email-structure/metadata.json`
- `conductor/tracks/06_semantics/metadata.json`
- `conductor/tracks/07_research/metadata.json`
- `conductor/tracks/08_release/metadata.json`
- `conductor/tracks/09_patterns/metadata.json`
- `conductor/tracks/10_evidence-incremental/metadata.json`
- `conductor/tracks/11_frontier-validation/metadata.json`
- `conductor/tracks/12_ecosystem-reuse/metadata.json`
- `conductor/tracks/13_graphrag/metadata.json`
- `docs/EXECUTION-ORDER.md`

## Verification and acceptance

1. Inspect the template, file scope, preserved task objects and existing metadata
   fields; only generated `updated_at` fields may change on existing tracks.
2. Run `python3 scripts/validate_scaffold.py`,
   `python3 scripts/conductor_doctor.py --sync`,
   `python3 scripts/conductor_doctor.py --check`,
   `python3 -m unittest discover -s scripts -p 'test_conductor_*.py'`,
   `python3 -m unittest discover -s scripts -p 'test_*.py'`, and
   `git diff --cached --check`. Record actual outcomes in `validation.json`.
3. Retain the required locked Rust tests, Rust4PM import checks, formatting,
   all-feature Clippy, the pinned planning/schema environment, actual-output
   runtime/selection schema checks and exact-head CI from the existing
   README/workflow before task acceptance and integration. This task grants no
   waiver. If tooling is unavailable, record blocked acceptance and leave the
   generated task checkbox uncompleted; a draft PR exposes the verified
   documentation change and the precise remaining gates.

## Boundaries and recovery

This task changes repository contribution records. Product code, dependency pins,
schemas, FOI ownership, data rights, optional research and live services are outside
its scope. No old task is repurposed or marked complete. A revert can remove this
isolated metadata assignment/template while retaining the original product state.
