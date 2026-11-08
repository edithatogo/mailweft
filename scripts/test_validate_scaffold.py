"""Regression tests for the planning validator; no application/runtime tests."""
from pathlib import Path
import json
import shutil
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parent.parent

class ScaffoldValidationTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.root = Path(self.tmp.name) / 'project'
        shutil.copytree(ROOT, self.root, ignore=shutil.ignore_patterns('.git', '.agents', '__pycache__'))
        self.path = self.root / 'conductor/task-manifest.json'
        self.manifest = json.loads(self.path.read_text())
    def tearDown(self):
        self.tmp.cleanup()
    def run_check(self):
        self.path.write_text(json.dumps(self.manifest))
        return subprocess.run([sys.executable, str(self.root/'scripts/validate_scaffold.py')], capture_output=True, text=True)
    def test_valid_scaffold_passes(self):
        result = self.run_check()
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
    def test_dangling_dependency_is_rejected(self):
        self.manifest['tasks'][0]['depends_on'] = ['T99.99']
        self.assertNotEqual(self.run_check().returncode, 0)
    def test_cycle_is_rejected(self):
        a,b = self.manifest['tasks'][:2]
        a['depends_on'] = [b['id']]; b['depends_on'] = [a['id']]
        self.assertNotEqual(self.run_check().returncode, 0)
    def test_duplicate_task_id_is_rejected(self):
        self.manifest['tasks'].append(dict(self.manifest['tasks'][0]))
        self.assertNotEqual(self.run_check().returncode, 0)
    def test_unknown_requirement_is_rejected(self):
        self.manifest['tasks'][0]['requirements'] = ['R999']
        self.assertNotEqual(self.run_check().returncode, 0)
    def test_core_cannot_depend_on_research(self):
        a,b = self.manifest['tasks'][:2]
        a['lane'] = 'core'; a['depends_on'] = [b['id']]
        b['lane'] = 'research'; b['depends_on'] = []
        self.assertNotEqual(self.run_check().returncode, 0)
    def test_task_registry_must_match_track(self):
        self.manifest['tasks'].pop()
        self.assertNotEqual(self.run_check().returncode, 0)
    def test_declared_task_card_must_exist(self):
        self.manifest['tasks'][0]['task_card'] = 'conductor/tasks/not-present.md'
        self.assertNotEqual(self.run_check().returncode, 0)

if __name__ == '__main__':
    unittest.main()
