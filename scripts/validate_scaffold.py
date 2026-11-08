#!/usr/bin/env python3
"""Validate planning integrity, not application behavior or scientific claims.

Standard-library only. Full Draft 2020-12 validation is a separate, optional
*development* check in validate_contract_examples.py; neither is a Rust runtime.
"""
from __future__ import annotations

import json
from pathlib import Path
import re
import sys
from typing import Any

ROOT = Path(__file__).resolve().parent.parent
LANES = {"core", "corpus", "enrichment", "research", "integration"}
STATUSES = {"not_started", "in_progress", "blocked", "completed"}
TASK_ID = re.compile(r"T\d{2}[a-z]?\.\d{2}\Z")
REQUIRED = (
    "README.md", "AGENTS.md", ".gitmodules", "docs/CONTRACT.md",
    "docs/REQUIREMENTS.md", "docs/DESIGN.md", "docs/DATASETS.md",
    "docs/VALIDATION.md", "docs/TRACEABILITY.md", "conductor/index.md",
    "conductor/tracks.md", "conductor/task-manifest.json",
)


def validate(root: Path) -> tuple[list[str], dict[str, int]]:
    """Return explicit errors and counts; never silently accept missing data."""
    errors: list[str] = []

    def load(path: Path) -> Any:
        try:
            return json.loads(path.read_text(encoding="utf-8"))
        except (OSError, ValueError) as exc:
            errors.append(f"cannot read {path.relative_to(root)}: {exc}")
            return None

    def path_exists(value: object, context: str) -> None:
        if not isinstance(value, str) or not value.strip():
            errors.append(f"{context}: expected a nonempty relative file path")
            return
        path = root / value
        if Path(value).is_absolute() or ".." in Path(value).parts:
            errors.append(f"{context}: path must remain within project")
        elif not path.is_file():
            errors.append(f"{context}: missing {value}")

    for name in REQUIRED:
        path_exists(name, "required document")
    requirements_text = (root / "docs/REQUIREMENTS.md").read_text(encoding="utf-8") if (root / "docs/REQUIREMENTS.md").is_file() else ""
    requirement_ids = re.findall(r"^\|\s*(R\d+)\s*\|", requirements_text, re.MULTILINE)
    if not requirement_ids or len(set(requirement_ids)) != len(requirement_ids):
        errors.append("requirements must have unique table IDs")
    known_requirements = set(requirement_ids)

    schemas = sorted((root / "contracts/v1").glob("*.schema.json"))
    schema_ids: list[str] = []
    for path in schemas:
        obj = load(path)
        if not isinstance(obj, dict):
            errors.append(f"schema must be an object: {path.name}")
            continue
        if obj.get("$schema") != "https://json-schema.org/draft/2020-12/schema":
            errors.append(f"schema dialect: {path.name}")
        if obj.get("additionalProperties") is not False:
            errors.append(f"unknown fields are not forbidden: {path.name}")
        if isinstance(obj.get("$id"), str):
            schema_ids.append(obj["$id"])
    if len(schema_ids) != len(set(schema_ids)):
        errors.append("schema $id values must be unique")

    manifest = load(root / "conductor/task-manifest.json")
    raw_tasks = manifest.get("tasks", []) if isinstance(manifest, dict) else []
    if not isinstance(raw_tasks, list) or not raw_tasks:
        errors.append("manifest tasks must be a nonempty list")
        raw_tasks = []
    tasks: dict[str, dict[str, Any]] = {}
    covered_requirements: set[str] = set()
    for task in raw_tasks:
        if not isinstance(task, dict):
            errors.append("manifest task must be an object")
            continue
        code = task.get("id")
        if not isinstance(code, str) or not TASK_ID.fullmatch(code):
            errors.append(f"invalid task id: {code!r}")
            continue
        if code in tasks:
            errors.append(f"duplicate task ID: {code}")
        tasks[code] = task
        for key in ("track", "title", "action", "acceptance", "tests", "evidence_path"):
            if not isinstance(task.get(key), str) or not task[key].strip():
                errors.append(f"{code}: missing or invalid {key}")
        if task.get("lane") not in LANES:
            errors.append(f"{code}: unknown lane")
        if task.get("status") not in STATUSES:
            errors.append(f"{code}: unknown status")
        deps = task.get("depends_on")
        if not isinstance(deps, list) or any(not isinstance(d, str) for d in deps):
            errors.append(f"{code}: dependencies must be task IDs")
            task["depends_on"] = []
        elif len(deps) != len(set(deps)):
            errors.append(f"{code}: duplicate dependency")
        refs = task.get("requirements")
        if not isinstance(refs, list) or not refs or any(not isinstance(r, str) for r in refs):
            errors.append(f"{code}: requirements must be nonempty IDs")
        else:
            unknown = set(refs) - known_requirements
            if unknown:
                errors.append(f"{code}: unknown requirements {sorted(unknown)}")
            covered_requirements.update(refs)
        if "task_card" in task:
            path_exists(task["task_card"], f"{code} card")
        path_exists(task.get("evidence_path"), f"{code} evidence")

    missing_requirements = known_requirements - covered_requirements
    if missing_requirements:
        errors.append(f"unmapped requirements: {sorted(missing_requirements)}")
    for code, task in tasks.items():
        for dep in task["depends_on"]:
            if dep not in tasks:
                errors.append(f"{code}: dangling dependency {dep}")
            elif task.get("lane") == "core" and tasks[dep].get("lane") != "core":
                errors.append(f"{code}: core cannot depend on {dep} ({tasks[dep].get('lane')})")

    visiting: set[str] = set()
    visited: set[str] = set()
    def visit(code: str, stack: tuple[str, ...]) -> None:
        if code in visiting:
            errors.append("dependency cycle: " + " -> ".join((*stack, code)))
            return
        if code in visited:
            return
        visiting.add(code)
        for dep in tasks[code]["depends_on"]:
            if dep in tasks:
                visit(dep, (*stack, code))
        visiting.remove(code)
        visited.add(code)
    for code in tasks:
        visit(code, ())

    registry: dict[str, str] = {}
    track_paths = sorted((root / "conductor/tracks").glob("*/metadata.json"))
    for path in track_paths:
        meta = load(path)
        if not isinstance(meta, dict):
            errors.append(f"bad track metadata: {path.parent.name}")
            continue
        track_id = path.parent.name
        if meta.get("id") != track_id:
            errors.append(f"track ID mismatch: {track_id}")
        members = meta.get("tasks")
        if not isinstance(members, list) or not members:
            errors.append(f"track has no task list: {track_id}")
            continue
        for name in ("plan.md", "spec.md", "evidence.md"):
            path_exists(str(path.parent.relative_to(root) / name), f"track {track_id}")
        plan_path = path.parent / "plan.md"
        plan = plan_path.read_text(encoding="utf-8") if plan_path.is_file() else ""
        for code in members:
            if not isinstance(code, str):
                errors.append(f"invalid track member: {track_id}")
                continue
            if code in registry:
                errors.append(f"task repeated in registry: {code}")
            registry[code] = track_id
            if code not in tasks or tasks[code].get("track") != track_id:
                errors.append(f"track/manifest mismatch: {track_id}/{code}")
            if not re.search(r"\*\*" + re.escape(code) + r"\s+—", plan):
                errors.append(f"plan heading missing: {code}")
    for code, task in tasks.items():
        if registry.get(code) != task.get("track"):
            errors.append(f"manifest task not registered: {code}")

    counts = {
        "requirements": len(known_requirements), "tracks": len(track_paths),
        "tasks": len(tasks), "schemas": len(schemas),
        "task_cards": sum("task_card" in task for task in tasks.values()),
    }
    return errors, counts


def main() -> int:
    errors, counts = validate(ROOT)
    if errors:
        print("SCAFFOLD FAILED:", *errors, sep="\n- ")
        return 1
    print("SCAFFOLD OK: " + ", ".join(f"{n} {name}" for name, n in counts.items()))
    print("Validated exact task DAG, references, requirement coverage and independent core closure.")
    print("Planning checks only; no Rust compilation, model accuracy or deployment is certified.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
