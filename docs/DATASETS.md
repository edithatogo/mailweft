# Raw corpora and validation datasets — controlled acquisition

## Source decision
**Default reference corpus:** CMU Enron 2015 distribution (https://www.cs.cmu.edu/~enron/) stores original email-like messages in maildir archive and has ~0.5M entries. It is suitable for Stage 1 headers/content and corpus-scale tests, **but CMU explicitly says attachments are absent**. Use synthetic attachment fixtures for initial Stage 2 and investigate a separately authorised source later.

**Secondary source:** EDRM Enron raw EML/XML archives, referenced by https://github.com/jamtho/enron-emails. These can include MIME attachments, legacy X.500/IMCEANOTES fields and XML companion metadata; dataset rights, retention/privacy, current authoritative source, archive integrity and non-redistribution constraints must be verified before download. Historical v2 was withdrawn/superseded for privacy concerns; **do not silently use an old mirror**.

**Processed reference:** Berti's Enron OCEL v3 (https://zenodo.org/records/21989732), available as OCEL representations. Treat as an **independent, non-authoritative reference** produced from another source/population until a verified per-message crosswalk exists. Do not generate event labels or train an extractor on this reference while calling it independent evaluation.

## Dataset registry fields
Record per source: dataset_id, title, steward, canonical URL, source/version/date, original licence text/evidence URL (not an inferred licence), allowed processing/redistribution, consent/privacy caveats, file format, SHA-256, acquisition date, record count, missing attachments, parser assumptions, redactions, permissible benchmark outputs, and approval for access.

## Matching protocol
1. Never compare raw-CMU to processed-EDRM or Berti per-email counts as if identical populations.
2. Derive candidate linkage using normalized Message-ID, From/To/Date/subject fingerprints and content fingerprints where allowable; require multiple independent concordant fields for exact match.
3. Label pairs `exact`, `likely`, `ambiguous`, `unmatched`. Store only synthetic or authorised pseudonymous mapping receipts. Human adjudication for uncertain links.
4. Report matching coverage, precision/recall from an adjudicated sample, population differences, duplicate/redaction effects, and leave-one-source-out sensitivity.
5. Structural OCEL parity does not imply agreement on semantic event meaning or that a message was sent/delivered.
6. Baseline Stage 1 with raw synthetic EML + small permitted Enron subset; test full corpus only after rights/resources qualified.

## Data never committed
Actual raw Enron messages, archive extracts, personal identifiers, extracted content, sensitive examples, URL tokens, model embeddings, paid API credentials, or legal/licensing notices copied from third-party datasets into a public code fixture. Citation and checksum receipts only where permitted.

## Additional task-oriented validation (proposed, subject to rights review)
- Quagga Enron and Apache annotated **email zones**, including identity aliases and detailed metadata: https://github.com/HPI-Information-Systems/Quagga ; GPL-3.0 source; assess corpus rights independently and do not add GPL code to an Apache-2.0 runtime.
- Talon/Forge Enron-derived **signature** annotations: https://github.com/mailgun/forge ; check repository and original source licences before any use or redistribution.
- Enron People Assignment (EPA), **task-to-person** gold labels: https://github.com/RevRameshkumar/EPADataset ; assess rights and corpus-version crosswalk before testing. https://aclanthology.org/W18-6104/
- Zebra nine-zone segmentation methodology: https://aclanthology.org/D09-1096/ ; paper metrics are historical reference points, not pass thresholds.
- Additional synthetic cases for missing Bcc, quoted/forwarded speakers, signature-derived role hints, aliases, rhetorical requests, multi-actor tasks and negated commitments. Keep evaluation split by thread/person, not random message alone.
