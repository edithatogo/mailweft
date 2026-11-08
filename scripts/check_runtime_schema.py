#!/usr/bin/env python3
"""Development test of real CLI output against independently stored contracts."""
import hashlib
import json
from pathlib import Path
import subprocess
import tempfile
from jsonschema import Draft202012Validator, FormatChecker

ROOT = Path(__file__).resolve().parent.parent
checker = FormatChecker()
assert 'date-time' in checker.checkers, 'date-time checker unavailable'
with tempfile.TemporaryDirectory() as directory:
    base = Path(directory).resolve()
    key, source = base / 'key.bin', base / 'fictional.eml'
    key.write_bytes(bytes([7]) * 32)
    raw = b'From: Alice <alice@example.test>\r\nTo: bob@example.test\r\nDate: Mon, 02 Nov 2026 09:30:00 +1000\r\n\r\nFictional only.\r\n'
    source.write_bytes(raw)
    result = subprocess.run(['cargo', 'run', '--quiet', '--offline', '--locked', '-p', 'mailweft-cli', '--', 'convert', '--synthetic', '--scope', 'schema-fixture', '--observed-at', '2026-11-08T00:00:00Z', '--view', 'both', '--key-file', str(key), str(source)], cwd=ROOT, check=True, capture_output=True)
    bundle = json.loads(result.stdout)
    for schema_name, records in [('observed-message', bundle['messages']), ('acquisition-manifest', [bundle['receipt']])]:
        schema = json.loads((ROOT / 'contracts/v1' / f'{schema_name}.schema.json').read_text())
        validator = Draft202012Validator(schema, format_checker=checker)
        for record in records:
            validator.validate(record)
    assert bundle['messages'][0]['source_sha256'] == hashlib.sha256(raw).hexdigest()
    assert bundle['receipt']['input_count'] == 1
    assert len(bundle['ocel']['events']) == 2
    assert all(value not in result.stdout for value in [b'alice@example.test', b'Fictional only.', str(base).encode()])
    ids = {obj['id'] for obj in bundle['ocel']['objects']}
    for record in [*bundle['ocel']['events'], *bundle['ocel']['objects']]:
        assert all(rel['objectId'] in ids for rel in record['relationships'])
print('Actual offline CLI output: both evidence schemas, source hash, chronology count, privacy and references PASS')
