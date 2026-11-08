# Contract versions and semantic validation

Seven existing v1 schemas are retained unchanged. Four new v1 record types are additive: communication-act, interaction-link, inference-receipt and interpretation-delta. They are planning contracts, not proof of implemented serialization or inference. Schema IDs use example.invalid intentionally until a canonical published namespace exists.

JSON Schema checks only structural constraints. Implement semantic checks for referenced IDs, hashes/offset bounds, UTF-8 boundaries, nested-zone cycles, no self-referential interaction edges, timestamp interval order, unique check names, projection policy and allowed-scope intersections. `admit_to_enriched_view` requires mandatory checks to pass and never denotes verified real-world truth. A calibrated score needs a valid held-out calibration receipt; copied evidence cannot raise independent corroboration.

Delta chains require monotonic revision identity, correct prior hash, idempotency/conflict detection and retained retraction semantics. A schema-valid delta is not by itself a valid revision. Cross-record, authorization and source-truth checks are future Rust tasks, not supplied by JSON Schema alone.

Source spans and uncertainty that OCEL cannot express remain in sidecars. OCEL 2.0 exports must not be widened to a private dialect. Optional OCEL 2.1 output must identify its version and loss profile separately.
