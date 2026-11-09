# Source-envelope verification — Rust 0.6

Mailweft is still a corpus-neutral **email-to-OCEL converter written in Rust**. Source capture and preservation remain outside this library. This feature consumes a private, versioned envelope describing already supplied bytes; it does not retrieve mail, authenticate a sender, verify permission or certify an archive's completeness.

## API and CLI

- `mailweft_core::source::parse_envelope_json(metadata_bytes)` rejects malformed, duplicate/unknown fields, missing timestamps, unsupported versions, invalid hashes and unbounded references.
- `convert_with_envelope(raw, &envelope)` verifies actual byte length and SHA-256 before invoking the existing conversion. `platform_export` and `rendered_html` are representable source-fidelity labels but are **not admitted** to raw-email conversion.
- `mailweft convert-evidence source.eml source-envelope.json` emits `{conversion, receipt}`. This is a bundle, **not** a bare OCEL root; the portable OCEL is `conversion.log`. Existing single-file and batch commands still emit their original OCEL format.
- `read_bounded` and `read_bounded_file` enforce a counted limit during input, not only a metadata check. Every CLI input mode uses these readers.

See `contracts/v1/source-envelope.schema.json` and the invented example at `fixtures/synthetic/contracts/source-envelope.json`. The example's hash corresponds only to the exact 81-byte synthetic message in the tests; it is not a template hash for real correspondence.

## Metadata and what the receipt proves

Required metadata: version, source ID, byte digest/length, supplied observation timestamp, source-fidelity class, coverage claim, producer name/version, and rights/scope references. Optional custodian/locator references and ordered transformation references remain in the protected envelope. References are not fetched or executed. Runtime reference limits count UTF-8 bytes, which may be stricter than JSON Schema character limits.

The receipt binds source-copy, candidate-message and observation-event IDs to the matched bytes and a fingerprint of the complete typed metadata. It records the converter and pinned parser version. Private source IDs, locators, custodian names, rights references and body content are not copied into the receipt; fingerprints support later comparison with the separately retained envelope.

The assurance is explicitly `bytes_matched_metadata_caller_asserted`. A caller who changes both bytes and metadata can produce a new matching receipt. There is **no signature, trusted clock, access-control decision, source authenticity check or legal conclusion**. `complete_supplied_selection` is a caller claim about its supplied selection, not proof that all relevant correspondence is present. Unknown coverage stays unknown. Rights/scope references document a caller assertion; external policy must authorize processing.

Fingerprint contract: serde's struct field order, normalized UTC datetimes, and supplied transformation-list order define the version-1 metadata serialization. This is not a claim of generic RFC 8785 JSON canonicalization. JSON whitespace/property ordering does not alter the typed metadata fingerprint. Source IDs and acquisition times preserve existing observation identities; changing only ancillary metadata changes the receipt, not the OCEL.

## Bounds and privacy

- Raw message: 16 MiB maximum. Envelope input: 64 KiB maximum. A bounded reader consumes at most its limit plus one sentinel byte and returns no partial result on failure.
- Batch: 1,024 supplied files and 256 MiB of actual bytes. Files that grow after metadata inspection remain bounded.
- Explicit regular files only. Visible final-component symlinks and special files are rejected. This is **not** a directory sandbox or protection against a hostile process replacing path components between checks/opening. Use an authorized, isolated immutable snapshot; race-resistant descriptor-relative traversal remains future work.
- Hashes expose equality and can be guessed for low-entropy values. Receipts, zoning sidecars and source envelopes require access controls; none is advertised as anonymized public data.
- Stable errors include SOURCE_ENVELOPE_INVALID, SOURCE_VERSION_UNSUPPORTED, SOURCE_FIDELITY_UNSUPPORTED, SOURCE_LENGTH_MISMATCH, SOURCE_HASH_MISMATCH, SOURCE_SIZE_LIMIT, SOURCE_EMPTY, SOURCE_NOT_REGULAR, SOURCE_READ_FAILED and SOURCE_CONVERSION_FAILED. Diagnostics omit the offending private values.

## Tests and unresolved work

Tests cover altered bytes, mismatched hashes/lengths, missing/duplicate/unknown metadata, unsupported source classes, deterministic receipts, metadata revision, unchanged OCEL, non-regular files, and a growing/infinite reader. Development-only Python validates actual CLI receipts and zone output against Draft 2020-12 schemas. No Python is required by the converter runtime.

This is not a corpus reconciliation manifest: input/accepted/rejected counts, directory traversal, per-source exclusion reasons, publisher-specific FYI adapters, signed provenance and retention/deletion integration remain open. `foi-process` remains the first consumer; `fyi-cli` captures, the verified archive owner preserves, and `foi-o` reviews ontology contributions. Unvalidated legacy FOI output remains excluded as a correctness reference.
