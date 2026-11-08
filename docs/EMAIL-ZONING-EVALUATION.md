# Independent email zoning benchmark protocol (planned)

All benchmarks are UNRUN, and all third-party corpus licences and permissions are UNVERIFIED until inspected. Never download datasets automatically in CI.

## Candidate benchmarks
- Zebra (Lampert, Dale, Paris 2009): compare nine-zone taxonomy at span/line level. https://aclanthology.org/D09-1096/
- Quagga (Repke, Krestel 2018): test two-/five-zone segmentation on Enron/ASF; check GPL-3.0 code vs separately licensed corpus annotations. https://github.com/HPI-Information-Systems/Quagga
- Talon/Forge: annotated signatures and quotations. Confirm source-rights and split design. https://github.com/mailgun/forge
- EPA: task-to-person assignment benchmark; differs from simple To/Cc roles. https://github.com/RevRameshkumar/EPADataset

## Metrics and controls
1. Rights + exact corpus version and checksum, no blind reuse. 
2. Split by thread/person/custodian and distinguish Enron versions and raw-message populations; no dataset leaks.
3. Zone macro F1, per-class F1, span-boundary F1, byte-offset fidelity and quote-attribution errors; always include confusion matrix.
4. Header role extraction micro precision/recall and selective coverage; track invisible/unknown Bcc as unknown.
5. In-text mention precision/recall, alias resolution, wrong attribution and task-to-actor precision/recall; label quoted speakers separately.
6. Deterministic baseline versus optional model results, confidence calibration and abstention curves. No automatic causality or process-status claims.
