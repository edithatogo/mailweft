# Executable implementation checkpoint — first Rust converter slice

Date: 2026-11-09. Scope: branch `feat/rust-eml-ocel-slice`, not a release.

## Executed, independently testable code

- Cargo workspace with `mailweft-core` and `mailweft-cli` (Rust 2024, MSRV 1.98 for mail-parser 1.0.1).
- One supplied raw EML source copy is parsed offline with maintained `mail-parser`.
- Stable, domain-separated SHA-256-derived message-candidate, source-copy and address IDs; no raw address/source locator in public OCEL.
- Sender/From/To/Cc/Bcc/Reply-To are distinct event-to-object qualifiers **only when observable**; address is not equated with a person or tasked actor.
- One `source_copy_observed` event at caller-provided acquisition time; no inferred sent, delivered, read or completed event.
- OCEL 2.0 JSON-shaped event/object types, typed event attributes, event-to-object and object-to-object qualifiers. Source hash included, no source bodies.
- `mailweft` executable converts `<raw.eml> <observed-at:RFC3339> [source-id]` to stdout JSON; bounded to 16 MiB.
- An optional `rust4pm-validation` feature adds a pinned Rust4PM 0.6.2 independent import test. It is not a default runtime dependency.
- Synthetic tests cover roles, no Bcc fabrication, separate source copies, deterministic output, invalid input, and separate observation event IDs for distinct acquisition times.

## Validation evidence discipline

The initial standard Rust test suite passed in GitHub Actions. The Rust4PM independent import test also passed in the hosted run prior to a formatting failure (run 42527779231, stage Rust4PM import succeeded); source formatting was subsequently applied. Refer to the latest exact-commit `Rust converter checks` workflow for the full test/format/clippy status; do not extrapolate from earlier runs.

**Not yet implemented:** header-date claimed chronology, thread references, mailing-list/group normalization, source-specific receipt manifest, full attachment metadata/bytes handling, maildir/MBOX, full multi-message OCEL log/merge, qualified export roundtrip for object attribute histories, semantic zoning, local NLP, GraphRAG, FOI consumer adapter, ontology candidate packets, Enron benchmark, licensed dataset acquisition, security/property fuzzing, release packaging.

## Next coding-agent tasks (core first)

