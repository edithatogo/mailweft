# Agent workflow

1. Select the earliest unblocked unchecked track and first unchecked Txx.yy step.
2. Read that track's `spec.md`, `plan.md`, shared contracts and minimal directly relevant source code.
3. Make the smallest change necessary to satisfy the single task's acceptance. Use existing libraries before custom algorithms. No unrelated refactors.
4. Run exactly the listed tests/commands and capture exit code; if dependencies, network, rights or tooling are missing, mark BLOCKED in the private/logged evidence and stop. Do not fabricate pass results.
5. Update track evidence and checklist only on successful verification; commit the task separately with `Txx.yy` in commit message.
6. After every phase, rerun specified integration tests. Review architecture if data loss, confidence confusion, unsafe content processing or licence incompatibility is discovered.
7. All external corpora remain outside source control. Never download or process a dataset merely because a task references it; check the dataset rights gate.
8. Before a release, run full synthetic acceptance suite, verify OCEL round trips, docs, supply chain checks and zero-sensitive-artifact audit.
