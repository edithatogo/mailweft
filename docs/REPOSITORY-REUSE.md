# Repository reuse and ownership boundaries

Review: 8 November 2026. This is a targeted review, not an inventory of every repository. The source register records inspected blob hashes. No claim of fresh build/test qualification is made.

**No sibling repository becomes a core dependency.** Reuse in this order: existing compatible upstream crate; small qualified neutral API; shared format/fixture; source design pattern. Extract a shared crate only after two real consumers and API/rights qualification justify it. Do not create another generic framework just for future possibilities.

| Repository | Relevant material | Decision and qualification boundary |
|---|---|---|
| `foi-process` | Rust replay, revisions/retractions and optional Rust4PM projection | Reuse candidate after source audit; prefer neutral patterns/interfaces over a FOI-domain dependency. src/rust4pm.rs inspected: snapshot output deliberately passes empty attribute vectors. It is NOT an attribute-complete OCEL adapter. Cargo.toml pins process_mining=0.6.0 and declares Apache-2.0. No tests executed in this review. |
| `sourceright` | Verification sidecars, conflict resolution and review queues | Reuse provenance/review conventions only after a contract audit. Reference/citation verification does not verify that an email assertion is true. Technical preview; not a generic email identity service. |
| `citeweft` | Rust source spans, uncertainty and backend-neutral scholarly extraction | Optional scholarly-attachment evidence adapter. Technical preview; scholarly scope. Do not assume general PDF/Office extraction or production backend parity. |
| `fe-reader` | Headless Rust document/PDF workflow contracts | Optional attachment provider behind read-only extraction interface. Technical preview; qualify actual text/OCR/geometry capability using its parity registry, not README feature names. |
| `nlp-policy-nz` | Existing NLP, language and evidence mapping experiments | Evaluation reference or separate opt-in service only. README specifies a Python production runtime with Rust acceleration. Never import it into the Rust-only default binary; policy-specific models need email-domain tests. |
| `searchright` | Rust evidence-search contracts, retrieval and provenance conventions | Optional literature/benchmark discovery and evidence-restricted retrieval. Technical alpha; scholarly search/review semantics are not mailbox semantics. Do not depend on its full workspace. |
| `rcagent` | Investigation chronology, source/account separation and research protocols | Downstream consumer of evidence exports and optional evaluation scenarios. Released core is a Markdown skill, not a Rust engine or independently validated business-event extractor. |
| `kairos` | Deterministic Rust DES/ABM | Optional synthetic workflow generator with independently authored email renderer. Alpha; use latent event ledger as truth, not generated emails or a second LLM as an oracle. No simulation dependency for extraction. |
| `process-mappings` | Domain profile registry idea | Future optional domain mappings only. README explicitly describes a non-canonical incubator and says no substantive mappings implemented. Not a reusable mapping library today. |

## Important adapter finding

`foi-process/src/rust4pm.rs` (blob `7665c02ae363507b22f659a14cebc372e1353e14`) contains real Rust integration using `AppendableOCEL`. Its current projection intentionally supplies empty attributes to type declarations, objects and events. Reusing that projection unchanged would breach email-ocel's attribute/history preservation contract. This is a **project adapter limitation**, not proof that Rust4PM cannot represent the attributes. Audit the selected Rust4PM API and use full structures; add typed event attributes, time-varying object attributes and E2O/O2O parity fixtures before any reuse.

`foi-process` also has domain-specific evidence types. Review replay/retraction patterns; do not import FOI terms into the generic email model. The adapter is snapshot-oriented: an append-only sink is not a correction/retraction store.

## Explicit non-dependencies

CareOps remains a downstream OCEL consumer. Rust4PM remains the process-mining engine; Propel may view exports. Kairos can generate test workflows, but never becomes a parser. Paignia, Voiage and innovation/economic models are not required to reconstruct communications; strategic intent or value-of-information research belongs outside this core.

CiteWeft and Fe Reader are complementary candidates: scholarly evidence structure versus document/PDF backend workflows. Neither is assumed to handle all attachments. SourceRight verifies reference/provider evidence, not business claims; Searchright's scholarly retrieval infrastructure cannot be relabelled as an email GraphRAG service.

## Contract-first integration packet

For each proposed adapter, record: producer/consumer package+revision, SPDX/notice verification, data classes allowed, exact input/output schemas, field-level loss ledger, access-scope propagation, timeout/resource limits, offline negative test, synthetic round-trip fixture, default-disabled feature and removal/fallback test. Pin qualified versions only after successful actual compilation. Existing repository histories and native systems remain untouched by this plan.
