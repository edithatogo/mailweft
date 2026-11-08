# FOI-only consumer qualification and migration implementation plan

**Goal:** bounded tasks in `spec.md`. **Architecture:** preserve independent raw-email OCEL core.
**Tech stack:** Rust application; development-only document checkers. **Status:** all runtime tasks not started.
**Spec:** `spec.md`, `docs/FOI-SCOPE-AND-ROBUSTNESS.md`.

## Global constraints and review focus
Only foi-process is an active owned consumer. Do not confuse raw and rendered sources, inferred and observed events, source status and independent gold labels, packet validation and executed migration, or absent fields and known-empty data. Exact task prerequisites are authoritative; optional integration never gates core release.

- [ ] **T12.01 — Reproduce FOI defects and freeze the consumer boundary**
  - Depends on: T03.08. Lane: `integration`.
  - Files: `docs/integrations/foi-defects.md`, `scripts/check_task_t12_01.py`.
  - Acceptance: A fixture containing typed event values and time-varying object values is marked loss when the inspected adapter drops attributes.; A suspected bug without an executable reproduction remains suspected; no whole-system failure or corrected-runtime claim is allowed.
  - Verification: `python3 scripts/check_task_t12_01.py && python3 scripts/validate_scaffold.py`.
  - Interface and oracles: [task card](../../tasks/T12.01.md).

- [ ] **T12.02 — Audit FYI raw-message availability and source fidelity**
  - Depends on: T12.01. Lane: `integration`.
  - Files: `docs/integrations/fyi-source-fidelity.md`, `scripts/check_task_t12_02.py`.
  - Acceptance: An HTML page cannot be labelled raw_rfc5322 or assigned fabricated Bcc/transport headers.; Unavailable Bcc and missing attachment bytes remain unavailable rather than a known-empty population.
  - Verification: `python3 scripts/check_task_t12_02.py && python3 scripts/validate_scaffold.py`.
  - Interface and oracles: [task card](../../tasks/T12.02.md).

- [ ] **T12.03 — Specify the one-way FOI adapter and lossless projection**
  - Depends on: T12.01, T12.02, T10.01. Lane: `integration`.
  - Files: `docs/integrations/foi-adapter-contract.md`, `scripts/check_task_t12_03.py`.
  - Acceptance: Dependency graph is foi-process -> email library; no FOI domain crate is imported into core.; Required qualifier/attribute/history loss fails admission rather than silently emitting a reduced log.
  - Verification: `python3 scripts/check_task_t12_03.py && python3 scripts/validate_scaffold.py`.
  - Interface and oracles: [task card](../../tasks/T12.03.md).

- [ ] **T12.04 — Freeze independently adjudicated FOI shadow comparisons**
  - Depends on: T12.03, T11.01. Lane: `integration`.
  - Files: `docs/integrations/foi-shadow-protocol.md`, `scripts/check_task_t12_04.py`.
  - Acceptance: A disagreement with existing output is not automatically a new-library regression.; A platform status used as input cannot be reused as independent outcome truth; label dependencies are explicit.
  - Verification: `python3 scripts/check_task_t12_04.py && python3 scripts/validate_scaffold.py`.
  - Interface and oracles: [task card](../../tasks/T12.04.md).

- [ ] **T12.05 — Prepare reversible consumer migration and replay acceptance**
  - Depends on: T12.03, T12.04, T10.04. Lane: `integration`.
  - Files: `docs/integrations/foi-migration-packet.md`, `scripts/check_task_t12_05.py`.
  - Acceptance: The packet requires repeat/replay and correction tests that retain source IDs without duplicated independent events.; Missing consumer execution/rollback evidence leaves migration pending; no remote edit or production cutover is implied.
  - Verification: `python3 scripts/check_task_t12_05.py && python3 scripts/validate_scaffold.py`.
  - Interface and oracles: [task card](../../tasks/T12.05.md).

- [ ] **T12.06 — Qualify FOI cutover without coupling core release**
  - Depends on: T12.01, T12.02, T12.03, T12.04, T12.05. Lane: `integration`.
  - Files: `docs/integrations/foi-acceptance.md`, `scripts/check_task_t12_06.py`.
  - Acceptance: Absent required real test receipts yields defer, not acceptance.; Core task closure excludes FOI integration and real-data dependencies; no unrelated owned repo enters the active scope.
  - Verification: `python3 scripts/check_task_t12_06.py && python3 scripts/validate_scaffold.py`.
  - Interface and oracles: [task card](../../tasks/T12.06.md).
