# Product

Mailweft's core outcome is **an email-to-OCEL converter written in Rust**. It accepts generic raw RFC 5322/MIME mail and emits OCEL 2.0 communication logs with source, loss and completeness receipts. This is an independently releasable product, not merely infrastructure for a research platform.

Enron is a canonical verification/example corpus only; it does not define the domain, input or schema. The first and only active owned consumer is foi-process, whose unvalidated outputs cannot be used as truth or a reference. fyi-cli/fyi-archive retain their evidence-production boundaries; validated consumer results can contribute reviewed candidates to foi-o.

Optional structural NLP, semantic reconstruction, attachment processing and GraphRAG preserve immutable Stage 1 observations. No models, cloud, graph database, FOI runtime or corpus access is required for core release. See docs/PRODUCT-SCOPE.md, docs/FOI-BOUNDARIES.md and docs/GRAPHRAG.md.
