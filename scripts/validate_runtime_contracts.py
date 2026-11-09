#!/usr/bin/env python3
"""Development-only validation of actual Rust CLI output on fictional messages."""
from copy import deepcopy
from pathlib import Path
import hashlib
import json
import subprocess
import sys
import tempfile
from jsonschema import Draft202012Validator, FormatChecker

ROOT = Path(__file__).resolve().parent.parent
RAW = b"From: alice@example.invalid\r\nTo: bob@example.invalid\r\nSubject: Fixture\r\n\r\nHello\r\n"


def main():
    binary = Path(sys.argv[1] if len(sys.argv) > 1 else ROOT / "target/debug/mailweft").resolve()
    if not binary.is_file():
        print("BLOCKED: build the Rust CLI before runtime contract validation", file=sys.stderr)
        return 2
    checker = FormatChecker()
    if "date-time" not in checker.checkers:
        print("BLOCKED: missing date-time validator", file=sys.stderr)
        return 2
    validators = {}
    for name in ("source-envelope", "source-receipt", "body-evidence"):
        schema = json.loads((ROOT / f"contracts/v1/{name}.schema.json").read_text())
        Draft202012Validator.check_schema(schema)
        validators[name] = Draft202012Validator(schema, format_checker=checker)
    envelope = json.loads((ROOT / "fixtures/synthetic/contracts/source-envelope.json").read_text())
    assert envelope["source_sha256"] == hashlib.sha256(RAW).hexdigest()
    assert envelope["byte_length"] == len(RAW)
    validators["source-envelope"].validate(envelope)
    with tempfile.TemporaryDirectory() as directory:
        mail = Path(directory) / "synthetic.eml"
        meta = Path(directory) / "envelope.json"
        mail.write_bytes(RAW)
        meta.write_text(json.dumps(envelope))
        def invoke(*args):
            result = subprocess.run([str(binary), *map(str, args)], capture_output=True, timeout=30)
            if result.returncode != 0:
                raise AssertionError("Rust CLI rejected a valid synthetic fixture")
            return json.loads(result.stdout)
        bundle = invoke("convert-evidence", mail, meta)
        receipt = bundle["receipt"]
        validators["source-receipt"].validate(receipt)
        assert receipt["source_copy_id"] == bundle["conversion"]["source_copy_id"]
        assert receipt["source_sha256"] == hashlib.sha256(RAW).hexdigest()
        evidence = invoke("inspect", mail)
        validators["body-evidence"].validate(evidence)
        encoded = b"From: source@example.invalid\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Transfer-Encoding: base64\r\n\r\nSGVsbG8K"
        mail.write_bytes(encoded)
        derived = invoke("inspect", mail)
        validators["body-evidence"].validate(derived)
        assert derived["parts"][0]["zones"][0]["offset_quality"] == "derived"

    invalid = []
    def mutate(name, original, key, value):
        item = deepcopy(original)
        item[key] = value
        invalid.append((name, item))
    mutate("source-envelope", envelope, "observed_at", "not-a-time")
    mutate("source-envelope", envelope, "schema_version", "99.0.0")
    mutate("source-envelope", envelope, "source_sha256", "invalid")
    mutate("source-envelope", envelope, "rights_ref", " ")
    mutate("source-envelope", envelope, "scope_ref", "")
    mutate("source-envelope", envelope, "byte_length", 0)
    mutate("source-envelope", envelope, "byte_length", 16777217)
    mutate("source-envelope", envelope, "invented_delivery", True)
    mutate("source-receipt", receipt, "assurance", "verified_authorship")
    mutate("source-receipt", receipt, "fidelity", "rendered_html")
    mutate("source-receipt", receipt, "scope_ref", "private-scope")
    wrong = deepcopy(derived)
    wrong["parts"][0]["zones"][0]["raw_start"] = 0
    invalid.append(("body-evidence", wrong))
    for name, item in invalid:
        if validators[name].is_valid(item):
            raise AssertionError(f"Invalid {name} mutation accepted")
    print(f"RUNTIME CONTRACTS OK: 4 actual/fixture outputs, {len(invalid)} invalid mutations; synthetic input only")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
