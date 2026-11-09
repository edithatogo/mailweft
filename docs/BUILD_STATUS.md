# Current build status — Mailweft 0.7

The repository contains a working experimental Rust email-to-OCEL converter, not only a planning scaffold. Implemented: raw EML and bounded batch conversion, header participant/chronology claims, source-copy identity, MIME metadata, conservative reply candidates, source-integrity receipts, source-selection reconciliation, non-destructive zones and bounded transfer-unit provenance.

Conductor project setup is reconciled: 116 tasks across 15 tracks, with 14 completed acceptance contracts, 12 partial/in-progress and 90 not started. The doctor verifies generated state, the real gitlink/checkout, command manifests, the pinned command's track-heading grammar, completed-task prerequisite acceptance and task/plan/metadata alignment. User-host extension registration is not asserted. Sync never declares a task completed.

## Executed qualification

- Preparation run **42595289280** qualified the initial reconciliation and mapping changes before generated context commit **3049851d023f2e592277133063c6069e8cf5c2c1**.
- Exact source head **4ef2bf7160320f26728515c46868e09fc22ef61d** passed ordinary read-only Rust CI **42596001104** and planning CI **42596001113**.
- Final review then identified a genuine Conductor protocol mismatch: list-style track entries were not the `##` headings required by the pinned implement command. Two new behavior tests failed at **38a3ff868de56276a76923e248c8102db6ff32e0**, run **42626883968**.
- The corrected registry/doctor passed all **25 planning tests** on **18e258657764bcae69c65675ab905fbc61eb9571**, run **42627398022**. Core Rust source was unchanged by that correction.

Final PR and post-merge workflows must qualify the final exact commit again. Consult their actual statuses; this document does not predict future runs. See `docs/VERIFICATION-0.7.md` and `docs/CONDUCTOR-COMPATIBILITY-REVIEW.md`.

## Release is not certified

**Dependency-security issue #4 remains open**: https://github.com/edithatogo/mailweft/issues/4 . The advisory findings recorded there concern the Rust4PM validation dependency graph. This batch does not suppress, remediate or independently re-audit them. Successful functional tests do not mean the dependency graph is security-cleared. Track T00.06 remains partial, and no package release or production deployment is approved by these checks.

The older alternative implementation in draft PR #3 is not merged by this change and is not used as a correctness oracle. Its useful hardening work requires separate reconciliation; the draft's security blocker is not erased by newer passing tests.

No real Enron/FYI correspondence was processed. No complete typed-OCEL/format parity, multilingual NLP accuracy, GraphRAG performance or live FOI consumer correctness is claimed. Historical publication receipts remain historical, not current status. The task manifest explicitly retains unfinished acceptance.
