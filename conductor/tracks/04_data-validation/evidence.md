# Track 04_data-validation — reconciled evidence

Canonical acceptance declarations: `../../task-manifest.json`. Current CI must pass before merge.
Earlier checkpoints remain in Git history; initial NOT_STARTED assertions are not current status.

No implementation acceptance is declared for this track. Specifications and planned tasks are not runtime evidence.


## 2026-11-19: one synthetic raw-to-processed receiving qualification (evidence only)

**Ruling: bounded source/receipt evidence, not T04.01 acceptance.** Canonical main at review: `890eb487e3908c8c41e2313c0333c90af8e4d03b`. No raw private mail, Enron material, public download, FOI data, or core/OCEL implementation changes were used.

### Fixture and independent byte oracle

- Independently constructed the fictional 81-byte CRLF RFC 5322 specimen used in `crates/mailweft-core/tests/source_integrity.rs`, `crates/mailweft-cli/tests/corpus_reconciliation.rs` and `scripts/validate_runtime_contracts.py`: header fields From `alice@example.invalid`, To `bob@example.invalid`, Subject `Fixture`, followed by `Hello\r\n`. No Message-ID, claimed delivery, or real sender is asserted.
- Independently calculated SHA-256 `2ba2c8f860bfc5f2b6d4c13d4cfb0d8eb234239e432880195d9e4579ef87951e` and length **81**, matching the version-controlled `fixtures/synthetic/contracts/source-envelope.json` (Git blob `a2bea67f687265fc6db84d5c74171bdf1bf609ff`). Envelope: version `1.0.0`, fidelity `raw_rfc5322`, observed at `2026-11-09T07:00:00Z`, rights `synthetic-original-work`, scope `synthetic-test-only`, coverage `unknown`.
- Fresh local **Python standard-library-only byte-oracle check: PASS 6/6, exit 0** on 19 November 2026: positive length/hash; invented `example.invalid` and no Enron; equal-length modified body yields a different SHA-256; truncated body changes length; `rendered_html` differs from required raw fidelity; blank-only scope is invalid under the documented nonblank-reference rule. These checks confirm *fixture construction and negative-case detectability*, **not** execution of the Rust converter.
- Reproduction of the positive byte oracle:

```sh
python3 - <<'PY'
import hashlib
raw = b"From: alice@example.invalid\r\nTo: bob@example.invalid\r\nSubject: Fixture\r\n\r\nHello\r\n"
expected = "2ba2c8f860bfc5f2b6d4c13d4cfb0d8eb234239e432880195d9e4579ef87951e"
assert len(raw) == 81 and hashlib.sha256(raw).hexdigest() == expected
assert b"example.invalid" in raw and b"enron" not in raw.lower()
changed = raw.replace(b"Hello", b"Hullo")
assert len(changed) == 81 and hashlib.sha256(changed).hexdigest() != expected
assert len(raw[:-1]) != 81
assert "rendered_html" != "raw_rfc5322"
assert not " ".strip()
print("TRACK04 STATIC BYTE ORACLE: PASS 6/6")
PY
```

### Canonical output and fail-closed receipts

