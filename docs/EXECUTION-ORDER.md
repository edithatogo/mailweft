# Execution order and independent release lanes

## First deliverable

Qualify minimal dependencies (T00.01), exclusions (T00.03), workspace (T00.04), pinned toolchain/lock (T00.05); then raw parser/headers/identity/OCEL tasks. T00.02 governs **real corpus rights**, not fictional-fixture development. Foundation workflow/submodule checks can proceed independently of corpus access.

Core package acceptance T08.07/T08.08 depends on full synthetic source/OCEL fidelity, default-feature fuzz/resource tests, supply-chain qualification and selected platform CI. It does not wait for MBOX, external engine comparators, optional XML/SQLite profiles, body NLP, attachment content extraction, graphs or real Enron benchmarks. JSON OCEL 2.0 is the baseline; every additional format remains separately qualified as stated in R07.

## Parallel paths after safe decoding

- Content-access policy T05.01 can begin after safe MIME metadata T01.06.
- Safe body decoder T05.02 enables structural NLP. Implement source offsets T05b.03 **before** zoning T05b.02, then views and participant mentions.
- Attachment converters/nested-message/size-security work stays independently gated; no untrusted converter is executed before its own guards. A safe body path does not authorize attachment content processing.
- Annotated structure and rule acts come before local-model experiments. New interaction track 09 and receipts/revision track 10 consume typed evidence rather than replacing it.
- Research track 11 and integration track 12 are optional, default-disabled paths. A blocked experiment is not a failed or blocked core release.

## Basic-model dispatch

Use the exact manifest entry, task card when present, owning spec and shared contracts. The 30 new tasks have explicit function/artifact interfaces, two named oracles, exact test targets and restricted files. Legacy tasks retain their prior acceptance criteria; before implementation, turn any generic test instruction into exact synthetic assertions using the accepted predecessor types, without changing the design. No task is claimed executable merely because a referenced Cargo command is written; the compiler/toolchain and actual predecessor implementation must exist.

```text
Implement ONLY <TASK_ID>. Check dependencies in conductor/task-manifest.json.
Read the task card and required contracts. Write the named failing tests first.
Edit only allowed files. Run the exact verification command and check that tests
actually ran. Record exit codes, limitations and evidence. Commit only accepted
work and stop. Missing dependencies/tools => BLOCKED, never a fabricated pass.
```

## Plan validation

Run `python3 scripts/validate_scaffold.py` and `python3 scripts/test_validate_scaffold.py`. These validate planning consistency, not application implementation. `python3 scripts/validate_contract_examples.py` additionally uses the development-only jsonschema package for Draft 2020-12/schema-example checks.

## FOI-only scope and robustness refinement
The sole active owned consumer is `foi-process`, which will depend on this generic library. Other owned-repository integrations are removed from active tasks. See [the superseding source-fidelity, baseline and error-impact protocol](FOI-SCOPE-AND-ROBUSTNESS.md). Repository name `mailweft` is selected. Planned package paths use Mailweft; historical schema IDs remain stable aliases (see docs/NAME-MIGRATION.md). No runtime benchmark or downstream migration is claimed.

## Mailweft refinement: independent baseline, ontology feedback and GraphRAG
Core release is T00/T01/T02/T03 plus synthetic/security/packaging closure. Enron does not shape the product. T12.03 now depends only on the Stage 1 consumer boundary, not model inference. T12.04 uses independently annotated source/synthetic gold; the legacy consumer is diagnostic-only. T12.07/T12.08 define reviewed FOI-O contributions without blocking initial converter use. Track 13 is a separate optional graph qualification path; T07.04 consolidates its experiment evidence rather than creating a duplicate implementation. Graph contract packets are not a deployed graph or a Rust runtime implementation.