1. Qualify OCEL serialization against official schema and Rust4PM roundtrips with all typed attributes/qualifiers (currently only one message/observation fixture). Prevent dropping either E2O or O2O values.
2. Build deterministic multi-message batch conversion and safe merging: same message in multiple mailbox copies shares a candidate logical ID but retains every source-copy observation. Observe duplicate/revision policies before introducing semantic dedup.
3. Preserve claimed Date/References/In-Reply-To as evidence with explicit accuracy and origin; do not equate these with reliable transport times or complete threads.
4. Add attachment metadata and source-span mapping without executing or logging unsafe content. Keep semantic parsing behind optional features and independent access controls.
5. Add source/projection loss ledger and errors for unsupported formats, damaged MIME, unknown source times, missing headers and large inputs. Continue synthetic test-first.
6. Only then add `foi-process` consumer (issue #153) and reviewed FOI-O feedback (issue #195), explicitly excluding legacy FOI output as correctness oracle.
7. Evaluate bounded deterministic graph traversal before optional GraphRAG; leave its model/graph dependency outside the core release.

## Conductor accounting

This checkpoint does not mark all of T00/T01/T03 complete. It overlaps a small vertical slice of foundation, EML parsing, participant roles, CLI and OCEL projection; detailed tasks requiring manifests, source receipt completeness, formal fidelity and format parity remain open. An agent must record exact tested commits and evidence before changing task checkboxes.

The core contract stays: **a Rust email-to-OCEL converter**. Enron is an external example/verification corpus, not a product dependency.

## Second implementation checkpoint: bounded multi-source assembly

Branch `feat/multi-eml-batch` adds `SourceInput` and `convert_batch`, merging source-copy OCEL observations deterministically. Exact duplicate observations are idempotent; identical raw messages seen in two custodians share one provisional message object but retain two copy objects, events and O2O qualifiers. Input ordering does not change the exported log. The CLI supports `mailweft batch <RFC3339 observed_at> <file.eml>...` with explicit bounded inputs.

Hosted Rust converter checks **passed** for commit `3e88747e2f0ea4b4766d71cc0847a3a3ed612d53`, including unit/integration tests, explicit batch acceptance, Rust4PM 0.6.2 import of the combined OCEL, rustfmt and clippy -D warnings (run 42528662115). Planning tests passed at that stage. The later temporary CI helper removal and documentation changes must be verified independently; they do not modify converter source.

This does not complete the Conductor source-manifest, header chronology, MIME attachment, source validation, full OCEL attribute/history or dataset benchmark tracks. The Conductor track task registry remains conservative; no blanket track completion is claimed. See `docs/BATCH-IMPLEMENTATION.md`.

## Third implementation checkpoint: claimed message chronology and references

Branch `feat/claimed-header-chronology` adds explicit Date, Message-ID, In-Reply-To and References *claims*. Observation event time is still caller-supplied and immutable. A uniquely parsed Date is exposed as `claimed_date_utc`, separate from `source_copy_observed.time`; missing, malformed or duplicate headers are encoded through typed statuses. Plausible parsed Message-ID and reply/reference values become hashed `claimed_message_identifier` objects linked by qualified E2O relationships; they **do not** create synthetic observed message events or conflate byte-distinct source copies.

The library and CLI advance to `0.2.0`, and the `Conversion` API schema version advances to `1.1.0`. See `docs/MIGRATION-0.2.md` and `docs/CLAIMED-HEADER-CHRONOLOGY.md` for the additive changes, evidence-fidelity boundaries and pseudonym re-identification limitations. Tests cover RFC timezone conversion, malformed and duplicate dates, missing fields, opaque references, identical claimed IDs in distinct raw messages, and independent Rust4PM import of the added metadata.

This checkpoint intentionally does **not** mark the complete threading/chronology tracks done. No source span ledger, verified delivery time, attachment metadata, message-level matching or cross-source independent corroboration is implemented. All run statuses must refer to an exact commit; planning/CI checks do not prove evidentiary truth or production readiness.

## Fourth implementation checkpoint: safe MIME metadata and object attribute history

Branch `feat/mime-attachment-metadata` uses Stalwart's parsed attachment parts to create `mime_attachment` OCEL objects and qualified event/object and message/object links, with source-observed (not authored/delivered) times. Object attributes now carry a time-qualified `declared_media_type`, `filename_sha256` and `decoded_byte_length` where present. Raw filenames, email bodies and attachment bytes are not emitted. The source-copy event also carries a typed `attachment_count` string field.

The batch merger deterministically unions time-varying object attributes and rejects two different values for the same object/name/time, rather than silently dropping an attribute history. This is an additive experimental contract version change: crates `0.3.0`, `Conversion.schema_version=1.2.0`. See `docs/MIGRATION-0.3.md` and `docs/ATTACHMENT-METADATA.md` for the data fidelity and privacy boundaries.

The new synthetic tests cover parser-classified attachment parts, metadata exclusion of raw filenames/payloads, no fabricated attachment for a plain body, and temporal history merging across two observations. Rust4PM import tests evaluate the time-qualified attributes. The code does **not** yet decode attachments for semantic analysis, validate their declared MIME types, perform OCR, or claim full MIME nesting/security or OCEL historical parity; no real corpus was used. CI status must be recorded from exact final commits before treating this checkpoint as verified.

## Fifth implementation checkpoint: non-destructive source zoning and OCEL declaration correction

Branch `feat/source-spans-zoning` adds `mailweft-core::inspect_body_zones` and `mailweft inspect raw.eml`. The independent `BodyEvidence` sidecar reports a raw source hash, MIME part raw-byte bounds/hash, decoded content hash and length, decoding problems and ordered line-level functional zone labels. Zone `raw_start/raw_end` are available **only** for exact source/decoded byte matches; encoded or re-decoded content has `derived` status with no fabricated raw offsets. This does not alter OCEL events or claim new business activity. HTML fallback is explicitly not taken as source plaintext. Synthetic tests cover provenance, no text leakage, missing material, invalid sources and unchanged OCEL output.

Also repairs a type-catalogue defect: emitted `mime_attachment` objects must have a declared OCEL `objectTypes` entry and typed attribute definitions. Version changes: Rust crates `0.4.0`, `Conversion.schema_version=1.3.0`. See `docs/MIGRATION-0.4.md` and `docs/SOURCE-PROVENANCE-AND-ZONING.md`.

This is a **partial precursor** to T05b.01–03, not completion of the planned nested structure, source map or multi-class evaluation. Source-envelope, rights/coverage, precise encoded-span mapping, signatures in forwarded blocks, NLP, GraphRAG and FOI consumer work remain outstanding. Run evidence must be recorded for the exact merged Git commit.
