# 0.7 migration — reconciliation, transfer maps and Conductor

Rust packages advance **0.6.0 -> 0.7.0**. The tested Rust toolchain is pinned to **1.99.1**; Cargo.lock changes only workspace versions and the CLI's explicit serde dependency edge (serde was already present). No new runtime package, model, cloud service or graph store is added.

Existing single-message/batch OCEL fields and identities remain unchanged; `Conversion.schema_version` remains **1.3.0**. SourceEnvelope/SourceReceipt remain **1.0.0**.

## New output contracts

- `mailweft reconcile selection.json [--allow-partial]` emits a selection report, not bare OCEL. Its optional `log` contains the OCEL; `manifest` accounts for every selected slot. Exit 2 means rejected sources remain even if partial projection was explicitly enabled.
- Selection request/report schema version **1.0.0**. Completeness assurance is limited to the supplied selection, not the underlying mailbox or FOI history.
- BodyEvidence advances **1.1.0 -> 1.2.0**, adding `transfer_map` with identity/verified_transfer/unavailable status. Encoded zones remain `derived`; the additional mapping records source units, not fictitious exact character positions.

## Conductor repair

The task manifest is now the canonical status source. Track plans, metadata, indexes, evidence summaries, setup summary and next-task ordering are rendered with `scripts/conductor_doctor.py --sync`. CI uses `--check`, which rejects drift and verifies the real pinned upstream checkout and command TOMLs. Root GEMINI.md and setup documentation point to the existing project; they do not instruct an agent to recreate it.

Current reconciled accounting: **14 completed, 12 in progress, 90 not started**, across the existing 116 tasks/15 tracks. Completed labels refer to their bounded acceptance contracts, not complete scientific requirements. All original requirements are retained. Specific dependency corrections decouple parser use from maildir discovery, timestamp claims from unrelated address edge cases, and Conductor setup from future security scanning.

Host-specific Gemini extension registration is **not verified on the user's machine**. The repository setup and pinned source are separately tested; linking instructions are documented without silently modifying a host.
