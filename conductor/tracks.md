# Execution tracks (noninteractive Conductor)

Status: [ ] not started; [~] in progress; [x] accepted. Agent should complete first unchecked task in first unblocked track.

- [ ] **00 — Foundation, legal/source gates and Rust workspace** ([spec](tracks/00_foundation/spec.md), [plan](tracks/00_foundation/plan.md)); requirements: R10 R12 R13 R15; prerequisite: none

- [ ] **01 — Raw EML and maildir input, manifests and parse boundaries** ([spec](tracks/01_ingestion/spec.md), [plan](tracks/01_ingestion/plan.md)); requirements: R01 R02 R03 R09 R11; prerequisite: 00 foundation; 

- [ ] **02 — Message/custodian identity, duplicates and conversation inference** ([spec](tracks/02_identity/spec.md), [plan](tracks/02_identity/plan.md)); requirements: R04 R05 R06 R19; prerequisite: 00 foundation; 01 previous track baseline

- [ ] **03 — Canonical OCEL 2.0 projection and Rust4PM verification** ([spec](tracks/03_ocel/spec.md), [plan](tracks/03_ocel/plan.md)); requirements: R07 R08; prerequisite: 00 foundation; 02 previous track baseline

- [ ] **04 — Synthetic and Enron raw-to-processed validation** ([spec](tracks/04_data-validation/spec.md), [plan](tracks/04_data-validation/plan.md)); requirements: R11 R13 R14 R24 R17; prerequisite: 00 foundation; 03 previous track baseline

- [ ] **05 — Opt-in text and attachment content extraction** ([spec](tracks/05_attachments/spec.md), [plan](tracks/05_attachments/plan.md)); requirements: R16 R20; prerequisite: 00 foundation; 04 previous track baseline

- [ ] **06 — Versioned NLP claims, entity resolution and evidence sidecar** ([spec](tracks/06_semantics/spec.md), [plan](tracks/06_semantics/plan.md)); requirements: R20 R21 R22; prerequisite: 00 foundation; 05 previous track baseline

- [ ] **07 — Quality, security, scaling and optional knowledge graph** ([spec](tracks/07_research/spec.md), [plan](tracks/07_research/plan.md)); requirements: R18 R22 R23 R24 R26; prerequisite: 00 foundation; 06 previous track baseline

- [ ] **08 — Interoperability, packaging and downstream consumer boundaries** ([spec](tracks/08_release/spec.md), [plan](tracks/08_release/plan.md)); requirements: R12 R16 R25; prerequisite: 00 foundation; 07 previous track baseline