- **Executed hosted Rust and CLI qualification:** [Rust converter checks, run 43870153745](https://github.com/edithatogo/mailweft/actions/runs/43870153745), job `133308880976`, at exact head `8c25d06213f50ae9879d410efb815a45b37dd11d`, completed successfully on **16 November 2026**. Its log shows passing `cargo test --locked --workspace --all-targets --no-fail-fast`, focused `source_integrity`, `batch`, `corpus_reconciliation`, `transfer_provenance`, Rust4PM import, `cargo fmt --all -- --check` and Clippy, with nonzero scoped test counts. It additionally prints:
  - `RUNTIME CONTRACTS OK: 4 actual/fixture outputs, 12 invalid mutations; synthetic input only`
  - `SYNTHETIC END-TO-END OK: unique, absent and ambiguous reply claims; quote and date perturbations`
  - `SELECTION/MAP CONTRACTS OK: actual Rust outputs, accounting/digests and invalid mutations; synthetic sources only`.
- Full Git-tree blob reconciliation between that executed head and reviewed main showed the same `Cargo.lock`, `rust-toolchain.toml`, Rust core/CLI sources and tests, source-envelope fixture, `scripts/synthetic_mime.py`, `scripts/validate_runtime_contracts.py`, `scripts/validate_synthetic_runtime.py`, `scripts/validate_selection_outputs.py`, and `conductor/task-manifest.json`. Differences were workflows, other GitHub configuration and `conductor/tracks/03_ocel/spec.md`; **there was no Rust run at current main head**. The main-head [planning run 44432910852](https://github.com/edithatogo/mailweft/actions/runs/44432910852) and [privacy run 44432910637](https://github.com/edithatogo/mailweft/actions/runs/44432910637) passed on 19 November. This is source-equivalent historical execution, **not exact-main Rust CI certification**.
- The compiled `convert-evidence` path for this same source-envelope specimen was exercised in `scripts/validate_runtime_contracts.py`: the emitted `conversion.log` passed the structural OCEL checker; `receipt.source_copy_id` matched `conversion.source_copy_id`; `receipt.source_sha256` matched the independent raw SHA-256; and the receipt passed its JSON Schema. Source-integrity Rust tests additionally established the OCEL conversion equals the unenveloped converter result, changing rights metadata changes receipt hashes **without changing source events**, and malformed/duplicate envelope fields or an absent observed timestamp are rejected.
- The hosted corpus reconciliation test for the same fictional raw message supplies **three selection slots**: one valid, one hash-mismatched, one unreadable. Strict mode requires exit **2**, supplied/accepted/rejected **3/1/2**, `projection_status=withheld`, `log=null`, rejection diagnostics `SOURCE_HASH_MISMATCH` and `SOURCE_READ_FAILED`, and **no disclosure** of raw IDs, paths, address or body. Explicit `--allow-partial` still returns exit **2**, labels status `partial`, and emits one accepted source-observation event. Other executed tests prohibit duplicate/empty selection IDs, preserve deterministic ordering and avoid counting duplicate source observations as two events.
- The compiled-output checker `scripts/validate_selection_outputs.py` verifies report/schema integrity, `supplied = accepted + rejected`, `unique_observations <= accepted`, SHA-256 manifest/OCEL digests and deliberately rejects corrupted count and digest records. A strict `withheld` record MUST have both `log` and `ocel_sha256` null. These are **selection-level** statements; no delivery, actor authority, whole-mailbox completeness or FOI/clinical semantics are inferred.

### Remaining task and limits

- **T04.01 stays `not_started`; no task checkbox or canonical manifest status is changed.** T00.05 is complete, so T04.01 is eligible. Existing `scripts/synthetic_mime.py` (merged in PR #15) already creates four original fictional EMLs and five controlled perturbations, but the unaccepted canonical T04.01 task still names `tools/generate_fixture.rs` as its only implementation path. The integration owner must reconcile that mismatch before calling T04.01 implemented; a redundant Rust generator or a competing task-manifest edit would be unjustified without a contract decision.
- T04.02 remains blocked on incomplete T00.02 rights assurance and acceptance of T04.01; T04.03–T04.08 remain unaccepted. No real-corpus crosswalk, event-level Enron correspondence, empirical accuracy/benchmark, new CI gate, or release approval is claimed.
- The current assistant execution host has **no `rustc`/`cargo` and cannot access GitHub over its shell network**, so it did **not** rerun the compiled binary at current main. The independent 6/6 check is static source-byte assurance; source-equivalent hosted tests are linked separately. Rust4PM dependency-security issue [#4](https://github.com/edithatogo/mailweft/issues/4) remains a release blocker and is outside this track.
