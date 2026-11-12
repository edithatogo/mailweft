# Multi-EML batch conversion (experimental Rust implementation)

This increment extends the standalone Rust email-to-OCEL converter without introducing Python, Enron-specific assumptions, FOI-specific mappings, source capture, GraphRAG or model dependencies.

## API

`convert_batch(&[SourceInput]) -> Result<Ocel, Error>` accepts raw in-memory EML byte slices, separately identified source copies and caller-supplied acquisition timestamps. It independently calls the qualified Stage 1 parser, then merges typed OCEL objects and observations in deterministic identifier order.

- Repeated source copies of exactly the same message bytes share a **candidate** email_message object.
- Different source IDs retain distinct source_copy objects and observed events.
- Repeating an identical source observation is idempotent; re-observation at a different timestamp remains a separate event.
- Message-to-copy object relationships accumulate without duplicate links.
- Conflicts on a stable object/event ID fail rather than silently choosing one record.
- The meaning of header Date, Received, Message-ID, References, source delivery and business completion is **not** inferred by batch merging.

This is exact-byte candidate identity, not deduplication of semantically equivalent or transported/reformatted messages.

## CLI

```sh
mailweft batch 2026-11-09T01:02:03Z path/first.eml path/second.eml > combined.ocel.json
```

The command reads explicit filenames only, not recursive folders, and requires the same acquisition timestamp for the supplied set. No network access occurs. Public results contain hashed address and copy identifiers, not the email bodies or raw addresses. The CLI bounds a batch to 1,024 files and 256 MiB total input (and the per-message limit remains 16 MiB). Paths serve as source identifiers: moving/renaming a file changes its copy identity unless a stable source name is introduced by a later source-envelope adapter.

The in-memory batch API currently materializes a combined OCEL; it is **not** a streaming or high-throughput full-corpus implementation. Enron remains an optional verification dataset and must not become a required input.

## Quality and missing capabilities

Synthetic tests check a two-custodian same-message case, object-link union, input-order invariance, repeated-observation idempotency, empty batches, and the actual CLI output. A feature-gated Rust4PM integration test verifies independent import of a combined two-event OCEL, with code/version pinned in Cargo.lock. Exact CI results must be read from the branch commit.

Not yet implemented: time-interval reconciliation, header chronology, robust metadata/threading, attachment extraction, complete source manifests, uncertainty sidecars, incremental archive persistence, rights-aware source adapters, multi-source business events, and full OCEL attribute history roundtrip. Do not infer validation of those capabilities from the basic batch tests.

The first downstream consumer remains `foi-process`, with reviewed ontology-feedback to `foi-o`. Its existing analytical outputs are not reference labels, and capture/preservation remains outside Mailweft.
