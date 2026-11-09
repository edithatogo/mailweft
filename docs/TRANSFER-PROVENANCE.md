# Transfer-unit provenance — Mailweft 0.7

BodyEvidence 1.2.0 adds a `transfer_map` to each accepted plaintext part. This relates parser-decoded UTF-8 analysis bytes to transfer-encoded source units. It does **not** convert transformed zones into invented exact character offsets.

## Admission and interpretation

The maintained mail-parser produces decoded content. Mailweft's mapping layer reconstructs only a bounded layout and admits it only when its bytes exactly agree with that parser output and the analysis text. Supported cases:

- Identity: unchanged 7bit/8bit/binary UTF-8 bytes, including multibyte characters. Positions remain byte offsets, never Unicode character counts.
- Base64: each source quartet maps to one to three decoded bytes. Ignored MIME whitespace is recorded as source-only ranges. The quartet's raw bounding range can include internal whitespace; its relation is not one-to-one.
- Quoted-printable: literal runs and hex escapes map to decoded bytes; soft line breaks are recorded separately as source-only ranges with no decoded text contribution.

Malformed decoding, duplicate encoding fields, unsupported encoding, noncanonical padding, layout disagreement, charset transformation, offset overflow or resource exhaustion return `status=unavailable` with a reason and no admitted units. They are not silently repaired into precise source spans.

When the transfer was non-identity, existing zone `offset_quality` remains `derived` and raw zone offsets remain null. A consumer can cite the enclosing verified source units, but cannot pretend a decoded character corresponds to a single original byte. General charset/HTML mapping remains open.

## Limits

At most 32,768 total mapping units/source-only ranges are admitted per inspected message. Existing 16 MiB raw message, 2 MiB decoded part and 65,536 zone bounds remain. Mapping failures do not discard existing decoded zones or their source-part hashes. The pure mapping function expects parser-produced, bounded input; use `inspect_body_zones` as the public ingestion boundary.

No source text, filename, private address or payload is introduced into the sidecar. Coordinates, digests and equality patterns are still sensitive metadata; this is not anonymization.

## Tests

Synthetic fixtures check base64 quartets with line folding, quoted-printable escapes/soft breaks, malformed base64, charset transformation abstention and multibyte UTF-8 identity mapping. Actual CLI sidecars are schema-validated. These fixtures establish specific invariants, not comprehensive MIME fuzzing or an external-corpus accuracy result.
