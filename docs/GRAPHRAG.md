# Optional evidence-constrained GraphRAG

## Decision
GraphRAG is worth evaluating for cross-message context and evidence retrieval. It is not needed to parse mail or emit OCEL. The default Rust converter remains offline and model-free. Choose an **evidence graph first**, then a bounded retriever, then optional generation. Do not rebuild a second ontology/extraction engine merely to use a GraphRAG brand or framework.

Alternatives considered: mandatory GraphRAG would broaden runtime and invent a core model dependency; no graph would forgo useful multi-hop retrieval; an optional graph projection preserves the converter and allows controlled comparison. The last is selected for this plan.

## Two separate uses
1. **Extraction assistance:** fetch related messages, quoted antecedents, attachment passages and previously identified entities to help resolve a pronoun, work item or asserted action. The result is an `ExtractionClaim` or proposed `InteractionLink`, routed through the existing evidence checks. It never directly writes observed events or edits a Stage 1 record.
2. **Post-extraction questions:** retrieve source-grounded evidence about process histories, conflicts and recurrent patterns. Narrative answers cite original source references. Counts, durations, rankings and conformance results come from deterministic queries over a named OCEL/evidence snapshot, not generated community summaries.

In the FOI application, useful questions include which correspondence supports a transfer or extension assertion, which attachment belongs to a response, and what counterevidence exists. Answerability and source completeness must be visible. A retrieval path does not establish legal authority or causation.

## Layered implementation
- **G0:** deterministic graph projection from versioned Mailweft objects, events, source copies, spans, claims and qualified links. Maintain distinct observed/asserted/inferred edges and copied-evidence families. No LLM is needed to construct the graph of already available records.
- **G1:** bounded graph traversal plus lexical retrieval over authorized source text. Add optional vectors only when benchmarked. Freeze request scope, snapshot time, edge-type allowlist, hop/node/token limits and candidate ordering. No database is mandatory for the first prototype.
- **G2:** optional local-model generation over the returned evidence. Return structured claims/answers, supporting and contradicting references, missingness and abstention. Retrieval scores and model agreement are not probabilities or independent corroboration.
- **G3:** optional community summaries / global or DRIFT-style retrieval after G1/G2 show value. Summaries remain derived, cite original evidence and never count as a second source confirming their own inputs.

The Microsoft GraphRAG project is a **reference/comparator**, not the default runtime dependency. Its documented indexing extracts entities/relationships and builds community summaries; local search combines graph structure with source text, global search uses community reports, and DRIFT adds community context to local retrieval. That reference uses a Python interface. A separate opt-in benchmark adapter may use it after licence/provider qualification; Rust production remains Rust. No claim of implementing the complete Microsoft algorithm or proving parity is made.

## Non-negotiable correctness/security
- Every graph node/edge has source references, source classification, evidence state and snapshot/version lineage. Keep known absence distinct from unavailable fields.
- Apply authorization before traversal/scoring, then before returning evidence. Cross-request links require an explicit permitted scope. Per-scope summaries and embeddings must not leak excluded records.
- Query `as_of` snapshots without future evidence. Invalidation covers retractions, revoked access, deleted content, updated source maps, model/prompt changes and index versions. Stale summaries are unavailable until rebuilt, not silently returned.
- Do not treat copied quotations, several mailbox copies, graph paths or repeated model answers as independent witnesses.
- Treat mail, attachments and retrieved text as untrusted data. No execution, remote-image fetch, URL-following, instructions or automatic ontology edits. Limit prompt/context sizes and model resource use.
- Admit candidate enrichment only if source anchors, types, allowed scope and extraction-policy checks pass. Evidential checks prove anchoring, not truth. All Stage 1 byte hashes must be unchanged after optional analysis.
- A GraphRAG failure cannot fail or silently alter the default conversion output. Optional features return explicit unavailable/budget/abstention states.

## Evaluation and activation gate
Compare (A) no retrieval, (B) lexical retrieval, (C) vector retrieval, (D) deterministic graph+source retrieval and (E) graph+source+generation with matched source scope, model and budget. Separate reconstruction quality from question answering. Use frozen independent annotations, thread/request heldouts and synthetic latent truth. Enron supplies a canonical example benchmark; the first applied evaluation is independently annotated FOI source evidence, never the old foi-process outputs.

Measure support recall/precision, source-attribution accuracy, false entity links, unsupported statements, answer coverage/risk, temporal leakage, scope leakage, cost/latency, memory, correction propagation and downstream OCEL distortion. Inject erroneous edges, entity merges/splits, stale summaries, duplicated quotations and missing evidence using the existing controlled-corruption protocol. Include negative questions whose answer is not present. Promote only with a preregistered improvement over simpler retrieval at matched answer coverage/resource budget, no known critical access-control failure, and a verified unchanged-core-output test. Statistical superiority is not assumed from implementation.

## Primary references (checked 2026-11-08)
- https://microsoft.github.io/graphrag/index/overview/
- https://microsoft.github.io/graphrag/index/architecture/
- https://microsoft.github.io/graphrag/query/overview/
- https://github.com/microsoft/graphrag/blob/main/docs/query/local_search.md
- https://github.com/microsoft/graphrag/blob/main/docs/query/global_search.md
- https://github.com/microsoft/graphrag/blob/main/docs/query/drift_search.md

See track `13_graphrag`. This is design and task specification only.
