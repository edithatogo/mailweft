#!/usr/bin/env python3
"""Development-only checks of compiled Rust selection accounting and map schema output."""
import copy
import hashlib
import json
from pathlib import Path
import subprocess
import tempfile
from jsonschema import Draft202012Validator, FormatChecker

ROOT = Path(__file__).resolve().parent.parent
RAW = b'From: alice@example.invalid\r\nTo: bob@example.invalid\r\nSubject: Fixture\r\n\r\nHello\r\n'

def verify(doc):
    manifest = doc['manifest']
    assert manifest['supplied'] == manifest['accepted'] + manifest['rejected'] == len(manifest['entries'])
    assert sum(e['status'] == 'accepted' for e in manifest['entries']) == manifest['accepted']
    assert manifest['unique_observations'] <= manifest['accepted']
    assert len({e['entry_sha256'] for e in manifest['entries']}) == manifest['supplied']
    digest = hashlib.sha256(json.dumps(manifest, ensure_ascii=False, separators=(',', ':')).encode()).hexdigest()
    assert doc['manifest_sha256'] == digest
    if manifest['projection_status'] == 'withheld':
        assert doc['log'] is None and manifest['ocel_sha256'] is None
    else:
        assert doc['log'] is not None
        assert len(doc['log']['events']) == manifest['unique_observations']
        assert manifest['ocel_sha256'] == hashlib.sha256(json.dumps(doc['log'], ensure_ascii=False, separators=(',', ':')).encode()).hexdigest()
        assert (manifest['projection_status'] == 'complete_selection') == (manifest['rejected'] == 0)

def main():
    binary = ROOT / 'target/debug/mailweft'
    schema = json.loads((ROOT / 'contracts/v1/selection-report.schema.json').read_text())
    Draft202012Validator.check_schema(schema)
    validator = Draft202012Validator(schema, format_checker=FormatChecker())
    with tempfile.TemporaryDirectory() as tmp:
        root = Path(tmp)
        (root/'mail.eml').write_bytes(RAW)
        env = json.loads((ROOT/'fixtures/synthetic/contracts/source-envelope.json').read_text())
        (root/'envelope.json').write_text(json.dumps(env))
        request = {'schema_version':'1.0.0','entries':[
            {'entry_id':'first','source_path':'mail.eml','envelope_path':'envelope.json'},
            {'entry_id':'missing','source_path':'missing.eml','envelope_path':'envelope.json'}]}
        (root/'selection.json').write_text(json.dumps(request))
        records = []
        for extra in ([], ['--allow-partial']):
            result = subprocess.run([str(binary),'reconcile',str(root/'selection.json'),*extra],capture_output=True,timeout=30)
            assert result.returncode == 2, 'partial selection must not return all-success'
            doc = json.loads(result.stdout)
            validator.validate(doc)
            verify(doc)
            records.append(doc)
        corrupted = copy.deepcopy(records[1])
        corrupted['manifest']['accepted'] = 99
        try: verify(corrupted)
        except AssertionError: pass
        else: raise AssertionError('bad cross-field accounting was not detected')
        corrupted = copy.deepcopy(records[1])
        corrupted['manifest_sha256'] = '0'*64
        try: verify(corrupted)
        except AssertionError: pass
        else: raise AssertionError('bad manifest digest was not detected')
        raw = b'From: a@example.invalid\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Transfer-Encoding: base64\r\n\r\nSGVsbG8K'
        (root/'encoded.eml').write_bytes(raw)
        result = subprocess.run([str(binary),'inspect',str(root/'encoded.eml')],capture_output=True,timeout=30,check=True)
        evidence = json.loads(result.stdout)
        body_schema = json.loads((ROOT/'contracts/v1/body-evidence.schema.json').read_text())
        Draft202012Validator(body_schema).validate(evidence)
        assert evidence['parts'][0]['transfer_map']['status'] == 'verified_transfer'
        invalid = copy.deepcopy(evidence)
        invalid['parts'][0]['transfer_map']['status'] = 'unavailable'
        assert not Draft202012Validator(body_schema).is_valid(invalid)
    print('SELECTION/MAP CONTRACTS OK: actual Rust outputs, accounting/digests and invalid mutations; synthetic sources only')
    return 0

if __name__ == '__main__': raise SystemExit(main())
