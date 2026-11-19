# Track 03: Canonical OCEL 2.0 projection and Rust4PM verification

**ID:** `03_ocel`  
**Status:** Not started  
**Requirements:** R07 R08  
**Dependencies:** Accept upstream contracts and completed dependencies from previous applicable tracks; see per-step plan.

## User story
As a maintainer of an offline, reusable raw-email-to-OCEL Rust library, I need canonical ocel 2.0 projection and rust4pm verification so that Stage 1 or optional Stage 2 is reliable, testable, privacy preserving and independent of CareOps.

## Inputs
- `docs/CONTRACT.md`, `docs/REQUIREMENTS.md`, `docs/DESIGN.md`, `docs/DATASETS.md`, JSON contracts in `contracts/v1`.
- Earlier track's tested API and synthetic fixture outputs (not proprietary/raw Enron).

## Out of scope
- Live mailbox access; automatic data download or upload; production authorisation; external LLM calls; changing other repositories; broad unrelated refactoring.

## Acceptance
- Every step T03.xx has passing exact acceptance; non-sensitive evidence recorded.
- Formatting, clippy, tests and privacy checks passing for changed code; errors documented without fake success.
- No loss of evidence IDs or OCEL 2.0 semantics; no silently invented event assertions.

## Serde and format dependency decision (18 November 2026)

**No Serde migration is indicated by the current implementation.** `mailweft-core` and CLI already derive `Serialize`/`Deserialize` for OCEL 2.0 structures and use `serde_json` as the JSON format. `mail-parser` implements RFC 5322/MIME decoding, while optional `process_mining` (Rust4PM) validates/interoperates with event logs. Neither parser nor the OCEL semantic validator should be replaced by Serde, which supplies type-serialization traits, not mailbox or process semantics.

Next scoped tests within the **existing track 03**, following `conductor/task-manifest.json` as canonical (do not edit generated `plan.md`/registry directly):
- Frozen OCEL compatibility tests for object/event IDs, qualifiers, timestamps/time zones, attribute types, duplicate keys, unknown event kinds, loss ledger and deterministic JSON bytes where those bytes are a published contract.
- Property/fuzz tests for malformed EML/MIME, malicious/oversized JSON and attachment-source evidence; preserve Stage 1 observation vs delivery/business-action claims.
- Compare optional Rust4PM read/write/round trips and its pinned XML dependencies independently. The existing quick-xml advisory/compatibility gate remains unresolved until its separately owned qualifying PR passes; Serde does **not** remediate XML parser advisories.
- Open manifest-owned implementation leaves only if tests expose a specific defect; retain no-go otherwise. Do not claim any action/test has been executed by adding this specification.
