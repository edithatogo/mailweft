# Track 00_repository_metadata — reconciled evidence

Canonical acceptance declarations: `../../task-manifest.json`. Current CI must pass before merge.
Earlier checkpoints remain in Git history; initial NOT_STARTED assertions are not current status.

## T00a.01 — blocked

The scoped template and maintenance records are prepared. Documentation, canonical task context, eight focused Conductor regression tests and all 45 Python planning tests passed locally in the available Python environment.

Artifacts: `.github/pull_request_template.md`, `conductor/tracks/00_repository_metadata/spec.md`, `conductor/tracks/00_repository_metadata/validation.json`
Verification: `python3 scripts/conductor_doctor.py --sync`; `python3 scripts/conductor_doctor.py --check`; `python3 scripts/validate_scaffold.py`; `python3 -m unittest discover -s scripts -p 'test_conductor_*.py'`; `python3 -m unittest discover -s scripts -p 'test_*.py'`
Evidence: conductor/tracks/00_repository_metadata/validation.json
Remaining: Task acceptance is blocked: Rust 1.99.1 cargo/rustc/rustup are unavailable locally and the first-party bootstrap download timed out. Locked Rust tests, Rust4PM import, formatting, all-feature Clippy, building the CLI, actual-output runtime/selection schema checks and exact-head hosted CI remain required and unverified. All 45 Python planning tests passed in the available Python 3.12.14 environment; the exact requirements-planning.txt environment was not installed or qualified.
