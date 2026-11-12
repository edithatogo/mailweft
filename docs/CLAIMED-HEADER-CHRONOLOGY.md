# Claimed chronology and opaque message references (experimental)

## Boundary

Mailweft's observation event remains `source_copy_observed` at the caller-supplied `observed_at`. This is **not** a claim of email sending or delivery. The RFC 5322 `Date` header is a **source claim**, even when parsable. Never replace the event time with it or calculate a verified response interval from header dates alone.

The new extraction uses the existing qualified Stalwart `mail-parser` dependency (no regex reinvention of RFC parsing) and adds only deterministic projection policy. Header presence/ambiguity is counted on the parsed message's top-level headers; any missing, malformed, or repeated Date, Message-ID, References or In-Reply-To is recorded as `absent`, `malformed` or `ambiguous`.

## OCEL projection

- The event retains `source_sha256` and gains `date_claim_status` and, **only when a single valid Date is present**, `claimed_date_utc` as a string attribute. Its actual OCEL event `time` remains `observed_at`.
- `message_id_claim_status`, `in_reply_to_claim_status` and `references_claim_status` are typed string attributes with `parsed / absent / malformed / ambiguous` values.
- When well formed, message identifiers are represented by `claimed_message_identifier` objects with domain-separated SHA-256 identifiers. The source observation event has `claimed_message_id`, `claimed_in_reply_to` and `claimed_reference` qualified event-to-object relationships.
- Claimed identifiers **do not produce observed email messages** and do not unify distinct raw source messages merely because they have the same Message-ID field.
- Duplicate header instances are not silently selected, even if a parser exposes a last-value convenience method.
- The original raw header strings, offsets, address values and bodies are **not included** in the public OCEL output; they remain available only via the separate source bytes retained by the caller. The hashed projection is deliberately lossy and should never be described as a full evidentiary source archive.

## Security, limits and assumptions

- SHA-256 identifiers are deterministic and can leak equality; they are **pseudonyms, not guaranteed anonymisation**. An attacker with a likely Message-ID can potentially compute its hash. Secret-keyed IDs and per-tenant scoped mapping are a future privacy hardening task before using sensitive real correspondence.
- Normalisation strips outer angle brackets but does **not** case-fold the entire Message-ID. Values must be ASCII, contain an at-sign and contain no whitespace/angle brackets. These are pragmatic limits pending full RFC edge-case conformance testing.
- At most 1,000 identifiers from one References/In-Reply-To field are projected; excessive or malformed lists are rejected as a whole. No threading causality, verified receipt timestamp or content-based NLP is inferred.
- In a multi-mailbox batch, multiple source-copy observations may refer to the same claimed identifier object without asserting independent evidence or identical business processes.
- Enron remains an optional verification fixture. FOI mapping/legality remains in `foi-process` and reviewed `foi-o` candidate packets, not this converter.

## Validation

The synthetic tests in `tests/header_claims.rs` address timezone normalisation, missing and malformed dates, duplicate date ambiguity, claimed reply/reference relationships, no PII leakage, and identical Message-ID claims in distinct raw messages. Feature-gated `tests/rust4pm_interop.rs` independently imports exported OCEL with the additional typed attributes and qualified relationships. Run `cargo test --locked --workspace --all-targets` and `cargo test --locked -p mailweft-core --features rust4pm-validation --test rust4pm_interop`.

The next separate feature should use source-anchored attachment metadata and text-region extraction without retrieving URLs, decoding executable content, or letting NLP rewrite these observations.
