# Evaluation that can support—or refute—a frontier claim

## Fixed comparisons

Raw-message truth, processed corpus compatibility and semantic/business-event truth are three different oracles. Pair messages only with verified source/version crosswalks. Published processed Enron is a frozen compatibility reference, never silently used as semantic ground truth. Licensed annotated training resources may be used in a separate manifest, with train/development/test separation; this does not convert processed OCEL outputs into training labels.

Evaluate: rules; conventional NLP; local model; rules+model; conversation context; graph-only; vector-only; graph+vector; full GraphRAG. Include the Buss et al. 2026 text-to-OCEL collector/refiner approach as an external comparator when its exact code/data rights and runtime are qualified. It is a baseline, not proof of an email-specific solution or a dependency to port without permission.

## Adjudication and leakage

Freeze annotations before exposing model outputs; use dual annotation with adjudication for ambiguous tasks. Split by connected conversation/duplicate/quote components and source release before message-level sampling. Hold out people/custodians where feasible; report incompatible split objectives rather than claim simultaneous isolation. Include out-of-time, different client, multilingual and non-Enron tests. Document pretraining contamination uncertainty. Template families and paraphrases must not straddle synthetic train/test splits.

## Metrics

Report denominator, excluded/ambiguous records and confidence intervals at independent conversation/component level. Include per-class zone/span/role/act precision-recall; false actor assignment; entity false merges/splits; quote-as-new-event errors; temporal interval coverage and partial-order errors; calibration, risk-coverage and abstention; event/object/qualified-edge preservation; source-span mapping fidelity; cost, latency and peak RSS. Runtime throughput is measured, never inferred from Rust versus Python.

Downstream measurements compare the same projection, vocabulary and event view: event/edge count distortion, OCDFG edge precision-recall, variant distribution drift, rework inflation and bottleneck-rank sensitivity. No LLM judge alone establishes a gold event, correct process model or causal explanation. Statistical superiority requires a predeclared primary metric and no unacceptable harm in identity, privacy or false-assertion metrics; report negative results.

## Controlled perturbation matrix

| Perturbation | Must change | Must remain invariant |
|---|---|---|
| To becomes Cc, body unchanged | header-role qualifier | no automatic reassignment of task actor |
| `Alex, please review` becomes `Sam, please review` | candidate actor and evidence | header roles and source chronology |
| add a signature title | signature role claim | author identity/authority not verified automatically |
| wrap a request as attributed quote | origin and new-act eligibility | raw source content and quoted assertion |
| `I will send` becomes `I sent` | modality/tense/act | neither becomes verified delivery |
| insert `not` | negation | unrelated objects/claims |
| duplicate across mailbox copies | source-copy count | no duplicated independent business event/corroboration |
| remove References | threading uncertainty | no fabricated parent |
| reorder same-time input records | processing order only | canonical identities and asserted partial order |
| change available mailbox scope | coverage/absence qualification | no inference of inaccessible Bcc recipients |
| malicious attachment instruction | quarantine/diagnostic | no network/file execution or data promotion |
| revise/retract a claim | active snapshot and dependents | original observed source facts |

## Adoption gates

Adopt advanced methods only after they beat the simplest eligible baseline on the registered objective at stated privacy/resource limits. Benchmark model weights, prompts and cache versions separately. Skip an unavailable comparator with an explicit reason; do not invent results. A source/schema validator is not a successful Rust extraction or a scientific result.

## FOI-only scope and robustness refinement
The sole active owned consumer is `foi-process`, which will depend on this generic library. Other owned-repository integrations are removed from active tasks. See [the superseding source-fidelity, baseline and error-impact protocol](FOI-SCOPE-AND-ROBUSTNESS.md). Repository name `mailweft` is selected. Planned package paths use Mailweft; historical schema IDs remain stable aliases (see docs/NAME-MIGRATION.md). No runtime benchmark or downstream migration is claimed.
