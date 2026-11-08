# Email structure, participant semantics and discourse-act taxonomy

**Status:** design specification, not implemented. **Source:** raw EML/MIME. All tests synthetic until independent dataset rights are checked. **Updated:** 2026-11-08.

## Three orthogonal classifications

1. **Transport/header participant roles** (`from`, `sender`, `reply_to`, `to`, `cc`, `bcc`, `resent_*`, `return_path`/envelope evidence): source-supported assertions. Preserve header field, parsed span, mailbox/source-copy provenance and confidence in parse. A `to` participant is not assumed to own a task, `cc` does not mean passive observer, `from` need not be the SMTP sender, `reply_to` is a route, and absence of `bcc` is not evidence of no Bcc. `Resent-*` refers to a distinct re-sending context. A source copy may expose fewer recipient facts than other copies. Multiple source copies must not create multiple independent messages without evidence.
2. **Text-zone function** (authored body, greeting/salutation, sign-off, signature/contact-card, disclaimer, advertising/list footer, original quoted reply, forwarded block, quoted inline header, attachment notice, auto-generated template). Zones may nest, overlap or occur repeatedly, and have source span, body-part source and claimed author attribution. MIME alternative versions must be linked as alternative renderings, not double-counted as two authors' utterances. This is **label and select**, not destructive stripping: original bytes stay in protected source storage and signature information can contribute to person, organisation and role linking.
3. **Utterance/dialogue acts** (inform/deliver, request, ask, propose, amend, commit/promise, accept, reject/refuse, delegate, remind, escalate, report completion, decision communicated, acknowledgement, other/uncertain). Multi-label at span/utterance level, not mutually exclusive whole-message classification. Keep actor, requested performer, object, modality, negation, actual vs claimed event time, and evidence provenance distinct. A cited/quoted request is not a new request by the present sender.

## Minimum source/evidence model

- Header role facts are `observed_header_claim` and retain source field and copy; in-text identification is **inferred** unless directly quoted as an attributed claim. Never silently promote a salutation, signature, forwarded inline header, or quote into an authoritative RFC header.
- Store byte offsets against decoded MIME part, with explicit byte map/source content hash and original raw-part reference; normalization must preserve a mapping back to the raw source. Use a parent-child/overlap structure so nested quotation and signature inside forwarding can coexist. Hash and scope sensitive text; never place it in OCEL/CI logs.
- Keep an explicit `semantic_analysis_view` projection that defaults to newly authored text plus context (subject/header participants). Omitting signatures/disclaimers/quoted material from **one model input** must not delete them from source or evidence. Give a separate `identity_view` containing permitted signature/contact information and `thread_context_view` containing attributed quotations, with appropriate authorization.
- Bcc visibility has three states: observed populated, observed empty, unknown/not present; preserve observed Bcc with stricter disclosure controls. Do not infer Bcc addresses merely from receiving a copy without an approved independent envelope record.
- Relationship names are scoped: `header_recipient_to`, `header_recipient_cc`, `header_recipient_bcc`, `text_addressee`, `text_named_person`, `text_signatory`, `task_candidate_actor`, `task_verified_actor`. Their evidentiary status is not interchangeable.
- Quote attribution is separate from content act classification; if quote provenance is uncertain, abstain from classifying it as a fresh speech act.

## Optional fourth (coarse) classification: email genre / workflow context

Genres are **multi-label contextual summaries**, not proof of individual events: human personal message, transactional work correspondence, meeting coordination, information sharing/FYI, decision circulation, approval request, complaint/escalation, automated notification, calendar invite, bulk/list message and unsolicited commercial email. Some are deterministically identifiable from MIME/calendar/List-* headers; others remain inferred. Classification should be calibrated and non-exclusive. Do not infer obligations from message genre.

## Process-friendly stages

1. **Stage 1:** parse outer EML, MIME parts, addresses, sender/recipient qualifiers and copy/thread references; export the observed communication OCEL independently of any NLP.
2. **Stage 1.5 (optional but first content stage):** supervised/deterministic zoning of decoded parts; extract text addressees and signature identity hints; provide both original and analysis views. No task assignment or business act asserted yet.
3. **Stage 2:** extract utterance-level acts and role-typed argument structures only from explicitly attributed spans; then link to participant identities, work items and temporal process events.
4. **Stage 3:** independently reconcile semantic claims against actual task, decision, file/version and other system facts before promoting any business event to a verified process observation.

