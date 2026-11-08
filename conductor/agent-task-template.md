# Single-task execution contract for a basic coding agent

Use one task at a time: no broad autonomous implementation.

1. Read `AGENTS.md`, `conductor/index.md`, `docs/CONTRACT.md`, `docs/REQUIREMENTS.md`, `docs/DESIGN.md`, `docs/DATASETS.md`.
2. Read the assigned task's `conductor/task-manifest.json` entry and the enclosing `conductor/tracks/<id>/{spec,plan}.md`.
3. Check every listed dependency is done; if not, stop and identify the blocker.
4. Show the specific failing synthetic test or document assertion that measures acceptance. Make a small change only in allowed paths. Do not refactor unrelated files.
5. Run test/format/static checks that exist. Tests requiring external datasets, network, Rust tooling or approval remain explicitly BLOCKED if unavailable; do not substitute a mock pass.
6. Record actual evidence in the track's `evidence.md`, update only this task checkbox if accepted, commit with the exact task ID, and stop.
7. Do not change the source specification or ontology without a separate documented design decision and compatibility test.

## Suggested agent instruction

> Implement ONLY task Txx.yy from `conductor/task-manifest.json`. Read its track specification and shared contracts. Follow the allowed-path, safety, acceptance and dependency requirements. Provide actual verification results and stop after one task. If blocked, do not mark the task as complete or widen the task.
