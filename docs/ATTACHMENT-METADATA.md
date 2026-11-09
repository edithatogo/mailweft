# Bounded MIME attachment metadata extraction

Use `mailweft-core::convert_eml(raw, source_id, observed_at)` or the bounded `convert_batch` Rust API, or `mailweft` / `mailweft batch` CLI. The Stalwart parser enumerates its classified MIME attachments; Mailweft emits `mime_attachment` OCEL objects, an observation event's attachment links, and message-to-part qualified relationships.

The converter captures **declared** media type, hashed filename (if present), and decoded byte length as attributes whose timestamp is the externally supplied acquisition observation. These are not verified document content, original authorship, successful transmission, or source document creation times.

No attachment payload, plaintext filename or full body is emitted; there is no network access, active content execution, macro evaluation, remote image load, HTML render, OCR or cloud model call. The original raw EML should be preserved by the caller alongside its hashes and rights/provenance receipt. The current filenames are pseudonymised with a stable, non-secret SHA-256 scheme and may be vulnerable to guessing; access-controlled source-side identity mapping is a future hardening task.

The batch merger deduplicates identical attributes and preserves attributes from observations at different times, failing on contradictory values for the same field and timestamp. Tests use invented MIME messages with base64-encoded `HELLO`, and confirm no attachment contents or filenames appear in OCEL. The optional Rust4PM feature tests that a standard parser imports the time-qualified object attributes.

This is a step toward, not completion of, attachment evidence handling. Future steps: safe content hashes, byte-offset mapping, MIME nesting and inline-vs-attachment policies, source receipt completeness, controlled document extraction adapters, and independent adversarial MIME/fuzz testing.
