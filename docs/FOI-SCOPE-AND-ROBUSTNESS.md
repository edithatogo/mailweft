# FOI-first integration and error-impact evaluation

**Decision date:** 8 November 2026. **Status:** proposed implementation and study protocol; no extraction experiment or downstream migration executed.

## Scope and naming

Selected repository: **edithatogo/mailweft**. Core outcome: **an email-to-OCEL converter written in Rust**. Enron is only a canonical verification/example corpus, with version/rights/source matching; it is not the library's domain or required input. See `PRODUCT-SCOPE.md` and `NAME-MIGRATION.md`.

`foi-process` is the first and sole active owned consumer and depends on Mailweft, never the reverse. `fyi-cli`/`fyi-archive` retain capture/archive boundaries; `foi-o` owns semantics and receives reviewed evidence-linked contributions from independently validated consumer results. See `FOI-BOUNDARIES.md` for the documented handover ambiguity and exact responsibilities.

The existing FOI pipeline is unvalidated. Do not use its outputs as a reference, expected-output baseline, teacher, gold set or behavioral-parity target. It may be inspected to diagnose specific defects only. All evaluation expectations are independent of it.

## Source fidelity and the FYI application

The source forms must be audited against actual available archived records before implementation:

- `raw_rfc5322`: original available message bytes, with their actual headers and MIME structure.
- `platform_message_export`: a platform-derived record whose supplied fields and omissions are explicit.
- `rendered_web_message`: a publicly rendered page or fragment, not a substitute for raw MIME.
- `attachment_artifact`: a linked document with its own bytes, hash, extraction method and origin.

Do not synthesize EML from a webpage and then claim raw-header fidelity. Missing Bcc, transport metadata or attachments are unavailable, not known-empty. The raw core may reject a non-raw source; the consumer can use a separately labelled optional text/evidence path for that source. No change to the raw-email core contract is implied.

Keep platform observations and email assertions independent. A page's status is not independent confirmation of the email that caused it, and a user's outcome label is not an adjudicated event. A receipt of an extension notice is not a judgment about whether an extension was lawful. Capture time, platform display time, header-asserted time and business-event time remain separate.

## Existing FOI evidence and migration

Read-only inspection of `foi-process/src/rust4pm.rs`, blob `7665c02ae363507b22f659a14cebc372e1353e14`, shows deliberate empty attribute vectors for type, object and event output. This is a documented adapter limitation, not a finding that Rust4PM cannot express attributes or that all FOI results are invalid. `src/normalize.rs`, blob `0eda8dd516754248b25615ac28dc8a86ff96a2cd`, maps supplied activity labels through an FYI profile and copies an allowlist of attributes. Reproduce each suspected defect at an exact commit before deciding what to replace.

1. Freeze permitted source hashes and independently annotated expectations before showing annotators any pipeline output.
2. Document individual reproducible defects using a separate diagnostic ledger; this does not turn legacy output into a reference.
3. Audit source representations, access, completeness and actual capture/archive owners.
4. Define field-complete one-way consumer mapping; retain explicit losses and revision semantics.
5. Test the candidate against independent source-based gold and synthetic fixtures, not equality with the old pipeline.
6. Route validated, source-linked ontology gaps/counterexamples to FOI-O as candidates; independent review controls promotion.
7. Promote only after actual consumer checks. Recovery is a validated snapshot or disabled/quarantined analysis, not assumed safety of the old pipeline.

## What surpassing Buss et al. means

Buss et al. (2026) already combine heuristic/generative collectors and refiners, test synthetic descriptions derived from six OCEL logs, demonstrate two natural-text corpora, and compare directly-follows graphs as well as extraction. Their stated limitations include generative controllability, API cost/runtime and lack of operational stakeholder validation. Therefore "text to OCEL", "hybrid extraction" and "DFG comparison" are not novel claims here.

Candidate contributions, to be tested rather than asserted:

