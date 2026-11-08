# Product and interoperability contract — v0.1 proposal

## Purpose
Convert a **raw RFC 5322/MIME message corpus** into reproducible, correctly typed **OCEL 2.0** communications data; optionally derive separately versioned and reviewable semantic interpretations of email and attachment contents. Generic, independent Rust library with no implicit healthcare or Microsoft dependencies.

## Interfaces
- **Input**: `SourceEnvelope` from raw EML/maildir/mbox (optional later), containing byte reference, SHA-256, source/custodian ID, acquisition timestamp, source location reference, rights/scope, parser version. Never require preprocessed datasets.
- **Stage 1 intermediate**: `ObservedMessage` and `AcquisitionManifest` per `contracts/v1`. Capture normalized fields plus raw parse provenance. `header_date` is the timestamp **asserted by the message header**; `observed_at` is ingestion time; `source_received_at` only where the source actually supports it.
- **Stage 1 output**: native **OCEL 2.0** JSON/XML/SQLite data, with deterministic events, objects, E2O qualifiers and O2O relationships, plus private source/evidence sidecar and reconciliation receipt. A message's presence in an archive is observable; the header-date event is a separate *asserted chronology*, never independent proof of sender intent or delivery. Separate archival observation chronology from claimed communication chronology; choose event-view policies explicitly.
- **Stage 2 input**: authorised, local-only access to raw body and attachment streams, keyed to Stage 1 evidence IDs, with source security policy. Not enabled in core.
- **Stage 2 output**: `ExtractionClaim` objects and graph/evidence sidecar; only accepted typed semantic projections enter an enriched OCEL with separate identifiers. Claims are not authoritative decisions or verified business events.
- **Mining API**: compatible OCEL 2.0 log accepted by `process_mining` (Rust4PM) and independently tested against PM4Py in optional validation. Neither graph inference nor PM4Py is required for producing OCEL.

## Identity and deduplication
- Distinguish `SourceCopy` (one mailbox/custodian/file observation), `LogicalMessage` (candidate globally deduplicated message) and `Conversation` (possibly inferred grouping). Use a cryptographic content hash + source-specific identifiers + headers, never Message-ID alone as a globally unique key.
- Stable deterministic IDs are domain-separated and versioned; IDs cannot contain raw personal information. Store optional reversible mappings only in a protected, separately authorised source vault.
- Preserve distinct sender/to/cc/bcc roles where actually observed. An email to five people does not imply five task assignments; unknown/missing recipients are not fabricated.
- Mark exact vs probable duplicates, missing references, subject-only links and ambiguous identities as distinct evidence states.

## OCEL 2.0 preservation
- Do not collapse an event linked to multiple objects into a single case ID. No destructive timestamp rounding, qualifier flattening, attribute-type coercion, or erasure of object attribute histories.
- Round-trip read/write preserves event IDs/types/timestamps/attributes, object IDs/types/attribute changes, E2O qualifiers, O2O qualifiers, and relation multiplicities, subject to explicit OCEL format limits. Report any unrepresentable field in a loss ledger.
- All process-specific confidence, source spans, graph assertions, correction history and model versions live in a versioned sidecar with links to exported OCEL IDs. Stage 1 never mutated by Stage 2.

## Provenance and uncertainty
- Every interpretation has `source_evidence_ids`, rule/model/ontology versions, extraction time, claimed event time (optional), negation/modality, confidence (optional), `observation_state`, and review status.
- States: observed-source, asserted-by-actor, inferred, corroborated, contradicted, rejected. These must not be silently promoted.
- An extracted claim such as 'approved yesterday' is not a verified `approval_decided` event. A task completion email does not establish that the task is complete.

## Scope and safety
- Offline by default; no automatic source downloads, external embeddings, API calls, cloud LLMs or data upload. Raw input is untrusted content. Resource limits on size, nesting, decompression and attachment types.
- File and attachment bytes never logged. Reject path traversal, unsafe URLs, remote HTML references and executable attachments. Permit raw content reading only in an approved scope and transient processing boundary.
- Public repository contains synthetic fixtures only; no real mailbox content or unlicensed upstream code. License original work Apache-2.0; preserve notices.

## Success contract
- Rust-only Stage 1 runs on 100 synthetic fixtures without Python and produces valid OCEL 2.0; all fixture IDs/qualifiers/attributes survive validated round trips.
- Independent engine reads exported logs; repeat runs on same bytes/config yield byte-identical canonical output and receipt after excluding declared run-time fields.
- Enron processed datasets are independent **benchmarks only**; exact-population matching is required for record-level comparisons. Otherwise publish structural/aggregate comparisons labelled non-paired.
- No Stage 2 action changes Stage 1 raw observation records; unknown findings are abstentions.
