# Related work and comparison register

Reviewed 8 November 2026. Public accessibility is not a software or data reuse licence. Exact versions, licences, corpus populations and applicability must be qualified before any comparator run. No upstream code or dataset has been copied.

## RFC5322: Internet Message Format

https://www.rfc-editor.org/rfc/rfc5322

Header roles, groups, Resent blocks and Bcc visibility; a source header is not independently verified identity.

## OCEL: OCEL specification overview

https://ocel-standard.org/specification/overview/

OCEL 2.0 JSON/XML/SQLite remain the baseline. The current site identifies CSV/bundled CSV-Parquet as OCEL 2.1 extensions; qualify separately.

## ZEBRA: Lampert, Dale and Paris (2009), Segmenting Email Message Text into Zones

https://aclanthology.org/D09-1096/

Historical zoning baseline; our labels are a proposed taxonomy, not a verbatim reproduction of their nine labels.

## ACTS: Cohen, Carvalho and Mitchell (2004), Learning to Classify Email into Speech Acts

https://aclanthology.org/W04-3240/

Prior art for communication-act classification; not a new contribution.

## REQUESTS: Lampert et al. (2008), Requests and Commitments in Email are More Complex Than You Think

https://aclanthology.org/U08-1009/

Utterance-level ambiguity and contextual attribution.

## EPA: Rameshkumar et al. (2018), EPA addressee tagging

https://aclanthology.org/W18-6104/

Task-to-person benchmark; it does not provide independent business-completion ground truth.

## QUAGGA: Quagga email segmentation reference implementation

https://github.com/HPI-Information-Systems/Quagga

Benchmark candidate; qualify code and annotation licences independently; no runtime adoption.

## TALON: Mailgun Talon

https://github.com/mailgun/talon

Quotation/signature baseline only; Python is not a production requirement.

## FORGE: Mailgun Forge

https://github.com/mailgun/forge

Annotation candidate from previous plan; exact version and rights still require qualification.

## TEXT_OCEL: Buss et al. (2026), Process mining between the lines

https://doi.org/10.1016/j.is.2026.102713

Published collector/refiner extraction approach combining heuristic and generative variants; mandatory research comparator, not a core dependency.

## TEXT_OCEL_CODE: Author implementation for Buss et al.

https://github.com/Alinabuss/OCEL_extractor_v2

Reproducibility candidate. Code licence not qualified in this review; do not copy/translate by default.

## OCED_QUALITY: Kabierski et al. (2026), Data quality in object-centric event data

https://doi.org/10.1007/s44311-026-00043-x

Map evaluation diagnostics to published object-centric quality issues.

## PROV: W3C PROV-O

https://www.w3.org/TR/prov-o/

Semantic crosswalk reference; retain minimal native JSON contracts, no mandatory RDF store.

## TIME: W3C OWL-Time

https://www.w3.org/TR/owl-time/

Temporal crosswalk reference; interval uncertainty remains in evidence sidecars.

## SKOS: W3C SKOS

https://www.w3.org/TR/skos-reference/

Versioned label mapping, broader/narrower relationships and explicit unknown mappings.

## Novelty boundary
Basic zoning, speech-act detection, task-to-actor assignment and text-to-OCEL extraction already have published precedents. A Rust rewrite alone is engineering, not evidence of a scientific advance. The research claim to test is whether raw-message/copy-aware, source-grounded, uncertainty-preserving reconstruction improves **downstream process conclusions**, at equal cost and privacy constraints. Compare reference methods, independent adjudication and cross-corpus held-out tests. Failure to beat a baseline is a valid result; retain the simpler baseline.

The frontier register is a dated shortlist, not proof of exhaustive novelty. Before a publication novelty claim, conduct a broader reproducible literature search, with Searchright optionally producing the search record.
