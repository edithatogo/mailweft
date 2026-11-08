# Email acts, genres and interaction patterns implementation plan

**Status:** all implementation tasks not started. **Goal:** the bounded specification in `spec.md`.
**Architecture:** retain independent Rust raw-email -> OCEL 2.0 core; optional lanes do not gate core.
**Tech stack:** qualified Rust dependencies; Python helpers only for development-document validation.
**Spec:** `spec.md`, `docs/CONTRACT.md`, `docs/FRONTIER-DESIGN.md`.

## Global constraints and review focus
No real email/source data in Git; no network/model fallback; no participant-role/authority conflation; preserve source attribution and OCEL attributes/qualifiers; observe exact task prerequisites, not whole-track gates. Test unknowns, malicious data, copied quotations, Unicode spans and missing/ambiguous references where applicable.

The manifest is authoritative for dependencies and lane. Checkboxes are completion records, not approval requests. Planning checks do not mark implementation complete.

- [ ] **T09.01 — Version the four independent classification vocabularies**
  - Lane: `enrichment`. Depends on: T06.01.
  - Edit only: `crates/email-ocel-semantics/src/taxonomy.rs`, `crates/email-ocel-semantics/tests/task_t09_01.rs`, `crates/email-ocel-semantics/src/lib.rs`.
  - Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
  - Acceptance: Unknown external labels return Unmapped; no nearest-label guessing.; A genre named approval_request cannot be used as an utterance act or zone without an explicit crosswalk.
  - Verification: `cargo test --locked -p email-ocel-semantics --test task_t09_01 && cargo fmt --all -- --check`.
  - Evidence: `conductor/tracks/09_patterns/evidence.md`. Commit: `T09.01: Version the four independent classification vocabularies`.
  - Exact interface and two oracle tests: [task card](../../tasks/T09.01.md).

- [ ] **T09.02 — Identify structured message kinds without inventing intent**
  - Lane: `enrichment`. Depends on: T09.01, T01.04.
  - Edit only: `crates/email-ocel-semantics/src/genre.rs`, `crates/email-ocel-semantics/tests/task_t09_02.rs`, `crates/email-ocel-semantics/src/lib.rs`.
  - Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
  - Acceptance: A calendar MIME part yields calendar_invitation kind, not a personal commitment.; An auto-response header yields automated_reply; CompletionReported remains absent.
  - Verification: `cargo test --locked -p email-ocel-semantics --test task_t09_02 && cargo fmt --all -- --check`.
  - Evidence: `conductor/tracks/09_patterns/evidence.md`. Commit: `T09.02: Identify structured message kinds without inventing intent`.
  - Exact interface and two oracle tests: [task card](../../tasks/T09.02.md).

- [ ] **T09.03 — Attach multi-label acts to attributed utterances**
  - Lane: `enrichment`. Depends on: T09.01, T06.04, T06.03.
  - Edit only: `crates/email-ocel-semantics/src/acts.rs`, `crates/email-ocel-semantics/tests/task_t09_03.rs`, `crates/email-ocel-semantics/src/lib.rs`.
  - Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
  - Acceptance: Two authored clauses can produce request and commit with different spans.; A quoted request retains origin=quoted and is excluded from a new-authored-act projection.
  - Verification: `cargo test --locked -p email-ocel-semantics --test task_t09_03 && cargo fmt --all -- --check`.
  - Evidence: `conductor/tracks/09_patterns/evidence.md`. Commit: `T09.03: Attach multi-label acts to attributed utterances`.
  - Exact interface and two oracle tests: [task card](../../tasks/T09.03.md).

