# Track 04: Synthetic and Enron raw-to-processed validation

**ID:** `04_data-validation`  
**Status:** Not started  
**Requirements:** R11 R13 R14 R24 R17  
**Dependencies:** Accept upstream contracts and completed dependencies from previous applicable tracks; see per-step plan.

## User story
As a maintainer of an offline, reusable raw-email-to-OCEL Rust library, I need synthetic and enron raw-to-processed validation so that Stage 1 or optional Stage 2 is reliable, testable, privacy preserving and independent of CareOps.

## Inputs
- `docs/CONTRACT.md`, `docs/REQUIREMENTS.md`, `docs/DESIGN.md`, `docs/DATASETS.md`, JSON contracts in `contracts/v1`.
- Earlier track's tested API and synthetic fixture outputs (not proprietary/raw Enron).

## Out of scope
- Live mailbox access; automatic data download or upload; production authorisation; external LLM calls; changing other repositories; broad unrelated refactoring.

## Acceptance
- Every step T04.xx has passing exact acceptance; non-sensitive evidence recorded.
- Formatting, clippy, tests and privacy checks passing for changed code; errors documented without fake success.
- No loss of evidence IDs or OCEL 2.0 semantics; no silently invented event assertions.
