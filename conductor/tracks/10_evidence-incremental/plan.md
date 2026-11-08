# Evidence receipts, alternatives and incremental interpretation — executable task plan

Read the track spec, AGENTS.md and the task manifest. Each unchecked task is unimplemented. Dependencies are exact task IDs, not whole-track gates.

## [ ] **T10.01 — Add evidence receipt types and semantic checker**

- Lane: `enrichment`; dependencies: T06.01, T05b.03.
- Requirements: R35.
- Allowed paths: `crates/mailweft-semantics/src/receipts.rs`, `crates/mailweft-semantics/tests/task_t10_01.rs`, `crates/mailweft-semantics/src/lib.rs`.
- Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
- Acceptance: Unknown evidence IDs cause missing_evidence diagnostic; no admission.; A structurally valid candidate receipt remains a candidate, not a verified business event.
- Verification: `cargo test --locked -p mailweft-semantics --test task_t10_01 && cargo fmt --all -- --check`.
- Evidence: `conductor/tracks/10_evidence-incremental/evidence.md`.
- Detailed card: [`T10.01`](../../tasks/T10.01.md).

## [ ] **T10.02 — Check authorization, anchoring and source-map fidelity**

- Lane: `enrichment`; dependencies: T10.01, T06.04.
- Requirements: R35, R43.
- Allowed paths: `crates/mailweft-semantics/src/receipt_policy.rs`, `crates/mailweft-semantics/tests/task_t10_02.rs`, `crates/mailweft-semantics/src/lib.rs`.
- Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
- Acceptance: Source hash mismatch or non-UTF8 boundary prevents admission.; A mandatory attribution/authorization check marked not_run cannot produce admit_to_enriched_view.
- Verification: `cargo test --locked -p mailweft-semantics --test task_t10_02 && cargo fmt --all -- --check`.
- Evidence: `conductor/tracks/10_evidence-incremental/evidence.md`.
- Detailed card: [`T10.02`](../../tasks/T10.02.md).

## [ ] **T10.03 — Track dependent copied and quoted evidence**

- Lane: `enrichment`; dependencies: T02.02, T05b.03.
- Requirements: R39.
- Allowed paths: `crates/mailweft-semantics/src/dependence.rs`, `crates/mailweft-semantics/tests/task_t10_03.rs`, `crates/mailweft-semantics/src/lib.rs`.
- Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
- Acceptance: Ten identical copied source claims contribute one dependence group, not ten independent corroborations.; Two classifiers on one span preserve one source lineage.
- Verification: `cargo test --locked -p mailweft-semantics --test task_t10_03 && cargo fmt --all -- --check`.
- Evidence: `conductor/tracks/10_evidence-incremental/evidence.md`.
- Detailed card: [`T10.03`](../../tasks/T10.03.md).

## [ ] **T10.04 — Preserve temporal intervals and partial order**

- Lane: `enrichment`; dependencies: T06.03, T01.05.
- Requirements: R37.
- Allowed paths: `crates/mailweft-semantics/src/temporal.rs`, `crates/mailweft-semantics/tests/task_t10_04.rs`, `crates/mailweft-semantics/src/lib.rs`.
- Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
- Acceptance: Without disambiguating locale/reference context, Friday stays unresolved or a candidate interval; never a fabricated point.; Equal timestamps use stable serialization order while causal ordering remains unknown.
- Verification: `cargo test --locked -p mailweft-semantics --test task_t10_04 && cargo fmt --all -- --check`.
- Evidence: `conductor/tracks/10_evidence-incremental/evidence.md`.
- Detailed card: [`T10.04`](../../tasks/T10.04.md).

## [ ] **T10.05 — Define revision and retraction ledger**

- Lane: `enrichment`; dependencies: T06.01.
- Requirements: R38.
- Allowed paths: `crates/mailweft-semantics/src/deltas.rs`, `crates/mailweft-semantics/tests/task_t10_05.rs`, `crates/mailweft-semantics/src/lib.rs`.
- Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
- Acceptance: An identical delta replay is a no-op; conflicting bytes with the same key produce conflict.; A supersession referencing the wrong prior hash does not alter active state.
- Verification: `cargo test --locked -p mailweft-semantics --test task_t10_05 && cargo fmt --all -- --check`.
- Evidence: `conductor/tracks/10_evidence-incremental/evidence.md`.
- Detailed card: [`T10.05`](../../tasks/T10.05.md).

## [ ] **T10.06 — Materialize current and as-of interpretation snapshots**

- Lane: `enrichment`; dependencies: T10.05, T10.03.
- Requirements: R38, R39.
- Allowed paths: `crates/mailweft-semantics/src/replay.rs`, `crates/mailweft-semantics/tests/task_t10_06.rs`, `crates/mailweft-semantics/src/lib.rs`.
- Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
- Acceptance: Upsert followed by retract leaves no active claim; prior as-of snapshot remains explainable.; Replay cannot update raw ObservedMessage or its source-copy identity.
- Verification: `cargo test --locked -p mailweft-semantics --test task_t10_06 && cargo fmt --all -- --check`.
- Evidence: `conductor/tracks/10_evidence-incremental/evidence.md`.
- Detailed card: [`T10.06`](../../tasks/T10.06.md).

## [ ] **T10.07 — Propagate restricted scope and permitted deletion**

- Lane: `enrichment`; dependencies: T10.02, T10.06.
- Requirements: R18, R43.
- Allowed paths: `crates/mailweft-semantics/src/scope.rs`, `crates/mailweft-semantics/tests/task_t10_07.rs`, `crates/mailweft-semantics/src/lib.rs`.
- Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
- Acceptance: A downstream path/summary containing restricted Bcc evidence is unavailable to an unauthorized scope.; Purge plan includes derived embeddings/indexes/summaries and retains only permitted tombstone metadata; no automatic destructive apply.
- Verification: `cargo test --locked -p mailweft-semantics --test task_t10_07 && cargo fmt --all -- --check`.
- Evidence: `conductor/tracks/10_evidence-incremental/evidence.md`.
- Detailed card: [`T10.07`](../../tasks/T10.07.md).

## [ ] **T10.08 — Invalidate caches and bound incremental recomputation**

- Lane: `enrichment`; dependencies: T10.06, T10.07.
- Requirements: R38, R46.
- Allowed paths: `crates/mailweft-semantics/src/incremental.rs`, `crates/mailweft-semantics/tests/task_t10_08.rs`, `crates/mailweft-semantics/src/lib.rs`.
- Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
- Acceptance: Changed parser/model/prompt/taxonomy configuration cannot reuse an old interpretation cache key.; Exceeded work budget produces explicit pending/partial state, never a complete result.
- Verification: `cargo test --locked -p mailweft-semantics --test task_t10_08 && cargo fmt --all -- --check`.
- Evidence: `conductor/tracks/10_evidence-incremental/evidence.md`.
- Detailed card: [`T10.08`](../../tasks/T10.08.md).
