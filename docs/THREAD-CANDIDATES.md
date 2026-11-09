# Conservative observed-message threading (experimental Rust 0.5)

## Core contract

`convert_batch` now creates an **optional, unverified O2O candidate** relationship between observed `email_message` objects when:

1. A child's source contains a plausible, uniquely parsed `In-Reply-To` Message-ID claim.
2. The claimed identifier matches the `Message-ID` claim of **exactly one distinct observed logical message** in that batch.
3. The child and unique candidate parent are different objects.

The relationship qualifier is `reply_target_candidate`, not `verified_reply_target` and not `received`, `sent` or `completed`. This is a candidate correspondence graph extracted from header claims. The existing source-observation timestamps remain independent of claimed header times.

Missing target: no edge. Multiple distinct observed messages claiming the same Message-ID: no edge. Several `In-Reply-To` identifiers: no direct parent edge. Self-link: omitted. `References` alone: no direct parent edge. Copies of an identical raw message in multiple custodians do not create duplicate candidate relationships. The results are deterministic regardless of source input order.

The independent Rust4PM validator must import the resulting object-to-object relationship without dropping its qualifier. Tests use fictional `example.invalid` IDs; no Enron corpus or FOI dataset is required.

## Limits

The *source* of the claim is the untrusted email header. Headers may be forged, duplicated, malformed, absent or incomplete. Presence of a uniquely matching identifier is not evidence of successful transmission, delivery, conversation intent or procedural causation. A matched identifier does not independently confirm attribution or prove a business task was executed.

A full threading system should consider subject matching, quoted text, forwarding, re-sending, group aliases, multiple threads per work item, missing source records, user adjudication and contradictory claims **as separately scored hypotheses**, never as silent replacements for this high-precision baseline. No GraphRAG or LLM is used here.

The output change is additive to the OCEL 2.0 format. Rust crates advance to `0.5.0` for consumer opt-in; `Conversion.schema_version=1.3.0` remains stable because the single-email conversion contract is unchanged. Batch consumers that compare exact object relationships must account for the optional `reply_target_candidate` O2O qualifier and preserve it during interchange.
