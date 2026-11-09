"""Regression checks against the pinned Conductor implementation protocol."""
from pathlib import Path
import json
import re
import shutil
import tempfile
import unittest

import conductor_doctor as doctor

ROOT = Path(__file__).resolve().parent.parent
HEADING = re.compile(r'^## \[([ ~x])\] Track: (.+)$', re.MULTILINE)
LINK = re.compile(r'\[\./tracks/([^/]+)/\]\(\./tracks/\1/\)')


class ConductorProtocolTests(unittest.TestCase):
    def test_pinned_implement_command_can_resolve_every_track(self):
        manifest = doctor.load(ROOT)
        expected = {task['track'] for task in manifest['tasks']}
        registry = doctor.rendered(ROOT)['conductor/tracks.md']
        actual = set()
        for section in registry.split('---'):
            if '*Link:' not in section:
                continue
            headings = HEADING.findall(section)
            self.assertEqual(len(headings), 1, 'Pinned implement.toml requires a ## track heading')
            links = LINK.findall(section)
            self.assertEqual(len(links), 1)
            self.assertNotIn(links[0], actual)
            actual.add(links[0])
        self.assertEqual(actual, expected)

    def test_current_completed_tasks_have_completed_prerequisites(self):
        tasks = {t['id']: t for t in doctor.load(ROOT)['tasks']}
        for task in tasks.values():
            if task['status'] == 'completed':
                for dep in task['depends_on']:
                    self.assertEqual(tasks[dep]['status'], 'completed', f"{task['id']} precedes unaccepted {dep}")

    def test_doctor_rejects_completed_task_with_unaccepted_dependency(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            # Only project context is copied; no checkout is mutated.
            for name in ('conductor', 'crates', 'docs', 'scripts', 'contracts', '.github'):
                shutil.copytree(ROOT / name, root / name)
            for name in ('GEMINI.md', 'rust-toolchain.toml', 'AGENTS.md', '.gitignore', 'Cargo.toml', 'Cargo.lock'):
                shutil.copy2(ROOT / name, root / name)
            self.assertEqual(doctor.check(root, verify_git=False), [])
            manifest = doctor.load(root)
            task = next(t for t in manifest['tasks'] if t['id'] == 'T00.07')
            task['depends_on'].append('T00.02')
            (root / 'conductor/task-manifest.json').write_text(json.dumps(manifest))
            doctor.sync(root)
            errors = doctor.check(root, verify_git=False)
            self.assertTrue(any('T00.07' in error and 'T00.02' in error for error in errors), errors)


if __name__ == '__main__':
    unittest.main()
