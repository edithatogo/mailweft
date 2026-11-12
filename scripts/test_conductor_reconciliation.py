"""Regression gates for executable Conductor context, not host installation claims."""
import importlib.util
import json
from pathlib import Path
import shutil
import tempfile
import unittest

ROOT = Path(__file__).resolve().parent.parent

class ConductorReconciliationTests(unittest.TestCase):
    def doctor(self):
        path = ROOT / 'scripts/conductor_doctor.py'
        self.assertTrue(path.is_file(), 'Conductor needs a real drift-checking doctor')
        spec = importlib.util.spec_from_file_location('conductor_doctor', path)
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        return module

    def copy_context(self, target):
        for folder in ('conductor', 'docs', 'crates', 'scripts'):
            shutil.copytree(ROOT / folder, target / folder, ignore=shutil.ignore_patterns('__pycache__'))
        for name in ('Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml', 'GEMINI.md', '.gitmodules'):
            if (ROOT / name).exists(): shutil.copy2(ROOT / name, target / name)

    def test_repo_context_is_reconciled_and_not_greenfield(self):
        state = json.loads((ROOT / 'conductor/setup_state.json').read_text())
        self.assertTrue(state.get('rust_workspace_implemented'))
        self.assertEqual(state.get('host_extension_status'), 'not_verified_in_user_host')
        self.assertNotIn('No Rust application', (ROOT / 'conductor/index.md').read_text())

    def test_every_track_is_complete_as_a_conductor_context_not_as_an_implementation(self):
        for path in (ROOT / 'conductor/tracks').iterdir():
            if not path.is_dir(): continue
            for name in ('spec.md', 'plan.md', 'index.md', 'metadata.json', 'evidence.md'):
                self.assertTrue((path / name).is_file(), f'{path.name}/{name}')
            metadata = json.loads((path / 'metadata.json').read_text())
            self.assertEqual(metadata.get('track_id'), path.name)
            self.assertIn(metadata['status'], ('new', 'in_progress', 'completed', 'cancelled'))

    def test_rendered_state_is_current(self):
        self.assertEqual(self.doctor().check(ROOT, verify_git=False), [])

    def test_stale_next_task_and_metadata_are_detected(self):
        doctor = self.doctor()
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            self.copy_context(root)
            state = json.loads((root / 'conductor/setup_state.json').read_text())
            state['rust_workspace_implemented'] = False
            (root / 'conductor/setup_state.json').write_text(json.dumps(state))
            errors = doctor.check(root, verify_git=False)
            self.assertTrue(any('setup_state' in error for error in errors))

    def test_sync_is_idempotent_and_never_marks_unproved_tasks_complete(self):
        doctor = self.doctor()
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            self.copy_context(root)
            manifest = (root / 'conductor/task-manifest.json').read_bytes()
            doctor.sync(root)
            first = {str(p.relative_to(root)): p.read_bytes() for p in (root / 'conductor').rglob('*') if p.is_file()}
            doctor.sync(root)
            second = {str(p.relative_to(root)): p.read_bytes() for p in (root / 'conductor').rglob('*') if p.is_file()}
            self.assertEqual(first, second)
            self.assertEqual(manifest, (root / 'conductor/task-manifest.json').read_bytes())

if __name__ == '__main__': unittest.main()
