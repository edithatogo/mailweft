# Email acts, genres and interaction patterns — executable task plan

Read the track spec, AGENTS.md and the task manifest. Each unchecked task is unimplemented. Dependencies are exact task IDs, not whole-track gates.

## [ ] **T09.01 — Version the four independent classification vocabularies**

- Lane: `enrichment`; dependencies: T06.01.
- Requirements: R30, R34.
- Allowed paths: `crates/mailweft-semantics/src/taxonomy.rs`, `crates/mailweft-semantics/tests/task_t09_01.rs`, `crates/mailweft-semantics/src/lib.rs`.
- Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
- Acceptance: Unknown external labels return Unmapped; no nearest-label guessing.; A genre named approval_request cannot be used as an utterance act or zone without an explicit crosswalk.
- Verification: `cargo test --locked -p mailweft-semantics --test task_t09_01 && cargo fmt --all -- --check`.
- Evidence: `conductor/tracks/09_patterns/evidence.md`.
- Detailed card: [`T09.01`](../../tasks/T09.01.md).

## [ ] **T09.02 — Identify structured message kinds without inventing intent**

- Lane: `enrichment`; dependencies: T09.01, T01.04.
- Requirements: R33, R34.
- Allowed paths: `crates/mailweft-semantics/src/genre.rs`, `crates/mailweft-semantics/tests/task_t09_02.rs`, `crates/mailweft-semantics/src/lib.rs`.
- Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
- Acceptance: A calendar MIME part yields calendar_invitation kind, not a personal commitment.; An auto-response header yields automated_reply; CompletionReported remains absent.
- Verification: `cargo test --locked -p mailweft-semantics --test task_t09_02 && cargo fmt --all -- --check`.
- Evidence: `conductor/tracks/09_patterns/evidence.md`.
- Detailed card: [`T09.02`](../../tasks/T09.02.md).

## [ ] **T09.03 — Attach multi-label acts to attributed utterances**

- Lane: `enrichment`; dependencies: T09.01, T06.04, T06.03.
- Requirements: R30, R35.
- Allowed paths: `crates/mailweft-semantics/src/acts.rs`, `crates/mailweft-semantics/tests/task_t09_03.rs`, `crates/mailweft-semantics/src/lib.rs`.
- Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
- Acceptance: Two authored clauses can produce request and commit with different spans.; A quoted request retains origin=quoted and is excluded from a new-authored-act projection.
- Verification: `cargo test --locked -p mailweft-semantics --test task_t09_03 && cargo fmt --all -- --check`.
- Evidence: `conductor/tracks/09_patterns/evidence.md`.
- Detailed card: [`T09.03`](../../tasks/T09.03.md).

## [ ] **T09.04 — Resolve candidate actor roles separately from recipients**

- Lane: `enrichment`; dependencies: T09.03, T05b.06.
- Requirements: R27, R33, R37.
- Allowed paths: `crates/mailweft-semantics/src/actors.rs`, `crates/mailweft-semantics/tests/task_t09_04.rs`, `crates/mailweft-semantics/src/lib.rs`.
- Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
- Acceptance: Moving the same address from To to Cc changes header role, not an explicit body-named actor.; Two Alex identities retain two candidate IDs with unresolved status; neither is silently selected.
- Verification: `cargo test --locked -p mailweft-semantics --test task_t09_04 && cargo fmt --all -- --check`.
- Evidence: `conductor/tracks/09_patterns/evidence.md`.
- Detailed card: [`T09.04`](../../tasks/T09.04.md).

## [ ] **T09.05 — Link dialogue responses to antecedents**

- Lane: `enrichment`; dependencies: T09.04, T10.01.
- Requirements: R36, R39.
- Allowed paths: `crates/mailweft-semantics/src/interactions.rs`, `crates/mailweft-semantics/tests/task_t09_05.rs`, `crates/mailweft-semantics/src/lib.rs`.
- Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
- Acceptance: Thanks produces acknowledgement candidate, not a verified fulfilment/approval event.; A quoted antecedent and its original share dependence lineage.
- Verification: `cargo test --locked -p mailweft-semantics --test task_t09_05 && cargo fmt --all -- --check`.
- Evidence: `conductor/tracks/09_patterns/evidence.md`.
- Detailed card: [`T09.05`](../../tasks/T09.05.md).

## [ ] **T09.06 — Model commitment and withdrawal candidates**

- Lane: `enrichment`; dependencies: T09.05.
- Requirements: R36, R37.
- Allowed paths: `crates/mailweft-semantics/src/commitments.rs`, `crates/mailweft-semantics/tests/task_t09_06.rs`, `crates/mailweft-semantics/src/lib.rs`.
- Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
- Acceptance: I will send can support commitment, never task-complete.; A later withdrawal supersedes the candidate state without editing the earlier source act.
- Verification: `cargo test --locked -p mailweft-semantics --test task_t09_06 && cargo fmt --all -- --check`.
- Evidence: `conductor/tracks/09_patterns/evidence.md`.
- Detailed card: [`T09.06`](../../tasks/T09.06.md).

## [ ] **T09.07 — Separate work-item links from thread/topic links**

- Lane: `enrichment`; dependencies: T09.05.
- Requirements: R33, R36, R37.
- Allowed paths: `crates/mailweft-semantics/src/work_items.rs`, `crates/mailweft-semantics/tests/task_t09_07.rs`, `crates/mailweft-semantics/src/lib.rs`.
- Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
- Acceptance: Two explicit task IDs remain two work-item candidates even in one email thread.; An independently matching task reference may link across threads with evidence and uncertainty, not merge source copies.
- Verification: `cargo test --locked -p mailweft-semantics --test task_t09_07 && cargo fmt --all -- --check`.
- Evidence: `conductor/tracks/09_patterns/evidence.md`.
- Detailed card: [`T09.07`](../../tasks/T09.07.md).

## [ ] **T09.08 — Project selected interactions without OCEL information loss**

- Lane: `enrichment`; dependencies: T09.06, T09.07, T06.08, T10.02.
- Requirements: R07, R35, R36.
- Allowed paths: `crates/mailweft-semantics/src/interaction_projection.rs`, `crates/mailweft-semantics/tests/task_t09_08.rs`, `crates/mailweft-semantics/src/lib.rs`.
- Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
- Acceptance: Candidate-only links are omitted with a recorded reason under accepted-only policy.; Selected E2O/O2O qualifiers and typed/history attributes survive canonical round trips; sidecar references remain resolvable.
- Verification: `cargo test --locked -p mailweft-semantics --test task_t09_08 && cargo fmt --all -- --check`.
- Evidence: `conductor/tracks/09_patterns/evidence.md`.
- Detailed card: [`T09.08`](../../tasks/T09.08.md).
