# Controlled benchmarks and frontier experiments — executable task plan

Read the track spec, AGENTS.md and the task manifest. Each unchecked task is unimplemented. Dependencies are exact task IDs, not whole-track gates.

## [ ] **T11.01 — Freeze independent annotation and split manifests**

- Lane: `research`; dependencies: T05b.07, T09.03.
- Requirements: R31, R44, R49.
- Allowed paths: `research/splits.md`, `scripts/check_task_t11_01.py`.
- Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
- Acceptance: Duplicate/quote/conversation connected components cannot span training and held-out evaluation.; Annotation manifest records independent adjudication and freeze before model-output review.
- Verification: `python3 scripts/check_task_t11_01.py && python3 scripts/validate_scaffold.py`.
- Evidence: `conductor/tracks/11_frontier-validation/evidence.md`.
- Detailed card: [`T11.01`](../../tasks/T11.01.md).

## [ ] **T11.02 — Implement controlled semantic perturbation fixtures**

- Lane: `research`; dependencies: T09.04, T09.05, T10.04.
- Requirements: R41.
- Allowed paths: `crates/mailweft-semantics/src/metamorphic.rs`, `crates/mailweft-semantics/tests/task_t11_02.rs`, `crates/mailweft-semantics/src/lib.rs`.
- Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
- Acceptance: To/Cc swap changes only eligible header qualifiers, not a body-explicit actor.; Inserting negation or moving text into quotation changes only the declared polarity/origin expectations and never creates verified events.
- Verification: `cargo test --locked -p mailweft-semantics --test task_t11_02 && cargo fmt --all -- --check`.
- Evidence: `conductor/tracks/11_frontier-validation/evidence.md`.
- Detailed card: [`T11.02`](../../tasks/T11.02.md).

## [ ] **T11.03 — Measure calibration, abstention and group-level uncertainty**

- Lane: `research`; dependencies: T11.01, T09.03.
- Requirements: R40.
- Allowed paths: `crates/mailweft-semantics/src/calibration.rs`, `crates/mailweft-semantics/tests/task_t11_03.rs`, `crates/mailweft-semantics/src/lib.rs`.
- Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
- Acceptance: No eligible gold labels yields metric=null with denominator=0, not perfect accuracy.; Uncalibrated scores are reported as scores; no probability or universal guarantee label.
- Verification: `cargo test --locked -p mailweft-semantics --test task_t11_03 && cargo fmt --all -- --check`.
- Evidence: `conductor/tracks/11_frontier-validation/evidence.md`.
- Detailed card: [`T11.03`](../../tasks/T11.03.md).

## [ ] **T11.04 — Qualify bounded rule-to-local-model routing**

- Lane: `research`; dependencies: T06.05, T11.03, T10.02.
- Requirements: R40, R46.
- Allowed paths: `crates/mailweft-semantics/src/routing.rs`, `crates/mailweft-semantics/tests/task_t11_04.rs`, `crates/mailweft-semantics/src/lib.rs`.
- Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
- Acceptance: Exhausted token/time budget abstains without cloud fallback.; Absent model capability reports unavailable for that lane and leaves Stage 1 usable.
- Verification: `cargo test --locked -p mailweft-semantics --test task_t11_04 && cargo fmt --all -- --check`.
- Evidence: `conductor/tracks/11_frontier-validation/evidence.md`.
- Detailed card: [`T11.04`](../../tasks/T11.04.md).

## [ ] **T11.05 — Measure downstream process-distortion sensitivity**

- Lane: `research`; dependencies: T09.08, T11.02, T11.03.
- Requirements: R42.
- Allowed paths: `crates/mailweft-semantics/src/process_metrics.rs`, `crates/mailweft-semantics/tests/task_t11_05.rs`, `crates/mailweft-semantics/src/lib.rs`.
- Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
- Acceptance: Synthetic duplicate acts cause detected edge/event inflation versus a frozen truth log.; Different event-view policies or object-vocabulary versions block paired comparisons unless an explicit crosswalk exists.
- Verification: `cargo test --locked -p mailweft-semantics --test task_t11_05 && cargo fmt --all -- --check`.
- Evidence: `conductor/tracks/11_frontier-validation/evidence.md`.
- Detailed card: [`T11.05`](../../tasks/T11.05.md).

## [ ] **T11.06 — Register and reproduce current text-to-OCEL comparators**

- Lane: `research`; dependencies: T11.01.
- Requirements: R44.
- Allowed paths: `research/comparators.md`, `scripts/check_task_t11_06.py`.
- Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
- Acceptance: Unknown comparator rights or runtime is blocked/not_run; no copied/translated upstream source.; Register collector/refiner variants where reproducible, equal input scope and no unsupported email-specific ground truth.
- Verification: `python3 scripts/check_task_t11_06.py && python3 scripts/validate_scaffold.py`.
- Evidence: `conductor/tracks/11_frontier-validation/evidence.md`.
- Detailed card: [`T11.06`](../../tasks/T11.06.md).

## [ ] **T11.07 — Benchmark graph value and retrieval scope leakage**

- Lane: `research`; dependencies: T07.05, T10.07, T11.05, T13.04.
- Requirements: R23, R43, R44.
- Allowed paths: `research/graph-value.md`, `scripts/check_task_t11_07.py`.
- Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
- Acceptance: All arms use the same permitted source set and declared budget; unavailable arm stays not_run.; Generated answers cannot be admitted as new observed source evidence; uncited answers abstain.
- Verification: `python3 scripts/check_task_t11_07.py && python3 scripts/validate_scaffold.py`.
- Evidence: `conductor/tracks/11_frontier-validation/evidence.md`.
- Detailed card: [`T11.07`](../../tasks/T11.07.md).

## [ ] **T11.08 — Define latent-truth simulation and independent renderers**

- Lane: `research`; dependencies: T11.01.
- Requirements: R41, R44, R45.
- Allowed paths: `research/synthetic-worlds.md`, `scripts/check_task_t11_08.py`.
- Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
- Acceptance: Latent workflow ledger is gold; generated email or LLM judge is not gold.; Synthetic templates/paraphrase families remain isolated across train/test; No external simulator or owned repository is required for latent-ledger fixtures.
- Verification: `python3 scripts/check_task_t11_08.py && python3 scripts/validate_scaffold.py`.
- Evidence: `conductor/tracks/11_frontier-validation/evidence.md`.
- Detailed card: [`T11.08`](../../tasks/T11.08.md).
