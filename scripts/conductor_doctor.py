#!/usr/bin/env python3
"""Reconcile repository Conductor context; never execute tasks or certify a user host."""
from __future__ import annotations
import argparse
from collections import Counter
import json
from pathlib import Path
import re
import subprocess
import tomllib

ROOT = Path(__file__).resolve().parent.parent
PIN = '7a5c560a4fdf5297be58594cc37527eb12790272'
PLUGIN = '.agents/plugins/conductor'
MARK = {'not_started': ' ', 'in_progress': '~', 'blocked': '~', 'completed': 'x'}

def encode(value):
    return json.dumps(value, ensure_ascii=False, indent=2) + '\n'

def load(root):
    return json.loads((root / 'conductor/task-manifest.json').read_text(encoding='utf-8'))

def ready_tasks(tasks):
    lookup = {t['id']: t for t in tasks}
    ready = [t for t in tasks if t['status'] in ('not_started', 'in_progress')
             and all(lookup[d]['status'] == 'completed' for d in t['depends_on'])]
    return sorted(ready, key=lambda t: (t['lane'] != 'core', t['status'] != 'in_progress', t['id']))

def rendered(root):
    manifest = load(root)
    tasks = manifest['tasks']
    by_track = {}
    for task in tasks:
        by_track.setdefault(task['track'], []).append(task)
    counts = Counter(t['status'] for t in tasks)
    ready = ready_tasks(tasks)
    next_id = ready[0]['id'] if ready else None
    stamp = manifest.get('reconciled_at', '2026-11-09T09:17:45Z')
    version = tomllib.loads((root / 'crates/mailweft-core/Cargo.toml').read_text())['package']['version']
    outputs = {}
    registry = ['# Conductor track registry — Mailweft', '',
                'Generated from task-manifest.json. `[x]` = accepted task; `[~]` = partial/in progress; `[ ]` = not started.',
                'Task prerequisites, not whole-track links, determine eligibility. Optional research cannot block the core.', '']
    for track, members in sorted(by_track.items()):
        folder = f'conductor/tracks/{track}'
        old = json.loads((root / folder / 'metadata.json').read_text())
        title = old['title']
        status = 'completed' if all(t['status'] == 'completed' for t in members) else (
            'in_progress' if any(t['status'] != 'not_started' for t in members) else 'new')
        metadata = {
            'track_id': track, 'id': track, 'type': 'chore' if track.startswith('00_') else 'feature',
            'title': title, 'description': title, 'status': status,
            'created_at': old.get('created_at', stamp), 'updated_at': stamp,
            'requirements': sorted({r for t in members for r in t['requirements']}),
            'tasks': [t['id'] for t in members],
            'dependency_policy': 'Only exact task dependencies in task-manifest.json are executable prerequisites.'
        }
        outputs[f'{folder}/metadata.json'] = encode(metadata)
        outputs[f'{folder}/index.md'] = f'# Track {track} Context\n\n- [Specification](./spec.md)\n- [Implementation Plan](./plan.md)\n- [Metadata](./metadata.json)\n- [Evidence](./evidence.md)\n\nStatus: {status}. Consult exact task prerequisites; do not redo completed tasks.\n'
        plan = [f'# {title} — executable task plan', '',
                'Generated from the canonical task manifest; edit that manifest, then run `python3 scripts/conductor_doctor.py --sync`.',
                'A partial task is not unimplemented and not complete. Evidence and remaining acceptance work are explicit.', '']
        evidence = [f'# Track {track} — reconciled evidence', '',
                    'Canonical acceptance declarations: `../../task-manifest.json`. Current CI must pass before merge.',
                    'Earlier checkpoints remain in Git history; initial NOT_STARTED assertions are not current status.', '']
        for task in members:
            code = task['id']
            plan += [f"- [{MARK[task['status']]}] Task: **{code} — {task['title']}**", '',
                     f"  - Lane: `{task['lane']}`; dependencies: {', '.join(task['depends_on']) or 'none'}.",
                     f"  - Requirements: {', '.join(task['requirements'])}.",
                     '  - Allowed paths: ' + ', '.join(f'`{p}`' for p in task['allowed_paths']) + '.',
                     f"  - Action: {task['action']}", f"  - Acceptance: {task['acceptance']}",
                     f"  - Verification: `{task['tests']}`.", f"  - Evidence: `{task['evidence_path']}`."]
            if task.get('task_card'):
                plan.append(f"  - Task card: `{task['task_card']}`.")
            impl = task.get('implementation')
            if impl:
                plan += [f"  - Implemented: {impl['summary']}", f"  - Remaining: {impl['remaining']}"]
                evidence += [f"## {code} — {task['status']}", '', impl['summary'], '',
                             'Artifacts: ' + ', '.join(f'`{p}`' for p in impl['files']),
                             'Verification: ' + '; '.join(f'`{c}`' for c in impl['commands']),
                             'Evidence: ' + '; '.join(impl['evidence']),
                             'Remaining: ' + impl['remaining'], '']
            plan.append('')
        if not any(t.get('implementation') for t in members):
            evidence += ['No implementation acceptance is declared for this track. Specifications and planned tasks are not runtime evidence.', '']
        outputs[f'{folder}/plan.md'] = '\n'.join(plan)
        outputs[f'{folder}/evidence.md'] = '\n'.join(evidence)
        marker = 'x' if status == 'completed' else '~' if status == 'in_progress' else ' '
        registry += ['---', '', f'- [{marker}] **Track: {title}**', f'*Link: [./tracks/{track}/](./tracks/{track}/)*', '']
    outputs['conductor/tracks.md'] = '\n'.join(registry)
    outputs['conductor/setup_state.json'] = encode({
        'product': 'mailweft', 'context_prepared': True, 'plugin_path': PLUGIN, 'plugin_commit': PIN,
        'plugin_version': '0.4.1', 'host_extension_status': 'not_verified_in_user_host',
        'rust_workspace_implemented': (root / 'crates/mailweft-core/src/lib.rs').is_file(),
        'rust_package_version': version, 'task_counts': dict(sorted(counts.items())),
        'next_task': next_id, 'scope_contract': 'conductor/scope-contract.json',
        'state_owner': 'generated_from_task_manifest_not_upstream_setup_state',
    })
    outputs['conductor/index.md'] = f'''# Conductor — Mailweft

**An email-to-OCEL converter written in Rust.** Experimental Rust implementation: {version}.

- [Product Definition](./product.md)
- [Product Guidelines](./product-guidelines.md)
- [Tech Stack](./tech-stack.md)
- [Workflow](./workflow.md)
- [Tracks Registry](./tracks.md)
- [Tracks Directory](./tracks/)
- [Task Manifest](./task-manifest.json)
- [Execution Order](../docs/EXECUTION-ORDER.md)
- [Conductor Setup and Verification](../docs/CONDUCTOR-SETUP.md)

Current accounting: {len(tasks)} tasks in {len(by_track)} tracks; {counts['completed']} completed, {counts['in_progress']} in progress, {counts['blocked']} blocked, {counts['not_started']} not started.
Next eligible task: **{next_id or 'none; resolve blocked prerequisites'}**. Resume partial work; do not recreate the Rust workspace or restart all tasks.

The task manifest owns status, prerequisites and acceptance evidence. `conductor_doctor.py --sync` regenerates plans/indexes/metadata; `--check` rejects drift. Sync does not change task status or certify correctness.

Enron is verification/example data only. `foi-process` is the first consumer; legacy outputs are not an oracle. GraphRAG/model work is optional and cannot block the core.

Pinned Conductor: `{PIN}` at `{PLUGIN}`. Repository setup is audited from artifacts and command manifests; host extension registration on the user's computer is not claimed. Do not rerun generic setup over this context.
'''
    lines = ['# Executable order — generated from exact task prerequisites', '',
             'Only tasks whose prerequisites are completed appear below. Optional lanes do not gate core release.', '',
             'Current ready tasks (resume partial work before starting another task):', '']
    for task in ready:
        lines.append(f"- **{task['id']}** ({task['lane']}, {task['status']}): {task['title']} — `{task['evidence_path']}`")
    lines += ['', 'Read the selected plan/card, write the failing test, implement the bounded change, run acceptance, update the manifest and regenerate context.',
              'Do not mark broad requirements complete from a narrower vertical slice. Dataset rights, source permissions and live integration remain separate gates.', '']
    outputs['docs/EXECUTION-ORDER.md'] = '\n'.join(lines)
    return outputs