- [ ] **T09.04 — Resolve candidate actor roles separately from recipients**
  - Lane: `enrichment`. Depends on: T09.03, T05b.06.
  - Edit only: `crates/email-ocel-semantics/src/actors.rs`, `crates/email-ocel-semantics/tests/task_t09_04.rs`, `crates/email-ocel-semantics/src/lib.rs`.
  - Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
  - Acceptance: Moving the same address from To to Cc changes header role, not an explicit body-named actor.; Two Alex identities retain two candidate IDs with unresolved status; neither is silently selected.
  - Verification: `cargo test --locked -p email-ocel-semantics --test task_t09_04 && cargo fmt --all -- --check`.
  - Evidence: `conductor/tracks/09_patterns/evidence.md`. Commit: `T09.04: Resolve candidate actor roles separately from recipients`.
  - Exact interface and two oracle tests: [task card](../../tasks/T09.04.md).

- [ ] **T09.05 — Link dialogue responses to antecedents**
  - Lane: `enrichment`. Depends on: T09.04, T10.01.
  - Edit only: `crates/email-ocel-semantics/src/interactions.rs`, `crates/email-ocel-semantics/tests/task_t09_05.rs`, `crates/email-ocel-semantics/src/lib.rs`.
  - Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
  - Acceptance: Thanks produces acknowledgement candidate, not a verified fulfilment/approval event.; A quoted antecedent and its original share dependence lineage.
  - Verification: `cargo test --locked -p email-ocel-semantics --test task_t09_05 && cargo fmt --all -- --check`.
  - Evidence: `conductor/tracks/09_patterns/evidence.md`. Commit: `T09.05: Link dialogue responses to antecedents`.
  - Exact interface and two oracle tests: [task card](../../tasks/T09.05.md).

- [ ] **T09.06 — Model commitment and withdrawal candidates**
  - Lane: `enrichment`. Depends on: T09.05.
  - Edit only: `crates/email-ocel-semantics/src/commitments.rs`, `crates/email-ocel-semantics/tests/task_t09_06.rs`, `crates/email-ocel-semantics/src/lib.rs`.
  - Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
  - Acceptance: I will send can support commitment, never task-complete.; A later withdrawal supersedes the candidate state without editing the earlier source act.
  - Verification: `cargo test --locked -p email-ocel-semantics --test task_t09_06 && cargo fmt --all -- --check`.
  - Evidence: `conductor/tracks/09_patterns/evidence.md`. Commit: `T09.06: Model commitment and withdrawal candidates`.
  - Exact interface and two oracle tests: [task card](../../tasks/T09.06.md).

- [ ] **T09.07 — Separate work-item links from thread/topic links**
  - Lane: `enrichment`. Depends on: T09.05.
  - Edit only: `crates/email-ocel-semantics/src/work_items.rs`, `crates/email-ocel-semantics/tests/task_t09_07.rs`, `crates/email-ocel-semantics/src/lib.rs`.
  - Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
  - Acceptance: Two explicit task IDs remain two work-item candidates even in one email thread.; An independently matching task reference may link across threads with evidence and uncertainty, not merge source copies.
  - Verification: `cargo test --locked -p email-ocel-semantics --test task_t09_07 && cargo fmt --all -- --check`.
  - Evidence: `conductor/tracks/09_patterns/evidence.md`. Commit: `T09.07: Separate work-item links from thread/topic links`.
  - Exact interface and two oracle tests: [task card](../../tasks/T09.07.md).

- [ ] **T09.08 — Project selected interactions without OCEL information loss**
  - Lane: `enrichment`. Depends on: T09.06, T09.07, T06.08, T10.02.
  - Edit only: `crates/email-ocel-semantics/src/interaction_projection.rs`, `crates/email-ocel-semantics/tests/task_t09_08.rs`, `crates/email-ocel-semantics/src/lib.rs`.
  - Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
  - Acceptance: Candidate-only links are omitted with a recorded reason under accepted-only policy.; Selected E2O/O2O qualifiers and typed/history attributes survive canonical round trips; sidecar references remain resolvable.
  - Verification: `cargo test --locked -p email-ocel-semantics --test task_t09_08 && cargo fmt --all -- --check`.
  - Evidence: `conductor/tracks/09_patterns/evidence.md`. Commit: `T09.08: Project selected interactions without OCEL information loss`.
  - Exact interface and two oracle tests: [task card](../../tasks/T09.08.md).
