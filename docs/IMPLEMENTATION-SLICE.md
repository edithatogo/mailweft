# Implemented slice — raw EML to OCEL 2.0 (experimental)

This branch introduces a compiling Rust **mailweft-core** and a thin **mailweft** CLI. The first stage observes one supplied source copy and projects it to a portable OCEL 2.0 JSON shape. It uses Stalwart `mail-parser` 1.0.1, and a feature-gated Rust4PM 0.6.2 interoperability test (not a required runtime dependency). The CLI requires the external acquisition timestamp, rather than silently treating an email Date header as proof of delivery. Enron remains an optional external example/benchmark.

## Usage

```sh
cargo run -p mailweft-cli -- raw.eml 2026-11-09T01:02:03Z source-copy-identifier > observed.ocel.json
cargo test --workspace
cargo test -p mailweft-core --features rust4pm-validation --test rust4pm_interop
```

The output deliberately contains only hashed address identifiers; the mapping to source addresses must be maintained outside the public OCEL. Source ID and input bytes are not printed. Message identity is provisional and content-derived: equal EML bytes share a candidate message ID, but each source/custodian has a distinct copy ID. This is an observation of supplied data, not a verified sent or delivered event.

## Current limitations

- Stage 1 currently accepts individual EML files, not recursive maildir, MBOX, PST, Outlook or live connectors. Source ID/time are caller-supplied. Only explicit parsed mailbox addresses are linked; unresolved names, groups without addresses, references, attachment objects and headers beyond From/Sender/To/Cc/Bcc/Reply-To await qualification. Do not infer a person from an address.
- The event type is `source_copy_observed`, not `email_sent`. Header date, in-reply-to, references, content/timezone uncertainty, source receipts, signatures and semantic business events remain future tasks.
- The OCEL JSON output uses the version-2 standard shape. Full interoperability requires qualified Rust4PM import and type/qualifier/attribute round-trips; successful serde serialization alone does not establish that.
- The current max input size is 16 MiB per raw message, with explicit error; size/depth bounds and safe attachment content handling need extension and tests.
- There is no GraphRAG implementation; it remains a separate, optional research lane.
- This branch is not yet a release; record exact commit CI and external import status before treating it as validated.

## Scope

Mailweft must remain a corpus-neutral Rust email-to-OCEL converter. Never import FOI semantics or trust the existing unvalidated FOI pipeline as an oracle. FOI mapping remains in `foi-process`, and reviewed ontology candidate promotion remains with `foi-o`. Original acquisition and preservation remain with the `fyi-cli` and archive boundaries.

See `docs/CONTRACT.md`, `docs/GRAPHRAG.md`, and Conductor tasks T01/T03.
