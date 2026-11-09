# 0.2 contract migration — claimed RFC header metadata

This is an **experimental additive change** to the initial Mailweft Stage 1 output; not a published stable API or a claim that the converter is feature complete.

## Versioned changes

- `mailweft-core` and `mailweft-cli` now declare version `0.2.0` (from `0.1.0`).
- The `Conversion.schema_version` for the Rust API increments from `1.0.0` to `1.1.0`, to flag the added interpretation-related projection.
- Portable OCEL 2.0 JSON itself retains its standard root structure (`objectTypes`, `eventTypes`, `objects`, `events`).
- Existing observation events remain `source_copy_observed` with time equal to the **supplied acquisition observation**, unchanged by the email Date header.
- New optional event attributes: `claimed_date_utc`, `date_claim_status`, `message_id_claim_status`, `in_reply_to_claim_status`, and `references_claim_status`. Statuses are `parsed`, `absent`, `malformed` or `ambiguous`.
- New `claimed_message_identifier` object type and `claimed_message_id`, `claimed_in_reply_to`, `claimed_reference` event-to-object qualifiers.
- No inferred email-sent/delivered/completion event, no automatic timeline correlation and no FOI-specific interpretation.

## Migration obligations

Consumers **must not** rely on exact event/object counts if claimed identifier headers are present. Consumers that whitelist event type attributes should extend their whitelist to the named fields. Do not treat a claimed Date as the mining timestamp, or a reference identifier as proof that the referenced message is in the observed corpus.

The hashed identifiers are equality-preserving pseudonyms, not anonymisation or evidence that two source messages are identical. A complete source receipt and separate text/byte span ledger are still future work. For now, callers remain responsible for preserving the original raw message and its evidence chain.

Consumer validation must include malformed and duplicate headers, private identifier non-disclosure, and a Rust4PM import test for new event/object types and qualifiers. The FOI integration stays separately governed by `foi-process`; its historic outputs are not a validation oracle.
