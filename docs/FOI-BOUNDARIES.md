# FOI ownership and ontology feedback

## Direction and responsibilities

```text
fyi-cli: authorised acquisition/capture
    -> fyi-archive boundary: preserved bytes + manifests/provenance
    -> foi-process: selects a pinned source scope and invokes Mailweft
         -> Mailweft: generic raw-email conversion / optional interpretations
         <- OCEL + evidence + completeness and loss receipts
    -> foi-process: applies a pinned FOI-O profile; validates FOI findings
    -> foi-o: receives evidence-linked ontology improvement candidates
```

| Owner | Responsibility | Must not be moved into Mailweft |
|---|---|---|
| fyi-cli | Authorized network enumeration/capture, actual source retrieval and capture receipts; live request operations under its own permission boundary | Crawling FYI, request submission, source login, automatic mail retrieval |
| fyi-archive archival boundary | Snapshot/manifests, provenance, preservation, distribution and mirror verification; accept an explicitly documented successor without changing the logical contract | A second archive, republishing source content, inventing acquisition receipts |
| Mailweft | Generic MIME/header parsing, identity/participant evidence, communication OCEL and optional typed, attributed interpretations | FOI law, authority policy, statutory clocks, ontology approval or platform certification |
| foi-process | First consumer; source selection, FOI mapping, independent domain validation, analyses and derived evidence packs | Treating legacy output as truth or importing FOI terms into Mailweft's core |
| foi-o | FOI vocabulary, relationships, constraints, competency questions, versioned process/jurisdiction profiles and promotion criteria | Automatic ontology changes inferred from prevalence, an LLM answer, or unreviewed extracted claims |

Only `foi-process -> mailweft` is a new runtime dependency. FOI-O profiles are consumer-owned, pinned input artifacts; generic Mailweft does not import the ontology project. FOI results can improve the ontology without creating a software dependency cycle.

## Observed documentation conflict (8 November 2026)

- `fyi-archive/README.md` blob `9afd72e55eca773b7fa06cc2fe6c81541e36cdfa` assigns all capture/network work to fyi-cli and orchestration/manifests/distribution to fyi-archive.
- `foi-o/README.md` blob `75360a339fee1377b8606e70c1de9d7b07b84128` describes archive work as succeeded by fyi-cli, while a later historical paragraph still describes fyi-archive as the publication boundary.
- GitHub metadata returned `archived: false` for fyi-archive in this review. This does not establish which operational commands currently own the complete workload.

T12.02 must verify the exact producer/manifest version and operational handover. Do not silently archive, merge or move another repository based on these conflicting descriptions. Mailweft consumes supplied evidence regardless of which qualified producer owns the archive boundary.

## Source fidelity
Accept `raw_rfc5322`, `platform_message_export`, `rendered_web_message` and `attachment_artifact` as distinct evidence classes in the consumer packet. Only the first enters the strict raw-MIME core. Other forms can use an optional text/evidence adapter and must not acquire invented headers, MIME parts, Bcc recipients or transport timestamps. Not-observed is different from known-absent. The owner controls authorized access and archived source completeness.

## Independent migration
1. Freeze source hashes and independently authored expectations before seeing old/new analysis outputs.
2. Inventory available raw bytes, source transformations, missing records and current endpoint ownership.
3. Reproduce individual legacy defects for diagnosis only. No equality-to-legacy acceptance criterion is permitted.
4. Evaluate Mailweft against an independently annotated FOI holdout and invented end-to-end fixtures; split whole requests/conversation components, and include domain-negative cases.
5. Qualify consumer mappings for qualifiers, typed values, temporal histories, identifiers, source states, losses and revisions. Do not silently drop fields to match an old exporter.
6. Switch the consumer only after actual downstream acceptance. Recovery means a validated snapshot or disabling/quarantining the new analysis; do not label the unvalidated legacy pipeline a safe rollback baseline.

## FOI-O contribution contract
The consumer emits a versioned `OntologyContributionPacket` with:
- packet ID, consumer/Mailweft revisions, source/annotation version, access class and sanitized source references;
- exact FOI-O base version and target term/relation/constraint/competency-question identifiers;
- candidate kind: missing concept, ambiguous definition, missing relationship, counterexample, constraint change, or new regression/competency test;
- observed/asserted/inferred/corroborated status, evidence spans, uncertainty, scope/denominator and known missingness;
- positive examples, counterexamples, competing interpretations and independent validation result;
- proposed change and test expectation, review status (`proposed`, `accepted`, `rejected`, `deferred`) and decision provenance.

Packets are **candidates**, not automatic RDF/OWL changes or legal conclusions. Legal-profile proposals retain their own expert review. Exclude raw mail, personal identifiers and restricted passages from public packets. Describe empirical patterns without equating frequency with normative correctness. A profile improved from a dataset must be re-evaluated on held-out requests; do not validate a new ontology using the same self-generated labels that created it. Store rejections and unresolved terms, not just favorable examples.

Implementation home: consumer-owned adapters and FOI-O contributions. This Mailweft track defines contracts/fixtures and handoff only; completion does not imply that remote sibling repositories have been changed.
