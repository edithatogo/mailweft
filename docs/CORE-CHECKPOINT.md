# First native converter checkpoint

## Implemented architecture

Two packages: `crates/mailweft-core` (public API and internal identity/parser/projection/pipeline modules) and `crates/mailweft-cli` (local adapter). This is the accepted minimal core+CLI shape, not seven publishable microcrates. Existing `crates/mailweft/` plan paths mean the core package and are reconciled accordingly. Stable schema IDs are unchanged.

`convert(&[RawEmail], &Config) -> Result<Conversion, Error>` is all-or-nothing. Inputs carry raw bytes, a stable source locator, caller-observed time and an explicit permission classification. No global clock, filesystem modification-time inference, network call or external process enters conversion.

Defaults: 8 MiB/message, 64 MiB/batch, 1,024 messages, 1,024 MIME parts/message and depth 16. Limits are nonzero and configurable in the Rust API. The CLI retains bounded defaults. Damaged MIME/decoding and exhausted parser limits fail explicitly rather than returning silently truncated evidence.

Objects: Message, SourceCopy, MailboxIdentity, Attachment. Header roles retain from/sender/to/cc/bcc/reply_to separately. Event views explicitly choose `message_archived_observed`, `message_header_date_asserted`, or both. Header date is not independently verified sending/delivery. MailboxIdentity is not an inferred verified Person.

Raw SHA-256 identifies exact content. HMAC-SHA-256 with caller-supplied key, explicit scope and length-framed domains produces pseudonymous identifiers. Copies retain distinct locators; exact raw-byte copies share a message candidate. Conflicting content or observation metadata for one source reference fails. The CLI hashes canonical local paths; moving a file changes its copy locator identity, not its raw-content identity within the same key/scope.

Attachments retain MIME type, bounded decoded byte count, keyed filename identifier and decoded-content hash. Attachment bytes are not serialized, executed, expanded as archives, OCR'd or semantically interpreted. Nested emails are marked unprojected rather than misattributed to the outer sender.

JSON output uses Rust4PM's full OCEL structs. Types, event attributes, object attribute timestamps and E2O/O2O qualifiers are retained. Native round-trip tests are not a claim of independent PM4Py/all-format parity.

## Evidence and test boundaries

The initial 19 tests actually failed against the compiling stub before implementation. CLI and MIME had separate observed red checkpoints. A later hardening test revealed that equal instants with different source offsets were silently considered identical; the fix compares the offset-bearing representation and rejects SOURCE_CONFLICT.

The test suite contains 37 native tests on Unix (36 on Windows; one Unix-specific symlink case). It includes 256 bounded malformed-byte inputs, not a sustained fuzzing campaign. See exact-head CI and the execution ledger for actual results; a test count is not proof of arbitrary MIME safety or scientific extraction accuracy.

`scripts/check_runtime_schema.py` invokes the real CLI offline after dependencies are built, validates observed-message and acquisition-manifest contracts, independently hashes the raw fixture, and checks chronology counts, privacy sentinels and referential integrity. Existing 17 planning-validator tests/11 schemas remain separate.

## Explicit limitations

- No maildir/directory/MBOX/PST/MSG ingestion or resumable streaming yet; bounded batches and individual EML files only.
- No References/In-Reply-To graph reconstruction, near-duplicate resolution, legacy Exchange address rules or cross-message identity inference. Such evidence is retained by raw source reference and flagged, not fabricated.
- Header-group structure, resent blocks and nested-message event projection remain incomplete and mark partial coverage. Missing dates never get a substitute sent time.
- No salutation/signature zoning, body/attachment NLP, GraphRAG, models, FOI consumer implementation or ontology promotion.
- No independent multi-format OCEL/PM4Py qualification, process-mining algorithm benchmark or arbitrary object-history import. Generated attributes preserve their explicit times; this is not a generic OCEL editor.
- No real Enron/FYI data was downloaded or processed. Enron remains a verification/example corpus.
- Pseudonymous output and hashes remain potentially linkable/sensitive; privacy tests are not an anonymisation guarantee.
- The CLI rejects static symlinks/traversal and caps actual bytes read, but is not a proven race-free sandbox against a concurrent hostile filesystem. Use trusted, quiescent input locations; descriptor-relative race-resistant IO remains a hardening task.
- Validation failures emit no JSON. A failed stdout write can still leave a partial stream; downstream consumers must parse/verify the complete bundle before admitting it.

## Security/release gate

[Issue #4](https://github.com/edithatogo/mailweft/issues/4) records quick-xml 0.37.5 through process_mining 0.6.2 and RUSTSEC-2026-0194/0195. Patched quick-xml is >=0.41.0, outside the upstream 0.37 constraint. The current JSON converter does not call XML parsing, but full reachability is unproven. No ignore entries or continue-on-error were added. This checkpoint is not released or merged as production-ready.

Next priority: qualify an upstream-compatible patched dependency; then implement bounded directory/source manifests and header threading with independent synthetic fixtures before higher-level NLP.