def sync(root=ROOT):
    """Render context only. The task manifest is deliberately never modified here."""
    for name, text in rendered(root).items():
        path = root / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding='utf-8')

def check(root=ROOT, verify_git=True):
    errors = []
    try:
        tasks = load(root)['tasks']
        expected = rendered(root)
    except (OSError, ValueError, KeyError, TypeError) as exc:
        return [f'Conductor context cannot be resolved: {type(exc).__name__}']
    for name, text in expected.items():
        path = root / name
        if not path.is_file() or path.read_text(encoding='utf-8') != text:
            errors.append(f'Conductor drift: {name}; run --sync after reviewing manifest changes')
    for task in tasks:
        if task['status'] != 'not_started':
            impl = task.get('implementation', {})
            if not all(impl.get(k) for k in ('summary', 'remaining', 'files', 'commands', 'evidence')):
                errors.append(f"Missing implementation evidence: {task['id']}")
            for name in impl.get('files', []):
                if Path(name).is_absolute() or '..' in Path(name).parts or not (root / name).is_file():
                    errors.append(f"Missing/unsafe evidence artifact: {task['id']}/{name}")
    for name in ('GEMINI.md', 'rust-toolchain.toml', 'docs/CONDUCTOR-SETUP.md',
                 'conductor/product.md', 'conductor/product-guidelines.md', 'conductor/tech-stack.md', 'conductor/workflow.md'):
        if not (root / name).is_file(): errors.append(f'Missing context artifact: {name}')
    if not list((root / 'conductor/code_styleguides').glob('*.md')):
        errors.append('Missing Conductor code styleguides')
    if verify_git:
        def git(*args):
            return subprocess.check_output(['git', '-C', str(root), *args], text=True, stderr=subprocess.DEVNULL).strip()
        try:
            entry = git('ls-tree', 'HEAD', PLUGIN).split()
            if entry[:3] != ['160000', 'commit', PIN]: errors.append('Conductor gitlink/pin mismatch')
            checked = git('-C', PLUGIN, 'rev-parse', 'HEAD')
            if checked != PIN: errors.append('Conductor checkout missing or wrong revision')
            if git('-C', PLUGIN, 'status', '--porcelain'): errors.append('Conductor upstream checkout is modified')
            config = git('config', '-f', '.gitmodules', '--get-regexp', r'^submodule\..*\.(path|url)$')
            if PLUGIN not in config or 'https://github.com/gemini-cli-extensions/conductor.git' not in config:
                errors.append('Conductor submodule origin/path mismatch')
            plugin = root / PLUGIN
            extension = json.loads((plugin / 'gemini-extension.json').read_text())
            if extension.get('name') != 'conductor' or extension.get('version') != '0.4.1':
                errors.append('Pinned extension manifest mismatch')
            for command in ('setup', 'newTrack', 'implement', 'status', 'review', 'revert'):
                obj = tomllib.loads((plugin / f'commands/conductor/{command}.toml').read_text())
                if not obj.get('prompt') or not obj.get('description'): errors.append(f'Invalid upstream command: {command}')
        except (OSError, ValueError, subprocess.CalledProcessError):
            errors.append('Conductor checkout/integrity unavailable; initialize the pinned submodule, do not claim host installation')
    return errors

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--sync', action='store_true')
    parser.add_argument('--check', action='store_true')
    parser.add_argument('--status', action='store_true')
    args = parser.parse_args()
    if args.sync: sync()
    errors = check()
    if errors:
        print('CONDUCTOR FAILED:\n- ' + '\n- '.join(errors))
        return 1
    tasks = load(ROOT)['tasks']
    print('CONDUCTOR OK: pinned source, command TOMLs, project/track artifacts and canonical task state agree.')
    print('Task status:', dict(Counter(t['status'] for t in tasks)))
    print('Next eligible:', ', '.join(t['id'] for t in ready_tasks(tasks)[:8]) or 'none')
    print('User-host extension registration is not verified by this repository check.')
    return 0

if __name__ == '__main__': raise SystemExit(main())
