# Evidence receipts, alternatives and incremental interpretation implementation plan

**Status:** all implementation tasks not started. **Goal:** the bounded specification in `spec.md`.
**Architecture:** retain independent Rust raw-email -> OCEL 2.0 core; optional lanes do not gate core.
**Tech stack:** qualified Rust dependencies; Python helpers only for development-document validation.
**Spec:** `spec.md`, `docs/CONTRACT.md`, `docs/FRONTIER-DESIGN.md`.

## Global constraints and review focus
No real email/source data in Git; no network/model fallback; no participant-role/authority conflation; preserve source attribution and OCEL attributes/qualifiers; observe exact task prerequisites, not whole-track gates. Test unknowns, malicious data, copied quotations, Unicode spans and missing/ambiguous references where applicable.

The manifest is authoritative for dependencies and lane. Checkboxes are completion records, not approval requests. Planning checks do not mark implementation complete.

- [ ] **T10.01 — Add evidence receipt types and semantic checker**
  - Lane: `enrichment`. Depends on: T06.01, T05b.03.
  - Edit only: `crates/email-ocel-semantics/src/receipts.rs`, `crates/email-ocel-semantics/tests/task_t10_01.rs`, `crates/email-ocel-semantics/src/lib.rs`.
  - Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
  - Acceptance: Unknown evidence IDs cause missing_evidence diagnostic; no admission.; A structurally valid candidate receipt remains a candidate, not a verified business event.
  - Verification: `cargo test --locked -p email-ocel-semantics --test task_t10_01 && cargo fmt --all -- --check`.
  - Evidence: `conductor/tracks/10_evidence-incremental/evidence.md`. Commit: `T10.01: Add evidence receipt types and semantic checker`.
  - Exact interface and two oracle tests: [task card](../../tasks/T10.01.md).

- [ ] **T10.02 — Check authorization, anchoring and source-map fidelity**
  - Lane: `enrichment`. Depends on: T10.01, T06.04.
  - Edit only: `crates/email-ocel-semantics/src/receipt_policy.rs`, `crates/email-ocel-semantics/tests/task_t10_02.rs`, `crates/email-ocel-semantics/src/lib.rs`.
  - Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
  - Acceptance: Source hash mismatch or non-UTF8 boundary prevents admission.; A mandatory attribution/authorization check marked not_run cannot produce admit_to_enriched_view.
  - Verification: `cargo test --locked -p email-ocel-semantics --test task_t10_02 && cargo fmt --all -- --check`.
  - Evidence: `conductor/tracks/10_evidence-incremental/evidence.md`. Commit: `T10.02: Check authorization, anchoring and source-map fidelity`.
  - Exact interface and two oracle tests: [task card](../../tasks/T10.02.md).

- [ ] **T10.03 — Track dependent copied and quoted evidence**
  - Lane: `enrichment`. Depends on: T02.02, T05b.03.
  - Edit only: `crates/email-ocel-semantics/src/dependence.rs`, `crates/email-ocel-semantics/tests/task_t10_03.rs`, `crates/email-ocel-semantics/src/lib.rs`.
  - Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
  - Acceptance: Ten identical copied source claims contribute one dependence group, not ten independent corroborations.; Two classifiers on one span preserve one source lineage.
  - Verification: `cargo test --locked -p email-ocel-semantics --test task_t10_03 && cargo fmt --all -- --check`.
  - Evidence: `conductor/tracks/10_evidence-incremental/evidence.md`. Commit: `T10.03: Track dependent copied and quoted evidence`.
  - Exact interface and two oracle tests: [task card](../../tasks/T10.03.md).

- [ ] **T10.04 — Preserve temporal intervals and partial order**
  - Lane: `enrichment`. Depends on: T06.03, T01.05.
  - Edit only: `crates/email-ocel-semantics/src/temporal.rs`, `crates/email-ocel-semantics/tests/task_t10_04.rs`, `crates/email-ocel-semantics/src/lib.rs`.
  - Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
  - Acceptance: Without disambiguating locale/reference context, Friday stays unresolved or a candidate interval; never a fabricated point.; Equal timestamps use stable serialization order while causal ordering remains unknown.
  - Verification: `cargo test --locked -p email-ocel-semantics --test task_t10_04 && cargo fmt --all -- --check`.
  - Evidence: `conductor/tracks/10_evidence-incremental/evidence.md`. Commit: `T10.04: Preserve temporal intervals and partial order`.
  - Exact interface and two oracle tests: [task card](../../tasks/T10.04.md).

