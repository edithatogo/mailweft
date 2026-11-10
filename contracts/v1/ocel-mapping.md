# Mailweft OCEL 2.0 mapping contract — implemented Stage 1

Status: versioned specification of the **current** Rust 0.7 converter; this is not a claim of full normative OCEL 2.0 value-type or format parity.

## Authoritative observation and objects

| OCEL type | Meaning | Evidentiary limitation |
|---|---|---|
| `source_copy_observed` event | Source bytes observed at caller-supplied `observed_at` | Not a sent, delivered, read or completed event |
| `source_copy` object | Source-identifier/byte-hash identity | Not proof of a complete archive |
| `email_message` object | Candidate logical message identified by exact bytes | Different bytes are distinct even with same claimed Message-ID |
| `email_address` object | Mailbox/address identifier | Not proof of human identity |
| `claimed_message_identifier` object | Opaque hash of RFC Message-ID/References claims | Not evidence referenced messages exist |
| `mime_attachment` object | Parser-classified attachment part | Declared media type is not content verification |

## Qualified relationships

- Event-to-object: `message`, `source_copy`, `from`, `sender`, `to`, `cc`, `bcc`, `reply_to`, `claimed_message_id`, `claimed_in_reply_to`, `claimed_reference`, `attachment_part`.
- Object-to-object: `observed_as_copy`, `contains_attachment_part`, `reply_target_candidate`.
- No absent `Bcc` assertion is synthesized. Header roles are source claims, not human-task assignments. A `reply_target_candidate` is not a verified conversation or causality edge.

## Attributes and time

- Event string attributes: `source_sha256`, `date_claim_status`, optional `claimed_date_utc`, `message_id_claim_status`, `in_reply_to_claim_status`, `references_claim_status`, `attachment_count`.
- Object string attributes on `mime_attachment`: `declared_media_type`, optional `filename_sha256`, `decoded_byte_length`; all have explicit observation time.
- All current attribute types are **string**. Values such as attachment count and decoded byte length are strings, not native OCEL integer values. The `claimed_date_utc` attribute is a source assertion, not the OCEL event time.

## Conformance boundaries and loss ledger

The Rust OCEL validator and independent development-only Python validator check type declarations, object/event IDs, qualified references, attributes and histories, but neither is a normative schema certification. Rust4PM independently imports the JSON representation under an optional validation feature.

Known limitations are explicit: native integer/float/boolean/time attribute types are not emitted; JSON/XML/SQLite equivalence is unqualified; variable attribute history is only exercised for source-observed attachment metadata; some headers and body/attachment content are intentionally excluded from OCEL, with source-side evidence maintained separately. No import/export equality guarantee is asserted for all OCEL 2.0 values.

The release-security blocker for the optional Rust4PM dependency remains open. These mapping and structural tests do not authorize release.
