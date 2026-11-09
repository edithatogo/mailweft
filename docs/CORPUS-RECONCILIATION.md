# Explicit source-selection reconciliation — Mailweft 0.7

The converter accepts an explicit local selection, not a discovered mailbox or a claim that all correspondence exists. Every selection slot has one accepted/rejected outcome. No network capture, recursive traversal, source generation or FOI-specific interpretation occurs.

## Usage

```sh
mailweft reconcile selection.json > reconciliation.json
mailweft reconcile selection.json --allow-partial > reconciliation.json
```

Private request example (paths relative to the selection file unless absolute):

```json
{
  "schema_version": "1.0.0",
  "entries": [
    {"entry_id": "capture-item-1", "source_path": "mail/one.eml", "envelope_path": "receipts/one.json"},
    {"entry_id": "capture-item-2", "source_path": "mail/two.eml", "envelope_path": "receipts/two.json"}
  ]
}
```

Each envelope follows the existing source-envelope contract: actual byte hash/length must match; rights, scope, timestamp, source format and coverage remain caller assertions. Missing files, invalid envelopes and failed integrity checks are rejected outcomes, never fabricated events. Duplicate selection keys are invalid; distinct slots may legitimately refer to the same observation.

## Output and exit codes

The report contains `manifest`, `manifest_sha256` and `log`. The manifest records supplied/accepted/rejected counts, unique observation count, projection status, per-slot hashed IDs and accepted source receipts or rejection diagnostics. `log`, when present, is the portable OCEL root. Neither request paths, raw slot IDs nor body text are emitted. Hashes remain pseudonymous and require access controls.

- Exit **0**: every supplied slot accepted; `projection_status=complete_selection`. This means the supplied selection passed, not that the entire mailbox/request history is complete.
- Exit **2**: at least one slot rejected. By default `projection_status=withheld` and `log=null`. With `--allow-partial`, accepted-only OCEL is emitted and labelled `partial`; exit status remains 2. When nothing is accepted, the log remains withheld even with opt-in.
- Exit **1**: invalid request, duplicate keys, aggregate projection conflict or fatal output failure. No report is claimed complete.

Consumers must parse the report and handle exit 2 deliberately. Do not suppress the nonzero exit and assume all source material was represented. The selection assurance is always `supplied_selection_only`.

## Determinism and bounds

Slots are ordered by their private entry IDs before I/O; output outcomes are ordered by opaque entry hashes. Reordering a request does not change accepted/rejected accounting or output, including deterministic resource exclusions. Repeated identical observations are idempotent in OCEL but remain distinct accepted selection slots. `accepted` and `unique_observations` therefore need not be equal.

The manifest fingerprint covers compact serde serialization of the typed manifest, including outcomes and optional OCEL digest. This is a versioned serialization contract, not generic JSON canonicalization or a digital signature. The optional OCEL hash covers the compact serialized log.

Limits: 1 MiB selection JSON, 1,024 entries, 64 KiB per envelope, 16 MiB per email and 256 MiB successfully read email bytes. Explicit regular files only; no sandbox, trusted-directory confinement or race-free path resolution is claimed. Processing and merged OCEL are in memory, not streaming. Hash-mismatched bytes still consume the input budget.

The core API `corpus::reconcile_selection` accepts supplied raw/envelope results, including caller-recorded source read failures. The CLI owns local file reading; this does not move capture/archive responsibility into Mailweft.

## Validation

`corpus_reconciliation.rs` checks mixed success/failure, strict versus partial output, missing files, bad hashes, reordering, duplicate observations and invalid selection keys. `validate_selection_outputs.py` checks actual compiled CLI output against its schema, cross-field counts and both digests, and rejects deliberately corrupted count/digest records. Existing Rust4PM tests protect the unchanged OCEL merge path.

Enron remains optional verification/example data. The first consumer is foi-process, which must independently validate its FOI mappings; legacy outputs are not a correctness oracle.