- [ ] **T10.05 — Define revision and retraction ledger**
  - Lane: `enrichment`. Depends on: T06.01.
  - Edit only: `crates/email-ocel-semantics/src/deltas.rs`, `crates/email-ocel-semantics/tests/task_t10_05.rs`, `crates/email-ocel-semantics/src/lib.rs`.
  - Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
  - Acceptance: An identical delta replay is a no-op; conflicting bytes with the same key produce conflict.; A supersession referencing the wrong prior hash does not alter active state.
  - Verification: `cargo test --locked -p email-ocel-semantics --test task_t10_05 && cargo fmt --all -- --check`.
  - Evidence: `conductor/tracks/10_evidence-incremental/evidence.md`. Commit: `T10.05: Define revision and retraction ledger`.
  - Exact interface and two oracle tests: [task card](../../tasks/T10.05.md).

- [ ] **T10.06 — Materialize current and as-of interpretation snapshots**
  - Lane: `enrichment`. Depends on: T10.05, T10.03.
  - Edit only: `crates/email-ocel-semantics/src/replay.rs`, `crates/email-ocel-semantics/tests/task_t10_06.rs`, `crates/email-ocel-semantics/src/lib.rs`.
  - Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
  - Acceptance: Upsert followed by retract leaves no active claim; prior as-of snapshot remains explainable.; Replay cannot update raw ObservedMessage or its source-copy identity.
  - Verification: `cargo test --locked -p email-ocel-semantics --test task_t10_06 && cargo fmt --all -- --check`.
  - Evidence: `conductor/tracks/10_evidence-incremental/evidence.md`. Commit: `T10.06: Materialize current and as-of interpretation snapshots`.
  - Exact interface and two oracle tests: [task card](../../tasks/T10.06.md).

- [ ] **T10.07 — Propagate restricted scope and permitted deletion**
  - Lane: `enrichment`. Depends on: T10.02, T10.06.
  - Edit only: `crates/email-ocel-semantics/src/scope.rs`, `crates/email-ocel-semantics/tests/task_t10_07.rs`, `crates/email-ocel-semantics/src/lib.rs`.
  - Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
  - Acceptance: A downstream path/summary containing restricted Bcc evidence is unavailable to an unauthorized scope.; Purge plan includes derived embeddings/indexes/summaries and retains only permitted tombstone metadata; no automatic destructive apply.
  - Verification: `cargo test --locked -p email-ocel-semantics --test task_t10_07 && cargo fmt --all -- --check`.
  - Evidence: `conductor/tracks/10_evidence-incremental/evidence.md`. Commit: `T10.07: Propagate restricted scope and permitted deletion`.
  - Exact interface and two oracle tests: [task card](../../tasks/T10.07.md).

- [ ] **T10.08 — Invalidate caches and bound incremental recomputation**
  - Lane: `enrichment`. Depends on: T10.06, T10.07.
  - Edit only: `crates/email-ocel-semantics/src/incremental.rs`, `crates/email-ocel-semantics/tests/task_t10_08.rs`, `crates/email-ocel-semantics/src/lib.rs`.
  - Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
  - Acceptance: Changed parser/model/prompt/taxonomy configuration cannot reuse an old interpretation cache key.; Exceeded work budget produces explicit pending/partial state, never a complete result.
  - Verification: `cargo test --locked -p email-ocel-semantics --test task_t10_08 && cargo fmt --all -- --check`.
  - Evidence: `conductor/tracks/10_evidence-incremental/evidence.md`. Commit: `T10.08: Invalidate caches and bound incremental recomputation`.
  - Exact interface and two oracle tests: [task card](../../tasks/T10.08.md).
