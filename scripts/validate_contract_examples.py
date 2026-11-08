#!/usr/bin/env python3
"""Development-only schema checks on fictional records; never semantic proof.

Requires jsonschema with Draft 2020-12 support in the *development* environment.
No network, actual mail, Rust engine or model is invoked.
"""
from __future__ import annotations

from copy import deepcopy
import json
from pathlib import Path
import sys

try:
    from jsonschema import Draft202012Validator, FormatChecker
except ImportError:
    print("BLOCKED: development dependency jsonschema is not installed", file=sys.stderr)
    raise SystemExit(2)

ROOT = Path(__file__).resolve().parent.parent


def main() -> int:
    checker = FormatChecker()
    if "date-time" not in checker.checkers:
        print("BLOCKED: date-time validation requires pinned rfc3339-validator", file=sys.stderr)
        return 2
    schemas = {}
    for path in sorted((ROOT / "contracts/v1").glob("*.schema.json")):
        schema = json.loads(path.read_text(encoding="utf-8"))
        Draft202012Validator.check_schema(schema)
        schemas[path.name.removesuffix(".schema.json")] = Draft202012Validator(
            schema, format_checker=checker
        )
    examples = {}
    for path in sorted((ROOT / "fixtures/synthetic/contracts").glob("*.json")):
        instance = json.loads(path.read_text(encoding="utf-8"))
        schemas[path.stem].validate(instance)
        examples[path.stem] = instance

    cases = []
    def altered(name, title, field, value=None, *, delete=False):
        instance = deepcopy(examples[name])
        if delete:
            instance.pop(field, None)
        else:
            instance[field] = value
        cases.append((name, title, instance))

    altered("communication-act", "raw narrative field forbidden", "raw_body", "fictional")
    altered("communication-act", "missing evidence", "evidence_refs", delete=True)
    altered("communication-act", "empty evidence", "evidence_refs", [])
    altered("communication-act", "unknown act", "labels", ["verified_culpability"])
    altered("communication-act", "unknown origin", "origin", "somewhere")
    altered("inference-receipt", "score above one", "score", 1.1)
    altered("inference-receipt", "score below zero", "score", -0.1)
    altered("inference-receipt", "bad configuration digest", "config_sha256", "xyz")
    altered("inference-receipt", "empty scope", "allowed_scope_ids", [])
    altered("inference-receipt", "calibration needs record", "calibration_status", "heldout_calibrated")
    altered("interaction-link", "missing response", "response_act_id", delete=True)
    altered("interaction-link", "unknown relationship", "relationship", "proves_truth")
    altered("interaction-link", "duplicate evidence", "evidence_refs", ["x", "x"])
    altered("interpretation-delta", "retraction cannot contain new record", "record_ref", "record:x")
    altered("interpretation-delta", "retraction must identify prior record", "prior_record_sha256", None)
    altered("interpretation-delta", "revision must be positive", "revision", 0)
    altered("interpretation-delta", "bad timestamp", "observed_at", "not-a-time")
    altered("interpretation-delta", "upsert needs a record", "operation", "upsert")
    failed = []
    for name, title, instance in cases:
        if schemas[name].is_valid(instance):
            failed.append(title)
    if failed:
        print("CONTRACT EXAMPLES FAILED: invalid instances were accepted", *failed, sep="\n- ")
        return 1
    print(f"CONTRACT EXAMPLES OK: {len(schemas)} metaschemas, {len(examples)} positive and {len(cases)} negative cases")
    print("Examples are independent fictional records, not a cross-record corpus. Referential integrity, policy checks and Rust round trips remain future tasks.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
