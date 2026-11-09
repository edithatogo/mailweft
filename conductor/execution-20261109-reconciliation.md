# Accepted execution slice: corpus reconciliation, encoding provenance and Conductor repair

Base 20a94560f6af91b2bc3764bd5bce93ddf0dff21d. The user authorizes continuing the remaining work and reconciling/setting up Conductor. Remote feature branch and isolated Actions checkout are the execution workspace: this session has no local Rust toolchain or outbound DNS. No real email source is used.

## Decisions
- T01.07/T01.08: explicit selection manifests, per-item accepted/rejected accounting, deterministic digest, original source receipts and fail-closed OCEL projection by default. Partial accepted-only projection requires explicit opt-in and retains a nonzero partial-success exit. Never infer corpus completeness from supplied selection completeness.
- T05b.03: add verified transfer-unit mappings for base64/quoted-printable only where parser-decoded bytes equal UTF-8 analysis bytes. Keep zone raw offsets derived; transfer-unit mappings are not fictitious one-to-one character offsets. Malformed input or charset transformations abstain.
- T00.07: keep the actual pinned Conductor source unchanged. Audit upstream format, create missing per-track indexes and upstream metadata aliases, synchronize task/plan/registry/setup state from the task manifest, and add drift tests. Do not reset existing context or falsely certify host registration.
- Reconcile implemented tasks by evidence; completed tasks need test commands/artifacts and residual limits. Keep partially implemented work in_progress rather than pretending all tasks are untouched or complete. Do not discard original requirements or optional-lane isolation.

## Sequence and evidence
1. Add CLI, mapping and Conductor regressions and observe failures against the current implementation.
2. Implement deterministic bounded selection reconciliation and transfer mapping using existing parser output as oracle, not a new MIME parser.
3. Repair Conductor state and add repeatable doctor/status/bootstrap instructions; pin tested Rust toolchain.
4. Run whole-suite, schema, Rust4PM, formatting, lint and Conductor integrity checks. Self-review full diff, remove any one-time write-enabled helper, publish only verified work.

These are bounded slices, not full large-corpus, GraphRAG, semantic NLP or live FOI validation. Enron remains canonical verification/example data. Capture/archive/FOI-O ownership is unchanged.
