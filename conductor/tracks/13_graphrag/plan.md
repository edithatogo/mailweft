# Optional evidence-constrained GraphRAG — executable task plan

Generated from the canonical task manifest; edit that manifest, then run `python3 scripts/conductor_doctor.py --sync`.
A partial task is not unimplemented and not complete. Evidence and remaining acceptance work are explicit.

- [ ] Task: **T13.01 — Define a source-grounded graph projection contract**

  - Lane: `research`; dependencies: T07.05, T10.06.
  - Requirements: R38, R52, R53.
  - Allowed paths: `docs/retrieval/graph-projection.json`, `scripts/check_task_t13_01.py`.
  - Action: Define the graph view over existing objects/events/claims; do not introduce a second authoritative store or require a graph database.
  - Acceptance: Declared contract checker and independent fictional tests pass; this packet is not implementation of retrieval or generation. Runtime build/test selection is specified in a later implementation track after provider qualification.
  - Verification: `python3 scripts/check_task_t13_01.py && python3 scripts/validate_scaffold.py`.
  - Evidence: `conductor/tracks/13_graphrag/evidence.md`.
  - Task card: `conductor/tasks/T13.01.md`.

- [ ] Task: **T13.02 — Specify bounded deterministic graph and lexical retrieval**

  - Lane: `research`; dependencies: T13.01, T10.07.
  - Requirements: R52, R53.
  - Allowed paths: `docs/retrieval/graph-retrieval.json`, `scripts/check_task_t13_02.py`.
  - Action: Use lexical and bounded graph retrieval as the first no-model comparator; the stated limits are initial test settings, not optimized scientific claims.
  - Acceptance: Declared contract checker and independent fictional tests pass; this packet is not implementation of retrieval or generation. Runtime build/test selection is specified in a later implementation track after provider qualification.
  - Verification: `python3 scripts/check_task_t13_02.py && python3 scripts/validate_scaffold.py`.
  - Evidence: `conductor/tracks/13_graphrag/evidence.md`.
  - Task card: `conductor/tasks/T13.02.md`.

- [ ] Task: **T13.03 — Specify optional generation over retrieved evidence**

  - Lane: `research`; dependencies: T13.02, T06.05.
  - Requirements: R20, R52, R53.
  - Allowed paths: `docs/retrieval/graph-generation.json`, `scripts/check_task_t13_03.py`.
  - Action: Define a local provider boundary; Microsoft GraphRAG is an optional separately qualified research comparator, not a Python dependency of the converter.
  - Acceptance: Declared contract checker and independent fictional tests pass; this packet is not implementation of retrieval or generation. Runtime build/test selection is specified in a later implementation track after provider qualification.
  - Verification: `python3 scripts/check_task_t13_03.py && python3 scripts/validate_scaffold.py`.
  - Evidence: `conductor/tracks/13_graphrag/evidence.md`.
  - Task card: `conductor/tasks/T13.03.md`.

- [ ] Task: **T13.04 — Specify temporal scope and correction invalidation**

  - Lane: `research`; dependencies: T13.03, T10.08.
  - Requirements: R43, R52, R53.
  - Allowed paths: `docs/retrieval/graph-invalidation.json`, `scripts/check_task_t13_04.py`.
  - Action: Propagate revisions/retractions/access changes through graph, vectors and summaries; no silent stale fallback.
  - Acceptance: Declared contract checker and independent fictional tests pass; this packet is not implementation of retrieval or generation. Runtime build/test selection is specified in a later implementation track after provider qualification.
  - Verification: `python3 scripts/check_task_t13_04.py && python3 scripts/validate_scaffold.py`.
  - Evidence: `conductor/tracks/13_graphrag/evidence.md`.
  - Task card: `conductor/tasks/T13.04.md`.

- [ ] Task: **T13.05 — Specify gated retrieval-assisted enrichment**

  - Lane: `research`; dependencies: T13.04, T10.02.
  - Requirements: R21, R35, R52, R53.
  - Allowed paths: `docs/retrieval/graph-enrichment.json`, `scripts/check_task_t13_05.py`.
  - Action: Route all proposed semantic improvements through existing claim/evidence admission. Do not automatically write FOI-O concepts or treat graph plausibility as event truth.
  - Acceptance: Declared contract checker and independent fictional tests pass; this packet is not implementation of retrieval or generation. Runtime build/test selection is specified in a later implementation track after provider qualification.
  - Verification: `python3 scripts/check_task_t13_05.py && python3 scripts/validate_scaffold.py`.
  - Evidence: `conductor/tracks/13_graphrag/evidence.md`.
  - Task card: `conductor/tasks/T13.05.md`.

- [ ] Task: **T13.06 — Pre-register GraphRAG value and unchanged-core acceptance**

  - Lane: `research`; dependencies: T13.05, T11.01, T11.05, T11.07.
  - Requirements: R23, R44, R52, R53.
  - Allowed paths: `docs/retrieval/graph-evaluation.json`, `scripts/check_task_t13_06.py`.
  - Action: Use independent FOI gold and canonical example corpora; test injected false edges, stale summaries and missing evidence. Activate only after measured value and privacy/correction tests pass.
  - Acceptance: Declared contract checker and independent fictional tests pass; this packet is not implementation of retrieval or generation. Runtime build/test selection is specified in a later implementation track after provider qualification.
  - Verification: `python3 scripts/check_task_t13_06.py && python3 scripts/validate_scaffold.py`.
  - Evidence: `conductor/tracks/13_graphrag/evidence.md`.
  - Task card: `conductor/tasks/T13.06.md`.
