# 0.5 Rust crate migration — candidate reply edges in batch OCEL

The Rust crates advance from 0.4.0 to 0.5.0. The single-email `Conversion.schema_version` remains **1.3.0** because its serialized fields and meanings are unchanged.

The multi-EML `convert_batch` OCEL graph may gain an object-to-object `reply_target_candidate` relationship. It is produced **only** when an In-Reply-To claim uniquely matches the Message-ID claim of a distinct observed logical message. A non-unique, absent, malformed or self-targeted claim does not cause an edge. `References` alone is insufficient for a direct candidate.

Consumers should not equate candidate links with verified replies or use them to infer event dates, responsibilities or legal compliance. Case/thread correlation remains a separate inference layer; legacy FOI outputs remain unsuitable as ground truth. Preserve O2O qualifiers during Rust4PM import and downstream projection. No new source acquisition, storage, NLP, GraphRAG, or real-data processing is introduced.
