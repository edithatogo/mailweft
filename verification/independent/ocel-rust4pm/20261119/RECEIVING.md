# Mailweft: independent OCEL / Rust4PM receiving qualification

**Review scope:** PR [#3](https://github.com/edithatogo/mailweft/pull/3), release blocker [#4](https://github.com/edithatogo/mailweft/issues/4), and the separate experimental Rust4PM remediation branch. **Receiving decision:** RETURN / NOT ADMITTED (20 November 2026). **No approval, merge, publication, waiver, source implementation change, or Conductor task completion.**

**Ownership:** independent receiving review lane only. Existing native/security/Conductor implementation owner: ChatGPT conversation `6af02cc1-56ec-8322-87df-7037f1c22500`. PRs #25 (Track 04 receiving) and #26 (fuzzing) are separate, unmodified lanes. This file is a handoff/evidence index, not a substitute for `conductor/task-manifest.json`.

## Exact recovered reproducibility bundle

The original **2026-11-19** 12-entry ZIP was recovered without modification and byte-verified. Its exact bytes are preserved in the account's ChatGPT Library:

`/GitHub/mailweft/ocel-receiving/mailweft_ocel_receiving_qualification_20261119.zip`

- **Archive SHA-256:** `5a957c58e4bf69f45c13b01d6a048e509cd207cc911052be7ff5ceacaf8a2829`
- **Archive byte size:** 14,474
- **Archive Git blob SHA-1, for identity if accepted into Git:** `0913801ac52e0ae380c85a753d3075d2cf124b33`
- Contents: original `RECEIPT.md`, `SHA256SUMS`, four exact read-only candidate Python source/test snapshots, independent 10-case OCEL negative/positive suite, independent 5-case SQLite semantic oracle, actual test-output transcripts, and a fictional 65,536-byte SQLite fixture. There is no real correspondence, Enron corpus, personal key, or production source modification.
- Library location was **proposed on 20 November 2026 and explicitly accepted by the owner on 28 November 2026**. The reviewer also preserved a local downloaded copy. This acknowledgement confirms the evidence receiving location only: do not publish this ZIP as a released product or imply its included copies are new implementation.
- The five independent failures are *expected RED acceptance regressions* and must not be hidden/skipped. The oracle does **not** run Rust4PM.

## Recheck execution (20 November 2026, Linux Python 3.13.5, SQLite 3.46.1)

Extract the unmodified archive to an empty temporary folder, from that folder run:

```sh
sha256sum -c SHA256SUMS
python3 -m unittest discover -v -s scripts -p 'test_*.py'  # exit 0: 13 passed
python3 independent_ocel_cases.py                          # exit 1: 5 passed, 5 expected RED failures
python3 sqlite_semantic_oracle.py                            # exit 0: 5 passed
python3 - <<'PY'
import sqlite3
db = sqlite3.connect("synthetic_ocel2.sqlite")
assert db.execute("PRAGMA integrity_check").fetchone() == ("ok",)
assert db.execute("PRAGMA foreign_key_check").fetchall() == []
db.close()
PY
```

The package's four published candidate Python files were **Git blob SHA-1 validated** before execution:

| Original path | Blob SHA-1 |
|---|---|
| `scripts/validate_ocel_integrity.py` | `a68c40b2c2873d96f8f9cfd781e3485a0bbddbda` |
| `scripts/test_ocel_integrity.py` | `60f33f0809e925a586802d7c0cc4a3b40076e2bb` |
| `scripts/release_guard.py` | `a18dbf8fa0ca36584d447abda932210d5f1f4f36` |
| `scripts/test_release_guard.py` | `5274390d2cb4cc513caf5346ab770768a4f83177` |

**Five reproduced rejector defects in that specific historical Python validator:**
1. Malformed event timestamp accepted.
2. Malformed object attribute-history timestamp accepted.
3. Duplicate event attribute names accepted when their extra time fields differ.
4. Unsupported attribute type declaration accepted when unused.
5. Timezone-naive string accepted as a declared OCEL `time` value.

These defects pertain to candidate `16948cb...`; the version on current main has a **different Git blob** and must be evaluated separately by its owner. The independent SQLite oracle proves that declaration ordering can differ without semantic corruption, while qualified links, primitive typed values, attribute histories and multiplicities must remain checked.

## Immutable reviewed source and supply-chain state

| Snapshot | Commit | `Cargo.lock` blob | quick-xml | Known quick-xml check |
|---|---|---|---|---|
| main | `890eb487e3908c8c41e2313c0333c90af8e4d03b` | `59c727a76feb95b5aad1340a36e7d90ed4b34808` | 0.37.5 | BLOCKED |
| draft PR #3 | `520d545951e9c722fff857674081608427ca2ec5` | `88fc060fae0cf6a37001b6ac8affad8e431f4a31` | 0.37.5 | BLOCKED |
| remediation candidate | `16948cb586f40cf608914f5bfb90f173f4cbf248` | `bf077a07a44c429fc17a39b229443021f8a19b09` | 0.41.0 | PRECHECK ONLY |

Known version-floor check covers RUSTSEC-2026-0194 and RUSTSEC-2026-0195; absence of these two known version blocks is **not** fresh full vulnerability, licence, source, or maintenance acceptance. The candidate selects `process_mining 0.6.2` from reviewed fork revision `7d8d72a139196deb170de7d6442b7c3df6512315`, but independent source/lifecycle and exact-feature qualification remain incomplete.

**Actual inspected hosted native log, not a rerun:** [run 42768686192, job 129671130529](https://github.com/edithatogo/mailweft/actions/runs/42768686192/job/129671130529) compiled quick-xml 0.41.0 and passed six JSON/XML Rust4PM tests, but its SQLite feature run failed 1/7 at `crates/mailweft-core/tests/rust4pm_interop.rs:184` on its first order-sensitive object-type vector comparison. Later object/event/relationship assertions were not reached. No fresh native, cross-platform, Clippy, cargo audit, cargo deny, DB revision or normative OCEL schema pass is claimed.

## Conductor disposition, read-only reconciliation

As of this review of canonical **main** manifest `e1c1b1346e2d29a017030d180d400245f0b9b77b`:

- Track `03_ocel`: `in_progress`; T03.03–T03.06 remain `in_progress`, T03.07–T03.08 remain `not_started`. Full native fidelity/feature acceptance is unresolved.
- Security CI T00.06: `in_progress`; supply-chain T07.07: `not_started`.
- Track `08_release`: `new`; T08.06–T08.08: `not_started`.
- Issue #4 is open; PR #3 remains draft/open, not merged. The remediation branch is not PR #3.
- **No task/track metadata, status, code, lock, workflows or registry files are modified by this receiving branch**. Only the existing owner may reconcile the canonical manifest and its generated Conductor artifacts.

## Existing owner: acknowledgement received; implementation fixes remain

**28 November 2026 owner receiving acknowledgement:** In this ChatGPT conversation, the user stated, “I'm the owner and I agree” in response to the explicit request to accept the original 14,474-byte ZIP (SHA-256 `5a957c58e4bf69f45c13b01d6a048e509cd207cc911052be7ff5ceacaf8a2829`) at `/GitHub/mailweft/ocel-receiving/mailweft_ocel_receiving_qualification_20261119.zip`. This accepts that location as the independent *synthetic evidence* receiving archive and completes the reviewer's receipt/coordination gate. The connected GitHub record relays the ChatGPT acknowledgement; it is **not** a separately submitted GitHub review or approval of native, security, merge or release gates. A future move into GitHub requires its own approval and verified upload. No Conductor or implementation ownership is transferred.

Owner retains:
- Validator rejection repairs and normative OCEL schema qualification.
- Canonical **lossless** JSON/XML/SQLite round-trips from original source meaning across primitive types, all time-varying histories, IDs, E2O/O2O qualifiers, multiplicity, i64 boundaries and scoped HMAC identity compatibility with PR #3.
- Patched fork provenance, maintenance/retirement plan, complete locked feature graph and full unsuppressed audit/deny/licence/source evidence.
- Pinned-toolchain exact-head native tests for relevant features, fmt/Clippy and Linux/macOS/Windows CI, followed by independently reviewed Conductor disposition.
- Release blocker #4 must remain open until substantive requirements pass; **do not merge PR #3, publish, waive gates or claim this reviewer branch supplies native acceptance**.

Earlier independent evidence: [15 Nov](https://github.com/edithatogo/mailweft/issues/4#issuecomment-6889450827) and [19 Nov](https://github.com/edithatogo/mailweft/issues/4#issuecomment-7009259457). Reviewer execution was completed on 20 November; **owner receiving acknowledgement was obtained on 28 November, so this review lane is complete**. The source/binary reproducibility bundle remains a non-release Library artifact; this draft evidence PR is separate from production code. Issue #4 and PR #3 remain unresolved implementation/security work.
