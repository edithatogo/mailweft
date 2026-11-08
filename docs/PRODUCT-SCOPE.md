# Mailweft: core output and scope

## Core outcome
**An email-to-OCEL converter written in Rust.** This is the product, not a by-product of an NLP research platform. Input is any authorised raw RFC 5322/MIME email corpus, initially local EML and maildir. Output is standards-compliant OCEL 2.0 JSON with qualified participants/objects, declared time semantics, reproducible identity, source coverage, and a separate evidence/loss receipt. Rust4PM is the selected qualified OCEL implementation, not a reason to tie the data to a particular mining UI.

Core acceptance is a complete offline vertical slice: raw synthetic messages -> parse -> identity/roles -> communication OCEL -> independent import/round-trip checks. No Enron access, FOI project, model, embeddings, GraphRAG, cloud, Python runtime, ontology service, or attachment-content converter is required. Basic fixture tests must include at least two unrelated invented domains and messages without any Enron metadata.

## Dataset role
Enron is a **canonical verification/example corpus**, not the subject, schema, mandatory input, default domain, training prerequisite, or organizing abstraction of Mailweft. Dataset-specific adapters and recipes live under benchmarks/examples only. Use pinned source versions, hashes, source-format inventory and rights records. Raw messages remain input; processed datasets are external comparisons after provenance/population matching, never automatically correct labels. No actual Enron bytes are bundled in source or crates.

## Independent validation
The existing `foi-process` pipeline is unvalidated and has reported issues. Its output must not be used as a reference, gold set, teacher, regression oracle, correctness baseline, or requirement for behavioural parity. Legacy code/output may be inspected only to reproduce a specifically documented defect. Expected outcomes come from raw source, independent adjudication, normative format contracts and separately validated synthetic fixtures. Reject a new result even when it agrees with the legacy path if it conflicts with independent evidence.

## Scope extensions
Structural understanding (zones and textual participants), business-act NLP, attachment content extraction, interpretation revisions and GraphRAG are opt-in layers. They never modify Stage 1 observations. `foi-process` is the first and sole active owned consumer. `fyi-cli`, `fyi-archive` and `foi-o` appear only as boundaries of that FOI application, not additional generic integrations or reverse dependencies.

## Readiness
This repository currently provides specifications, contracts, granular tasks and planning checks. It is not yet an implemented Rust converter. Research claims require executed, reproducible comparisons; documents and schema checks do not establish extraction accuracy.
