# Release boundary and qualification checkpoint — 10 November 2026

This is a supplementary evidence record, **not a generated task-state override**. The canonical `conductor/task-manifest.json` remains authoritative, and the doctor continues checking the existing project context.

## Accepted direction

The product remains a Rust email-to-OCEL 2.0 converter. Enron is example/verification data only, and the first downstream consumer is `foi-process`; FOI mappings and reviewed ontology changes remain external. No automatic release is authorized. There is no premium connector, cloud model, GraphRAG or Python runtime dependency in the converter.

## Work introduced on this branch

- A deterministic, failing known-advisory check for vulnerable `quick-xml` in `Cargo.lock` (RUSTSEC-2026-0194 and -0195); it is explicitly a **narrow supplement**, not a substitute for the current full RustSec audit.
- A manually dispatched and tag-triggered release-security workflow that requires full `cargo audit`, `cargo deny` and the known-blocker precheck to pass; it **never publishes** a release. The existing dependency graph is expected to fail; no advisory ignore or allowed failure is introduced.
- A Linux/macOS/Windows cross-platform functional matrix with independent Rust4PM import tests.
- Additional independent generated-OCEL structural integrity checks: unique IDs, type and attribute catalogues, qualified E2O/O2O reference integrity, and time-qualified object attributes. This is **not** full normative OCEL 2.0 certification.
- A new release policy mapping explicit blockers to actionable follow-up work.

## Task reconciliation

Keep T00.06 in progress, T07.07 and T08.06–T08.08 not complete until actual security, reproducibility, interoperability and release evidence is accepted. No blanket Conductor status update is justified by writing workflows. `scripts/conductor_doctor.py --check` should continue to pass without regenerating any derived project context, since no canonical statuses or prerequisites were changed.

## Hard release blocker

`process_mining=0.6.2` pulls `quick-xml=0.37.5`, which is affected by RUSTSEC-2026-0194 and RUSTSEC-2026-0195. The upstream constraint `^0.37.4` prevents a direct lockfile jump to patched quick-xml `>=0.41.0`. The default CLI does not expose an XML input path, but that is not an exemption from dependency-security qualification. Issue [#4](https://github.com/edithatogo/mailweft/issues/4) remains open, and alternative draft PR [#3](https://github.com/edithatogo/mailweft/pull/3) is not presumed safe or mergeable.

## Next implementation order

1. Upstream-compatible Rust4PM dependency remediation, or an independently justified maintained validator replacement **without sacrificing OCEL attribute/qualifier/history fidelity**, followed by exact-lock advisory and licence audits without ignores.
2. Normative OCEL 2.0 schema/roundtrip validation with independent reader and explicit loss ledger; consider JSON, XML, SQLite only when their parsing dependencies are qualified.
3. Bounded malicious-MIME and resource fuzzing, safe directory traversal, secret-keyed scoped pseudonyms to address dictionary attacks, and strengthened privacy verification.
4. Cross-platform runs and an evidence-backed release checklist and notices. A core-only release may proceed without Enron/full-corpus, NLP/GraphRAG or FOI migration only after the core gates pass and an authorized release decision is recorded.
5. FOI source adapter and independent scientific evaluation remain separate consumer work. Unvalidated existing FOI-process results must never become the oracle.

No real correspondence is processed or published by this checkpoint. Record exact commit-linked CI outcomes after executing workflows; do not infer a future green result.
