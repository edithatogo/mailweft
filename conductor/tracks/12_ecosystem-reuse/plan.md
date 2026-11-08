# First FOI consumer and reviewed ontology feedback — executable task plan

Read the track spec, AGENTS.md and the task manifest. Each unchecked task is unimplemented. Dependencies are exact task IDs, not whole-track gates.

## [ ] **T12.01 — Document independent FOI acceptance and reproduce diagnostic defects**

- Lane: `integration`; dependencies: T03.08.
- Requirements: R07, R49.
- Allowed paths: `docs/integrations/foi-independent-acceptance.json`, `scripts/check_task_t12_01.py`.
- Action: Freeze source-based expected results before inspecting pipeline predictions. Reproduce individual suspected legacy defects only for diagnosis; never use legacy output as an expected-output baseline.
- Acceptance: Both declared fictional packet oracles pass; unsupported runtime and source ownership stay explicit. Independent evidence only; no legacy expected-output comparison.
- Verification: `python3 scripts/check_task_t12_01.py && python3 scripts/validate_scaffold.py`.
- Evidence: `conductor/tracks/12_ecosystem-reuse/evidence.md`.
- Detailed card: [`T12.01`](../../tasks/T12.01.md).

## [ ] **T12.02 — Verify acquisition/archive ownership and actual FYI source fidelity**

- Lane: `integration`; dependencies: T12.01.
- Requirements: R20, R49, R50.
- Allowed paths: `docs/integrations/foi-source-boundary.json`, `scripts/check_task_t12_02.py`.
- Action: Inventory exact fyi-cli/fyi-archive producer versions, manifests and the documented handover conflict; classify raw RFC, export, rendered page and attachment separately. Do not acquire source or invent headers.
- Acceptance: Both declared fictional packet oracles pass; unsupported runtime and source ownership stay explicit. Independent evidence only; no legacy expected-output comparison.
- Verification: `python3 scripts/check_task_t12_02.py && python3 scripts/validate_scaffold.py`.
- Evidence: `conductor/tracks/12_ecosystem-reuse/evidence.md`.
- Detailed card: [`T12.02`](../../tasks/T12.02.md).

## [ ] **T12.03 — Specify loss-complete stage-one Mailweft consumer mapping**

- Lane: `integration`; dependencies: T12.01, T12.02, T03.08.
- Requirements: R07, R25, R50.
- Allowed paths: `docs/integrations/foi-mailweft-adapter.json`, `scripts/check_task_t12_03.py`.
- Action: Define the one-way foi-process -> Mailweft invocation and full OCEL/evidence mapping; keep FOI-O terms in a pinned consumer profile. Stage-one integration requires no NLP or GraphRAG.
- Acceptance: Both declared fictional packet oracles pass; unsupported runtime and source ownership stay explicit. Independent evidence only; no legacy expected-output comparison.
- Verification: `python3 scripts/check_task_t12_03.py && python3 scripts/validate_scaffold.py`.
- Evidence: `conductor/tracks/12_ecosystem-reuse/evidence.md`.
- Detailed card: [`T12.03`](../../tasks/T12.03.md).

## [ ] **T12.04 — Define independently adjudicated FOI holdout acceptance**

- Lane: `integration`; dependencies: T12.03, T04.01.
- Requirements: R23, R40, R49, R50.
- Allowed paths: `docs/integrations/foi-holdout.json`, `scripts/check_task_t12_04.py`.
- Action: Prepare source-first blinded annotation and heldout whole-request partitions. Platform status, legacy output and model output may not supply independent gold. Evaluate metadata-only, correspondence-only and combined evidence arms.
- Acceptance: Both declared fictional packet oracles pass; unsupported runtime and source ownership stay explicit. Independent evidence only; no legacy expected-output comparison.
- Verification: `python3 scripts/check_task_t12_04.py && python3 scripts/validate_scaffold.py`.
- Evidence: `conductor/tracks/12_ecosystem-reuse/evidence.md`.
- Detailed card: [`T12.04`](../../tasks/T12.04.md).

## [ ] **T12.05 — Prepare validated consumer migration and safe recovery**

- Lane: `integration`; dependencies: T12.03, T12.04.
- Requirements: R38, R43, R49, R50.
- Allowed paths: `docs/integrations/foi-migration.json`, `scripts/check_task_t12_05.py`.
- Action: Specify cutover evidence and replay with independent expectations. Recovery must disable/quarantine analysis or use a verified snapshot; the unvalidated old pipeline is not a safe rollback reference.
- Acceptance: Both declared fictional packet oracles pass; unsupported runtime and source ownership stay explicit. Independent evidence only; no legacy expected-output comparison.
- Verification: `python3 scripts/check_task_t12_05.py && python3 scripts/validate_scaffold.py`.
- Evidence: `conductor/tracks/12_ecosystem-reuse/evidence.md`.
- Detailed card: [`T12.05`](../../tasks/T12.05.md).

## [ ] **T12.06 — Qualify FOI consumer handoff independently of core release**

- Lane: `integration`; dependencies: T12.01, T12.02, T12.03, T12.04, T12.05.
- Requirements: R25, R47, R49, R50.
- Allowed paths: `docs/integrations/foi-cutover.json`, `scripts/check_task_t12_06.py`.
- Action: Produce the consumer acceptance packet and distinguish not_run, blocked and observed checks. Remote sibling changes and runtime qualification remain separately executed actions, not automatic from a document.
- Acceptance: Both declared fictional packet oracles pass; unsupported runtime and source ownership stay explicit. Independent evidence only; no legacy expected-output comparison.
- Verification: `python3 scripts/check_task_t12_06.py && python3 scripts/validate_scaffold.py`.
- Evidence: `conductor/tracks/12_ecosystem-reuse/evidence.md`.
- Detailed card: [`T12.06`](../../tasks/T12.06.md).

## [ ] **T12.07 — Define evidence-linked FOI-O contribution packets**

- Lane: `integration`; dependencies: T12.04.
- Requirements: R49, R50, R51.
- Allowed paths: `docs/integrations/foi-o-contribution.json`, `scripts/check_task_t12_07.py`.
- Action: Specify consumer-owned ontology candidates with exact base profile, evidence state, sanitized references, counterexamples, proposed competency tests and review status. Do not edit the ontology automatically.
- Acceptance: Both declared fictional packet oracles pass; unsupported runtime and source ownership stay explicit. Independent evidence only; no legacy expected-output comparison.
- Verification: `python3 scripts/check_task_t12_07.py && python3 scripts/validate_scaffold.py`.
- Evidence: `conductor/tracks/12_ecosystem-reuse/evidence.md`.
- Detailed card: [`T12.07`](../../tasks/T12.07.md).

## [ ] **T12.08 — Specify versioned FOI-O feedback and independent re-evaluation**

- Lane: `integration`; dependencies: T12.07.
- Requirements: R49, R51.
- Allowed paths: `docs/integrations/foi-o-feedback.json`, `scripts/check_task_t12_08.py`.
- Action: Define accepted/rejected/deferred feedback receipts and next-version regression/heldout checks. Prevent the data or generated labels used to propose an ontology change from also proving its validity.
- Acceptance: Both declared fictional packet oracles pass; unsupported runtime and source ownership stay explicit. Independent evidence only; no legacy expected-output comparison.
- Verification: `python3 scripts/check_task_t12_08.py && python3 scripts/validate_scaffold.py`.
- Evidence: `conductor/tracks/12_ecosystem-reuse/evidence.md`.
- Detailed card: [`T12.08`](../../tasks/T12.08.md).
