# Optional repository reuse qualification implementation plan

**Status:** all implementation tasks not started. **Goal:** the bounded specification in `spec.md`.
**Architecture:** retain independent Rust raw-email -> OCEL 2.0 core; optional lanes do not gate core.
**Tech stack:** qualified Rust dependencies; Python helpers only for development-document validation.
**Spec:** `spec.md`, `docs/CONTRACT.md`, `docs/FRONTIER-DESIGN.md`.

## Global constraints and review focus
No real email/source data in Git; no network/model fallback; no participant-role/authority conflation; preserve source attribution and OCEL attributes/qualifiers; observe exact task prerequisites, not whole-track gates. Test unknowns, malicious data, copied quotations, Unicode spans and missing/ambiguous references where applicable.

The manifest is authoritative for dependencies and lane. Checkboxes are completion records, not approval requests. Planning checks do not mark implementation complete.

- [ ] **T12.01 — Qualify FOI replay patterns and full-fidelity Rust4PM boundary**
  - Lane: `integration`. Depends on: T03.08.
  - Edit only: `docs/integrations/foi-process.md`, `scripts/check_task_t12_01.py`.
  - Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
  - Acceptance: Fixture with event string/number values and time-varying object attribute fails if copied adapter drops attributes.; Snapshot sink is not advertised as revision/retraction store; no downstream code changed.
  - Verification: `python3 scripts/check_task_t12_01.py && python3 scripts/validate_scaffold.py`.
  - Evidence: `conductor/tracks/12_ecosystem-reuse/evidence.md`. Commit: `T12.01: Qualify FOI replay patterns and full-fidelity Rust4PM boundary`.
  - Exact interface and two oracle tests: [task card](../../tasks/T12.01.md).

- [ ] **T12.02 — Qualify Fe Reader and CiteWeft attachment evidence contracts**
  - Lane: `integration`. Depends on: T05.05, T05b.03.
  - Edit only: `docs/integrations/documents.md`, `scripts/check_task_t12_02.py`.
  - Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
  - Acceptance: Unsupported format/OCR/geometry produces unsupported, not synthesized extraction.; Provider output includes source hash and rendition/geometry mapping with exact/approximate fidelity.
  - Verification: `python3 scripts/check_task_t12_02.py && python3 scripts/validate_scaffold.py`.
  - Evidence: `conductor/tracks/12_ecosystem-reuse/evidence.md`. Commit: `T12.02: Qualify Fe Reader and CiteWeft attachment evidence contracts`.
  - Exact interface and two oracle tests: [task card](../../tasks/T12.02.md).

- [ ] **T12.03 — Map SourceRight and Searchright evidence conventions**
  - Lane: `integration`. Depends on: T10.01.
  - Edit only: `docs/integrations/evidence.md`, `scripts/check_task_t12_03.py`.
  - Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
  - Acceptance: Reference/provider verification never changes an email claim into verified task completion.; Export to a consumer with insufficient scope is refused; summary evidence lineage is retained.
  - Verification: `python3 scripts/check_task_t12_03.py && python3 scripts/validate_scaffold.py`.
  - Evidence: `conductor/tracks/12_ecosystem-reuse/evidence.md`. Commit: `T12.03: Map SourceRight and Searchright evidence conventions`.
  - Exact interface and two oracle tests: [task card](../../tasks/T12.03.md).

- [ ] **T12.04 — Define optional NLP/RCA consumer contracts**
  - Lane: `integration`. Depends on: T09.08.
  - Edit only: `docs/integrations/nlp-rca.md`, `scripts/check_task_t12_04.py`.
  - Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
  - Acceptance: Rust default-feature build and fixture execution require no Python/model service.; RCA receives source/claim distinction and cannot silently overwrite source events.
  - Verification: `python3 scripts/check_task_t12_04.py && python3 scripts/validate_scaffold.py`.
  - Evidence: `conductor/tracks/12_ecosystem-reuse/evidence.md`. Commit: `T12.04: Define optional NLP/RCA consumer contracts`.
  - Exact interface and two oracle tests: [task card](../../tasks/T12.04.md).

- [ ] **T12.05 — Document downstream CareOps and viewer interoperability**
  - Lane: `integration`. Depends on: T08.01, T03.08.
  - Edit only: `docs/integrations/consumers.md`, `scripts/check_task_t12_05.py`.
  - Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
  - Acceptance: Absent all downstream repos, core raw-to-OCEL fixtures still run.; Fork SHA/API/patch status recorded before adoption; no assumption of upstream parity or UI compliance.
  - Verification: `python3 scripts/check_task_t12_05.py && python3 scripts/validate_scaffold.py`.
  - Evidence: `conductor/tracks/12_ecosystem-reuse/evidence.md`. Commit: `T12.05: Document downstream CareOps and viewer interoperability`.
  - Exact interface and two oracle tests: [task card](../../tasks/T12.05.md).

- [ ] **T12.06 — Decide reuse after bounded evidence, without new core coupling**
  - Lane: `integration`. Depends on: T12.01, T12.02, T12.03, T12.04, T12.05.
  - Edit only: `docs/integrations/qualification.md`, `scripts/check_task_t12_06.py`.
  - Action: Execute the task card using synthetic input, a failing regression, minimal implementation and actual verification; do not widen scope.
  - Acceptance: README claims cannot be labeled fresh runtime qualification; exact test receipt required.; All core task dependencies remain core after adapter planning; no optional integration becomes a release gate.
  - Verification: `python3 scripts/check_task_t12_06.py && python3 scripts/validate_scaffold.py`.
  - Evidence: `conductor/tracks/12_ecosystem-reuse/evidence.md`. Commit: `T12.06: Decide reuse after bounded evidence, without new core coupling`.
  - Exact interface and two oracle tests: [task card](../../tasks/T12.06.md).
