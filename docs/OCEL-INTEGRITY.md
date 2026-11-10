# Independent OCEL structural integrity checks (development-only)

The compiled Rust CLI's synthetic `convert-evidence` and selection-reconciliation output is now checked with `scripts/validate_ocel_integrity.py`. It checks that all object/event types are declared, attributes are declared for their types, all qualified E2O/O2O references resolve, duplicate identifiers/qualified edges are rejected, and object attributes carry observation times. Synthetic negative mutation tests ensure these checks fail on deliberately corrupted output.

**This is not normative OCEL 2.0 schema certification.** It is an additional safeguard alongside Rust4PM import qualification. The validator cannot establish semantic correctness, actor identity, a complete corpus, equivalence across JSON/XML/SQLite, or preservation of all legal OCEL value types. The current Rust converter stores source-derived attributes as strings; non-string typed values, object histories, more complex transformations and complete standard roundtrips remain core qualification work.

The independent validator is Python **development infrastructure only**, not part of the Rust runtime. A successful test provides evidence of the enumerated invariants for tested synthetic outputs, not a complete scientific or security evaluation. See `docs/VALIDATION.md` and the unfinished core release tasks in the canonical Conductor manifest.
