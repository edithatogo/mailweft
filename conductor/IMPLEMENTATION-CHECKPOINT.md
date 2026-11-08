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
