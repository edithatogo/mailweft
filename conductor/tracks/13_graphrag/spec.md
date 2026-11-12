# Optional evidence-constrained GraphRAG specification

**Goal:** qualify whether bounded graph-assisted retrieval improves optional reconstruction and evidence questions while preserving the Rust converter.
**Design:** docs/GRAPHRAG.md, docs/PRODUCT-SCOPE.md, docs/FRONTIER-EVALUATION.md.
**Scope:** six concrete contract/qualification packets and their synthetic checkers. No feature implementation or model capability is claimed by these tasks. Qualified provider selection leads to an explicit Rust implementation track with exact accepted API versions; do not fabricate a Rust Microsoft GraphRAG API.
**Review focus:** scope-crossing paths, future information, stale/retracted summaries, quotation dependence, unsupported generated assertions, prompt injection, unchanged core output.
**Acceptance:** exact evidence/snapshot/scope boundaries, deterministic no-model baseline, optional local provider, correction invalidation, gated candidate enrichment and independent five-arm evaluation. GraphRAG remains default-off, and no task here is a core-release dependency.
