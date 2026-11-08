# Structural, semantic and frontier extension design — 0.2

**Status:** approved direction translated into a proposed implementation plan; no Rust feature implemented by this document. The product contract is unchanged: raw mail -> deterministic communications OCEL 2.0; opt-in enrichment produces separately versioned claims/OCEL and evidence sidecars. Current Stage 1 data/schema contracts stay valid.

## Approach and trade-offs

Selected approach: a small deterministic core, with optional, evidence-preserving enrichments. A monolithic LLM pipeline would obscure source attribution, create expensive dependencies and make failures difficult to localise. Embedding this in CareOps would violate the independent-library boundary. Neither alternative is adopted. Additional claims, evaluation and integration records are additive contract types; breaking existing fields requires a new schema version and migration tests.

## Staged semantics

1. **Source/copy layer:** raw RFC fields, groups, From/Sender distinction, To/Cc/Bcc, Reply-To, repeated/Resent blocks and header references. Preserve source-specific visibility. Absence of Bcc, reading receipts or response evidence is not proof of absence in the real process.
2. **Structure layer:** non-destructive spans for salutation, authored passages, inline replies, quote headers, signatures, sign-offs, disclaimers and attachment notices. Keep authored, identity and context views; masking is not deletion. Never treat a quoted header block as the current transport envelope.
3. **Acts layer:** utterance-level, multi-label Request, Commit, Propose, Inform, Deliver, Accept, Decline, Acknowledge, Delegate, Remind, Escalate, Withdraw, DecisionCommunicated and CompletionReported. `Thanks` does not automatically fulfil a task; `I will` is not `I have`.
4. **Interaction layer:** link acts to their antecedents and work items. Request -> clarification -> revised request -> commitment -> reported completion is a candidate interaction, not a fixed universal workflow. Conversation, topic and work item are not the same object; one thread may contain several work items and one work item several threads.
5. **Verification layer:** separately linked source-system facts may support/corroborate claims. No valid authority or task completion is inferred solely from communication patterns. Retrieval and generated summaries are never new source evidence.

MIME message kinds and business genres are separate from speech acts. Calendar invitations, delivery-status notifications, automated replies and list traffic can have structural labels without becoming human requests or evidence of receipt/read. Email-associated address objects are not automatically people; shared mailboxes, aliases, mailing lists and delegates need explicit scope-aware mappings.

## Source maps and attribution

Raw bytes -> decoded MIME part -> normalized text/HTML rendition -> zone -> utterance -> claim. Each transformation records input/output hashes, version and a many-to-many span map. No impossible one-to-one raw-byte mapping is promised after charset decoding, HTML removal or OCR. Where precise origin cannot be reconstructed, retain the enclosing part/document and mark mapping fidelity approximate; do not invent offsets. Offsets refer to UTF-8 byte positions in a named immutable rendition, not character indexes. Validate boundaries on Unicode examples.

Zoning is a partition within one layer plus explicit nested/overlapping annotations across layers. Retain bottom-posted and interleaved authored replies; never assume everything after the first quote delimiter is old content. A signature can supply a claimed title as of a source date, not authoritative organisation membership. Role/person resolution can remain unresolved with candidate IDs rather than forced merges.

## Evidence receipts rather than model self-certification

Every projected semantic assertion needs a source reference, attributed speaker, zone/origin, extraction configuration, bounded arguments and a projection-policy decision. Structural checks can prove a span exists and an output conforms to a schema; they cannot prove the interpretation is true. Use nullable scores and calibration status. Existing v1 numeric confidence fields are raw extractor scores until a separate receipt establishes calibration; never silently reinterpret them as real-world probabilities.

Receipt status records pass/fail/not_run/not_applicable for anchoring, attribution, role-domain validity, quotation handling, temporal plausibility and contradiction review. A failed mandatory check causes quarantine/abstention, not an invented repair. Human adjudication appends a revision. Multiple models agreeing on the same source are not independent corroboration.

## Time, alternatives and absence