- Real raw-mail/attachment reconstruction with independent, source-grounded annotation and held-out request threads; evidence beyond synthetic prose.
- Incremental, auditable interpretation with traceable source spans, explicit uncertainty, quotation dependence and correction/retraction.
- Error propagation from message corruption and extraction errors to OCEL structure, process metrics and stability of domain conclusions.
- Selective prediction: better false-assertion risk at matched retained-event coverage, with honest abstention when evidence is insufficient.
- Practical Rust/local deployment measured against strong baselines at matched model, prompt, context and resource budgets. Rust performance does not establish NLP accuracy; local models/caching are engineering hypotheses, not independent novelty claims.

Compare original released configurations where executable, and a refreshed baseline with the same contemporary model used by the candidate. Run both native-input and equal-information experiments: a raw-mail pipeline must not claim algorithmic superiority solely because it received headers/attachments denied to a text-only baseline. Record exact code/model/prompt versions, calls/tokens, hardware, wall time, peak RSS, invalid outputs and unavailable arms. Never copy unlicensed upstream implementation or use a language port as the scientific novelty.

## Error-injection protocol

### Three distinct test families

1. **Semantic invariance:** reflow lines, change MIME encoding without changing decoded content, add harmless sign-off, duplicate a mailbox copy. Only declared representation/source-copy fields may change.
2. **Semantic counterfactuals:** change the requested actor, negate a statement, change "will send" to "sent". Update gold labels for the changed proposition; unchanged predictions are failures. These are interventions, not merely corruptions.
3. **Information degradation:** remove a date, hide an attachment, break references, truncate a body, corrupt an identifier. Keep latent truth for measuring loss, but label what remains knowable from the degraded evidence. Correct behaviour can be uncertainty or abstention, not guessing hidden truth.

Use original read-only sources and derived experimental copies only. Never alter FYI public records, send messages or publish corrupted data as evidence. Record a mutation manifest with source hash, operator/version, layer, seed, eligible population, selected records, requested/realised rate, magnitude and expected change/invariant.

### Four injection boundaries

| Boundary | Example operator | Evaluation purpose |
|---|---|---|
| Raw/decoded source | malformed headers, missing References, timezone removal, HTML quotation changes, attachment omission | Parsing/attribution robustness and loss of observability |
| Intermediate interpretation | incorrect zone, swapped actor, quote promoted to new act, negation dropped | Isolate semantic component errors without parser confounding |
| OCEL projection | object split/merge, missing or false E2O/O2O edge, qualifier change, typed-attribute loss | Attribute downstream mining error to a known log defect |
| End-to-end pipeline | paired source changes propagated through all enabled stages | Measure combined effects and compensating/compounding errors |

Start with single-factor perturbations at proposed rates 0%, 1%, 5%, 10%, 20% and 40% of the **eligible** unit. Record the unit (message, field, event or relation); rates across units are not directly interchangeable. For a bounded pilot use 20 paired seeds, report Monte Carlo uncertainty, then choose confirmatory repetitions from a prespecified precision target. Seeds do not create independent real requests.

Vary magnitude separately: one-character ID collision differs from complete entity reassignment; one-minute timestamp jitter differs from a timezone displacement. Add clustered failures (whole mailbox/thread/authority/time block), selective missingness (e.g. responses with withheld attachments) and predeclared two-factor interactions after single-factor runs. Do not assume random missingness models realistic absence.

### Metrics and endpoints

Measure all four levels: parser/source fidelity; zones/roles/acts/identity/calibration; OCEL event/object/qualified-relation preservation; downstream process conclusions. Primary proposed research endpoint is the change in the rate of **predeclared source-answerable conclusions that become wrong**, comparing injected and clean input at matched answer coverage. Register the exact query set and gold evidence before model runs. Always report absolute clean error as well as the injected-minus-clean difference.

