"""Planning invariants only; these do not exercise a Rust application."""
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parent.parent
VALID = {
    "product": "mailweft",
    "core_outcome": "rust_email_to_ocel_converter",
    "core_input": "raw_rfc5322_mime",
    "enron_role": "verification_and_example_only",
    "legacy_foi_is_oracle": False,
    "first_consumer": "foi-process",
    "dependency_direction": "foi-process -> mailweft",
    "acquisition_owner": "fyi-cli",
    "archive_boundary": "fyi-archive manifests or explicitly verified successor",
    "ontology_owner": "foi-o",
    "ontology_feedback": "evidence_linked_candidates_not_automatic_promotion",
    "graphrag_default_enabled": False,
    "core_requires_graph_or_model": False,
}

class ScopeContractTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.root = Path(self.tmp.name) / 'project'
        shutil.copytree(ROOT, self.root, ignore=shutil.ignore_patterns('.git', '.agents', '__pycache__'))
        self.path = self.root / 'conductor/scope-contract.json'
        self.scope = dict(VALID)
    def tearDown(self):
        self.tmp.cleanup()
    def run_check(self):
        self.path.write_text(json.dumps(self.scope), encoding='utf-8')
        return subprocess.run([sys.executable, str(self.root/'scripts/validate_scaffold.py')], capture_output=True, text=True)
    def test_approved_scope_passes(self):
        result = self.run_check()
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
    def test_legacy_foi_cannot_be_validation_oracle(self):
        self.scope['legacy_foi_is_oracle'] = True
        self.assertNotEqual(self.run_check().returncode, 0)
    def test_enron_cannot_be_product_input_requirement(self):
        self.scope['enron_role'] = 'mandatory_product_input'
        self.assertNotEqual(self.run_check().returncode, 0)
    def test_graph_cannot_be_default_requirement(self):
        self.scope['core_requires_graph_or_model'] = True
        self.assertNotEqual(self.run_check().returncode, 0)
    def test_converter_cannot_depend_on_consumer(self):
        self.scope['dependency_direction'] = 'mailweft -> foi-process'
        self.assertNotEqual(self.run_check().returncode, 0)
    def test_ontology_promotion_cannot_be_automatic(self):
        self.scope['ontology_feedback'] = 'automatically_promote'
        self.assertNotEqual(self.run_check().returncode, 0)
    def test_missing_scope_field_is_rejected(self):
        del self.scope['acquisition_owner']
        self.assertNotEqual(self.run_check().returncode, 0)

if __name__ == '__main__':
    unittest.main()
