# Track 01: Raw EML and maildir input, manifests and parse boundaries

**ID:** `01_ingestion`  
**Status:** Not started  
**Requirements:** R01 R02 R03 R09 R11  
**Dependencies:** Accept upstream contracts and completed dependencies from previous applicable tracks; see per-step plan.

## User story
As a maintainer of an offline, reusable raw-email-to-OCEL Rust library, I need raw eml and maildir input, manifests and parse boundaries so that Stage 1 or optional Stage 2 is reliable, testable, privacy preserving and independent of CareOps.

## Inputs
- `docs/CONTRACT.md`, `docs/REQUIREMENTS.md`, `docs/DESIGN.md`, `docs/DATASETS.md`, JSON contracts in `contracts/v1`.
- Earlier track's tested API and synthetic fixture outputs (not proprietary/raw Enron).

## Out of scope
- Live mailbox access; automatic data download or upload; production authorisation; external LLM calls; changing other repositories; broad unrelated refactoring.

## Acceptance
- Every step T01.xx has passing exact acceptance; non-sensitive evidence recorded.
- Formatting, clippy, tests and privacy checks passing for changed code; errors documented without fake success.
- No loss of evidence IDs or OCEL 2.0 semantics; no silently invented event assertions.