## Historical reference classifications

- **Lampert, Dale and Paris (EMNLP 2009), Zebra:** nine-zone historical classifier. Our detailed zone labels below are a proposed project vocabulary; verify a crosswalk against the paper before claiming equivalence. https://aclanthology.org/D09-1096/
- **Repke and Krestel (ECIR 2018), Quagga:** two-/five-zone email segmentation, including detailed Enron/ASF annotations and entity aliases; GPL-3.0 code. Reference/benchmarks only pending dataset-licence assessment, not a runtime dependency. https://github.com/HPI-Information-Systems/Quagga
- **Cohen, Carvalho and Mitchell (EMNLP 2004):** email speech acts as verb–noun combinations, including request, commit, propose and deliver. https://aclanthology.org/W04-3240/
- **Carvalho and Cohen (2006):** n-gram selection for speech-act classification; use the specific paper, not whole proceedings. https://aclanthology.org/W06-3406/
- **Lampert, Dale and Paris (AAAI 2008):** utterance-level request and commitment definitions for Enron. https://researchers.mq.edu.au/en/publications/the-nature-of-requests-and-commitments-in-email-messages/
- **Rameshkumar, Bailey, Jha and Quirk (W-NUT 2018), EPA:** Enron People Assignment benchmark for linking task intent to responsible person. https://aclanthology.org/W18-6104/ and https://github.com/RevRameshkumar/EPADataset
- **Talon / Forge:** signature and quotation segmentation methods and annotated Enron-derived examples; independently verify dataset/code licences and reuse restrictions. https://github.com/mailgun/talon and https://github.com/mailgun/forge
- **Task taxonomies:** e-mail act, expected reply, number of tasks, time sensitivity and task type; use as *auxiliary labels* rather than assume every email is about a task. https://doi.org/10.1016/j.ins.2016.03.002

## Annotation and benchmarking

- Create synthetic controlled message sets covering single/multiple To, CC, Bcc omission, observed Bcc, group addresses, multiple From/Sender, Reply-To, re-sent and aliases, Outlook/Exchange inline headers, salutation naming someone not in To, signature for assistant/delegate, short replies and multiple tasks.
- Span-level zone gold labels: macro F1, per-class precision/recall, boundary F1 and provenance/offset fidelity. Include quote attribution confusion, unknown/ambiguous class, multilingual and HTML-to-text normalization. No arbitrary numeric performance claims without calibration.
- Participant-level metrics: header-role exact match and recall, Bcc leakage tests, in-text mention precision/recall and person-resolution accuracy conditional on provenance. Task-to-actor metrics using EPA if permitted; split by conversation/person/custodian to prevent identity leakage, not random messages only.
- Discourse-act metrics: multi-label precision/recall and calibration; require specific negative examples (quoted request, hypothetical, negated commitment, delegated/collective tasks, polite question vs actual request).
- Evaluate deterministic/rule-only baseline before local model; compare Quagga/Zebra/Talon only as reference if reproducible and legally usable; never ship their code or corpus in this repository by default.
- Prioritise **minimising false new events and wrong addressees**. High-confidence extraction plus abstention is more useful than silently filling all missing roles.

## 0.2 integration into the wider pipeline
This document's label vocabulary is project-proposed; historical crosswalks must be verified against each paper/annotation guide rather than attributed verbatim. Use [RELATED-WORK.md](RELATED-WORK.md) for corrected, specific references. In particular, the previously cited whole 2006 workshop is not a precise collective-classification experiment reference.

Add an independent interaction layer after utterance acts, per [FRONTIER-DESIGN.md](FRONTIER-DESIGN.md). No whole-email genre determines all utterance acts. No To/Cc position assigns responsibility. Non-destructive source maps precede zoning; bottom-posted and inline responses survive segmentation. A hypothesis that two messages concern the same work item is not an identity fact.
