# 0.6 experimental migration — source receipts and zoning origins

The Rust library and CLI advance from 0.5.0 to **0.6.0**. No new production dependency is added.

## Unchanged

Existing single-message/batch OCEL fields, identifiers, timestamps and qualified relationships are unchanged. `Conversion.schema_version` remains **1.3.0**. Enron stays an optional verification/example corpus. FOI-specific logic and GraphRAG are not introduced.

## Added source contracts

`SourceEnvelope` and `SourceReceipt` are version **1.0.0**. The opt-in `convert-evidence` command emits a bundle with the original conversion and a receipt, not a bare OCEL. Source metadata remains caller-asserted; only supplied byte length/hash is verified. Unsupported fidelity classes are rejected. Unknown JSON fields, duplicate fields, missing timestamps and unsupported schema versions fail closed. Consumers must retain the private envelope separately if they need to inspect its producer/rights/locator references.

All CLI reads are now counted and bounded; directories, special files and visible final-component symlinks are rejected. Consumers previously relying on final-component symlinks must supply a regular file. This does not provide race-resistant filesystem confinement.

## Zoning sidecar

`BodyEvidence.schema_version` advances from **1.0.0 to 1.1.0**. Each zone gains `origin_scope`, `quote_depth` and `forward_depth`; the zone vocabulary adds `forwarded_content` and `inline_header`. `skipped_parts` reports unsupported HTML, unavailable decoding, unverifiable part ranges and resource exclusions. Total emitted zones are capped at 65,536 per inspected source; admitted plaintext parts remain capped at 2 MiB.

Forwarded passages are conservatively kept out of newly authored content to the end of the part, absent an independently qualified end boundary. Quoted lines preserve nesting depth; unprefixed inline replies are not automatically quoted. Disclaimer detection precedes signature carry-over. All labels/origins remain rule hypotheses, not verified authorship or business acts. Exact raw offsets still require byte equivalence; transformed text remains `derived` with null raw-span offsets.

The implementation-specific `body-evidence` schema is separate from the older proposed `email-zone` interchange contract. It does not claim completion of nested zone graphs, author resolution or encoded byte/character source mapping.
