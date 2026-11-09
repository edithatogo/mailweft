# 0.3 contract migration — source-observed MIME attachment metadata

Mailweft's core remains an offline Rust raw-email-to-OCEL converter. This **experimental additive** projection adds attachment metadata and advances the Rust packages to `0.3.0`; `Conversion.schema_version` advances to `1.2.0`. The portable OCEL 2.0 root structure remains unchanged.

## Changes

- New `mime_attachment` object type, identified by a domain-separated hash of the exact-byte message candidate and parser MIME part position. A MIME part is **not** proof that an attachment was delivered or read.
- New event-to-object qualifier `attachment_part` and message-to-object qualifier `contains_attachment_part`; the existing observation event retains its original acquisition timestamp.
- Attachment object attributes are **time qualified** in OCEL 2.0: `declared_media_type`, `filename_sha256` (only when a filename is present), and `decoded_byte_length`. Their attribute time is the supplied observation time, not a document creation time.
- New observation-event string attribute `attachment_count` (including zero).
- Objects now expose typed `ObjectAttribute {name, time, value}` rather than the generic event attribute structure. Since earlier released converter outputs left all object attributes empty, this changes the Rust API but does not reinterpret historical object values.
- `convert_batch` merges time-varying object attribute histories deterministically. Conflicting values for the same name, object and observation time produce an explicit error rather than silent last-write-wins.

## Source protection and limits

All metadata are derived from supplied MIME bytes using the Stalwart parser; Mailweft does not execute, render or fetch attachment content. Raw filenames and attachment bodies are **not exported**. MIME types are source declarations, not content verification. The `decoded_byte_length` is the parser-reported length, not an antivirus/semantic quality assessment. A 16 MiB per-message input limit remains enforced; nested MIME complexity and malicious-input fuzzing need separate qualification.

Filename and object hashes are **pseudonymous, not anonymous**; a guessed filename can be hashed. Exact bytes and part position determine attachment identity, not content equality between different messages. Attachments present in nested forwarded emails may require more complete source-copy attribution later.

Existing OCEL consumers that rely on exact event/object counts must account for the new attachment objects and qualifiers, while event counts continue to reflect source-copy observations. Consumers that ignore object attributes will lose MIME metadata. Independent Rust4PM import acceptance is required for time-qualified attributes and relationships. This remains an experimental implementation, not proof of complete OCEL format parity.

## Non-goals

No attachment OCR, PDF/DOCX parsing, malware scanning, raw-content persistence, source completeness manifest, threaded business-event inference, GraphRAG or FOI ontology mapping is added. Enron remains an optional external verification corpus only; `foi-process` is the first downstream consumer under separate independent evaluation.