Keep source observation, header-asserted time, claimed business-event time, interpretation time and valid intervals distinct. Record timezone/locale, granularity and evidence for expressions such as `Friday`; unresolved ambiguity remains an interval/candidate set. Preserve partial order and conflicting chronology. Stable ID tie-breaks are serialization choices, never causal order.

Uncertain event membership, actor links and object associations remain in a sidecar hypothesis set. Standard OCEL exports are named projections with a loss/omission ledger, not a new probability dialect. Compare conservative and permissive projections; optional downstream uncertainty analysis must state its assumptions. An unanswered request is `no reply observed within this admitted source/time scope`, not a proven ignored request.

## Incrementality and dependent evidence

Use `EvidenceDelta`-style revision semantics: upsert/supersede/retract a derived interpretation, with monotonic revision identity, prior-record hash, reason and idempotency key. Materialize current and as-of snapshots; invalidate dependent summaries, graph links and exports when a source/interpretation is corrected. Thread changes must not change source-copy IDs. Hash-keyed caches include parser, normalizer, model, prompt/config and ontology versions. Duplicated mailbox copies and quoted/forwarded text retain common lineage so confidence is not inflated by repetition.

Raw immutability is bounded by retention: lawful deletion removes protected bytes, mappings and derivative indexes/embeddings/summaries; retain only permitted tombstone metadata. Pseudonymous IDs/hashes are not assumed anonymisation. Authorization applies before retrieval and after graph traversal, with scopes retained on derived artifacts.

## Frontier experiments (not core requirements)

- **Calibrated selective extraction:** rules -> conventional local NLP -> optional local model only for eligible uncertain cases. Compare precision at equal coverage, abstention, cost and latency. Conformal techniques are candidates only with stated exchangeability/domain-shift limitations; no universal confidence guarantee.
- **Uncertainty propagation:** measure how ambiguous actors/events change process edges, variants and bottleneck rankings; compare inference outputs with independently adjudicated and latent-truth fixtures.
- **Conversation-aware reconstruction:** discourse antecedents, work-item splitting/merging and commitment transitions, with quoted/forwarded dependence and temporal alternatives.
- **Counterfactual/metamorphic evaluation:** controlled perturbations of addresses, salutations, signatures, quotations, negation, modality and deadlines. Expected deltas are fixed before model execution; not an observational causal claim about people.
- **Active learning:** prioritize adjudication by expected error reduction per review cost with random-review controls; do not put research economics libraries in the core.
- **Graph retrieval:** deterministic typed retrieval first, then vector/graph/GraphRAG comparisons at matched scope/cost. Query answers require admissible source evidence and must not re-enter the event store as observations.
- **Synthetic worlds:** independently authored latent workflow fixtures -> independent email renderer -> blind reconstruction. Generated text is input, not ground truth; generator/model overlap, contamination and simulator domain bias must be reported.

## What not to infer

No personality, motives, wrongdoing, strategic behaviour, competence or individual performance scores from address position, response speed or speech acts. No automated workflow action, mail send, evidence collection or cloud call. Domain decision rules, legal/clinical interpretations, simulation and economic valuation are consumer responsibilities.

## Verification and release

Core acceptance: synthetic raw email -> complete typed OCEL 2.0 -> qualified Rust4PM read/round-trip, with bounded resources and no silent qualifier/attribute loss. Body understanding is an optional increment and can start as soon as safe MIME decoding/source maps exist; it does not wait for full Enron acquisition or document OCR. NLP, advanced attachments, graph experiments, optional adapters and full-corpus benchmarking cannot block the independently releasable core.

External standard correction: JSON/XML/SQLite are the OCEL 2.0 baseline. The current standard site describes CSV and bundled CSV/Parquet under OCEL 2.1. Preserve 2.0, and add later interchange only as a version-labelled optional profile with feature/loss tests.

## Scope correction and stronger comparison
Only `foi-process` remains in the active integration queue, as a downstream consumer. See `FOI-SCOPE-AND-ROBUSTNESS.md` for source-fidelity, defect-led migration, prior-art limits and multi-layer error-impact experiments. No other owned-repository provider is selected by this document.