Possible FOI conclusion queries include whether an acknowledgement or response is visible by an observation date, whether transfer/extension/clarification was communicated, whether a published response is partial, and which workflow stage has the largest measured delay. These are not automatic legal-compliance determinations. Compare process-graph edge precision/recall, event-count bias, duration error, rank stability and false-completion assertions. Missing evidence may make a metric not identifiable; report that rather than an invented point estimate.

Report clean performance, degraded performance, degradation slope/area, first tested level of materially unstable conclusions, detection recall/precision, recovery quality and collateral repair errors. Show risk-versus-coverage curves so abstaining on everything cannot win. For uncertain alternatives, separate exhaustive bounds from Monte Carlo sensitivity intervals; do not claim universal guarantees from sampled worlds.

Compare no repair, deterministic repair, uncertainty-preserving handling and optional learned repair. Repairs require evidence; never force a suspicious event sequence to match the desired workflow. Include oracle-repair simulations only as labelled upper-bound experiments.

### Statistical and annotation controls

Freeze independent labels before inspecting outputs; keep disputed labels and observable-versus-latent truth distinct. Keep whole request/conversation/duplicate/quote components within splits; hold out time periods and authorities where feasible. Treat large connected components explicitly rather than secretly reverting to message splits. Use paired comparisons and bootstrap at the independent request/component level; add authority-level generalisation checks. Report effect sizes and uncertainty, multiplicity policy and held-out design. Generated text and LLM judges alone are not gold labels.

Prevent circularity: do not use the same FYI status both as an extraction input and independent evaluation truth. Run metadata-only, text-only and combined arms separately and identify which reference labels remain independent in each. Platform metadata can be an anchor for chronology but is not automatically a definitive business outcome.

## Prior art and sources

- Buss et al. (2026), *Process mining between the lines*, Information Systems 140, 102713. DOI: https://doi.org/10.1016/j.is.2026.102713 . Code: https://github.com/Alinabuss/OCEL_extractor_v2 . Paper read: https://publica-rest.fraunhofer.de/server/api/core/bitstreams/978fb060-a826-4f48-a5d0-d75186238b59/content . PDF screenshot service returned cache errors; claims here rely on parsed narrative text and source README, not unseen figures.
- Sadeghianasl et al. (2026), *Object-centric event-data imperfection patterns*, Information Systems 141, 102766. https://doi.org/10.1016/j.is.2026.102766 . Already injects six object-centric error patterns into BPIC2017 at 10% of traces; error injection itself is not novel.
- Kabierski et al. (2026), *Data quality in object-centric event data: issues classification and evaluation*. https://doi.org/10.1007/s44311-026-00043-x . Map mutation families to event/object/relation quality dimensions before introducing email-specific extensions.
- Ribeiro et al. (2020), *Beyond Accuracy: Behavioral Testing of NLP Models with CheckList*. https://aclanthology.org/2020.acl-main.442/ . Behavioural/minimum-functionality, invariance and directional tests provide methodological precedent.
- Pegoraro, Uysal and van der Aalst (2021), *Conformance checking over uncertain event data*. https://doi.org/10.1016/j.is.2021.101810 . Explicit uncertainty and conformance bounds already have precedent; email-specific composition and empirical validation must establish any new contribution.
- FYI source context: https://fyi.org.nz/help/about . Source availability, rights and interpretation must be verified for the selected records; no live ingestion was executed.

## Existing executable work allocation

- T11.01: frozen independent labels, observability policy, request/component splits.
- T11.02: invariance and semantic-change operators plus mutation manifests; expanded operator families follow the same contract.
- T11.03: calibration and matched-coverage error/abstention evaluation.
- T11.05: staged injection, dose-response, process effects and predeclared conclusion endpoints.
- T11.06: Buss, data-quality and uncertainty comparators with fair information/model budgets.
- T11.08: independent latent-ledger fixtures and renderers, without a simulation-repository dependency.
- T12.01–T12.06: only FOI diagnosis, source fidelity, adapter contracts, shadow comparison, migration and acceptance packets.

These are planning deliverables; all referenced runtime functions, operators, migration actions and evaluations still require implementation and execution evidence.
