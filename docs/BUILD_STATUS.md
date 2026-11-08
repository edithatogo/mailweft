# Verification status — Mailweft publication, 8 November 2026

## Current revision

This revision updates local baseline `e1c201b` for the accepted `mailweft` repository. It is planning, contract and development-validation work only. All **116 application/research/integration tasks** remain `not_started`.

Inventory: **53 requirements, 15 tracks, 116 tasks, 38 detailed task cards and 11 unchanged schema contracts**. The original seven scope-regression tests were run against the inherited validator: the approved profile passed, but six invalid scope profiles incorrectly passed. Enforcement was then added. Fresh checks below verify rejection of those profiles; they are not Rust application tests.

## Local checks actually executed

- `python3 scripts/validate_scaffold.py`: PASS, 53 requirements / 15 tracks / 116 tasks / 11 schemas / 38 cards; exact task DAG, requirement references, registration and independent core closure.
- `python3 -m unittest discover -s scripts -p 'test_*.py'`: PASS, **15 tests** (eight inherited planning checks plus seven scope checks).
- `python3 scripts/validate_contract_examples.py`: PASS, **11 Draft 2020-12 metaschemas, four positive fictional examples and 18 rejected negative variants**.
- `git diff --check`: run before publication; whitespace failure blocks commit.
- Schema bytes and original Conductor gitlink are retained. Accepted name changes product/crate plans, not pre-existing schema identities.

The schema examples are independent fictional records; validation does not establish cross-record integrity, source truth, privacy enforcement, calibration, process accuracy or real-world effectiveness.

## Publication and Conductor

The remote target is `edithatogo/mailweft`. Publication uses a verified Git-history transfer and a merge that preserves the prior planning commits. A separate publication receipt records the actual remote SHA and readback after success; this document does not imply a future write happened.

Gemini Conductor is an actual gitlink, mode `160000`, at `.agents/plugins/conductor`, pinned to `7a5c560a4fdf5297be58594cc37527eb12790272`. The local authoring environment cannot reach GitHub, so its upstream checkout is uninitialized. The hosted publication/check job is intended to initialize and verify that pinned source. Host-specific plugin registration remains a separate user-environment action; checking out the source is not proof of a Gemini or other host extension installation.

The read-only planning CI verifies the gitlink/checkout and the planning tests. It does not run or claim any converter, NLP, corpus, graph or domain benchmark. No real emails, credentials, research contacts, correspondence drafts, model weights or operational source payloads belong in this publication.

## Product and boundary checks

Mailweft remains **an email-to-OCEL converter written in Rust**. Enron is a canonical verification/example corpus, never required input or the product domain. `foi-process` is the first consumer; legacy outputs are explicitly excluded from reference/oracle/teacher/equality roles. `fyi-cli` and the archive boundary retain acquisition/preservation ownership; `foi-o` owns reviewed ontology promotion. GraphRAG is optional and outside the core dependency closure.

No Rust application was implemented or compiled; no real-data processing, external model execution, FOI migration, ontology update or collaboration email was performed by the planning revision.

## Clean-host validation correction

The initial hosted history import, run `42475424694`, verified all nine transport
parts, reconstructed the exact source commit/tree, and initialized the pinned
Conductor checkout. The scaffold check and 15 planning tests passed. Publication
stopped because the negative `bad timestamp` example was incorrectly accepted.

Root cause: `jsonschema.FormatChecker` registers RFC 3339 date-time checks only
when its optional checker is installed. The local environment contained it, but
the clean-host requirements omitted it. The corrective change pins
`rfc3339-validator==0.1.4` and `six==1.17.0`, and explicitly returns BLOCKED if the
date-time checker is missing. Two new regression tests first failed for the
missing dependency pins and absent fail-closed behavior.

After the correction, the local planning suite passes **17 tests**, and all
11 schema checks / four positive / 18 negative examples pass. The source
application remains unimplemented. A successful rerun and remote publication
readback must be recorded separately; this correction does not imply they ran.
