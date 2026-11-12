# Track 03 — OCEL interoperability implementation checkpoint (10 November 2026)

This is an additive evidence record, **not** a replacement for the generated track plan or a declaration that the entire track is complete. Canonical task status and dependency graph remain in `conductor/task-manifest.json`.

## Implemented in this slice

- `contracts/v1/ocel-mapping.md` documents every current Stage 1 object/event type, attribute and qualified E2O/O2O relationship, with source-observation and claimed-header distinctions and an explicit list of unsupported native types and formats.
- `mailweft_core::ocel_validate::validate_ocel` rejects undeclared types/attributes, duplicate IDs, dangling qualified object links, duplicate relationships, conflicting time-qualified object attributes, duplicate event attributes and unsupported type declarations. It reports stable violation codes without silently repairing input.
- `crates/mailweft-core/tests/ocel_contract.rs` tests actual generated synthetic MIME OCEL, serde JSON roundtrip equality and deliberate corruption of type catalogues, object links, qualified multiplicity, attribute histories and unsupported type claims.
- Existing independent Rust4PM import and development-only Python structural checks remain in CI.

## Explicit remaining acceptance

T03.01: full Rust4PM API/format audit, including optional dependency advisory remediation.
T03.02: mapping vocabulary is now documented; task acceptance depends on its prerequisite T03.01 and final checks.
T03.03–T03.04: current Stage 1 JSON is implemented; native typed-value and full normative schema/export parity remain.
T03.05: string-only values and attachment observation history; full primitive types and all time-varying histories remain.
T03.06: current qualified relationships are tested; all OCEL relationship multiplicities and independent export roundtrips remain.
T03.07: object-centric mining algorithm execution remains unimplemented.
T03.08: the mapping document records known losses, but a machine-readable loss ledger and full JSON/XML/SQLite matrix remain unimplemented.

No task is marked complete by writing this checkpoint. No Enron/FYI or other real correspondence is processed. The independent Rust4PM quick-xml security issue remains an explicit release blocker. Exact CI run evidence must be added to the PR and verified after merge.
