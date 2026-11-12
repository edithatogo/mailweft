#!/usr/bin/env python3
"""Fail-closed pre-release guard for known locked dependency blockers.

This narrow deterministic policy check is NOT a substitute for a fresh
RustSec audit of the entire dependency graph. No advisory exceptions.
"""
from __future__ import annotations

import argparse
from pathlib import Path
import sys
import tomllib

ROOT = Path(__file__).resolve().parent.parent
ADVISORIES = ("RUSTSEC-2026-0194", "RUSTSEC-2026-0195")
PATCHED_QUICK_XML = (0, 41, 0)


def numeric_version(version: str) -> tuple[int, ...]:
    core = version.partition("+")[0].partition("-")[0]
    components = core.split(".")
    if len(components) != 3 or not all(c.isdecimal() for c in components):
        raise ValueError("unrecognized version")
    return tuple(map(int, components))


def inspect_lock(lock: dict) -> dict:
    packages = lock.get("package")
    if not isinstance(packages, list):
        raise ValueError("Cargo.lock missing package list")
    blocking = []
    for p in packages:
        if not isinstance(p, dict) or p.get("name") != "quick-xml":
            continue
        version = p.get("version")
        if not isinstance(version, str):
            blocking.append({"crate": "quick-xml", "version": "unknown", "advisories": ADVISORIES})
            continue
        try:
            vulnerable = numeric_version(version) < PATCHED_QUICK_XML
        except ValueError:
            vulnerable = True
        if vulnerable:
            blocking.append({"crate": "quick-xml", "version": version, "advisories": ADVISORIES})
    return {
        "status": "blocked" if blocking else "no_known_quick_xml_blocker",
        "blocking": blocking,
        "coverage": "known_quick_xml_advisories_only",
        "full_advisory_scan_required": True,
        "licence_source_audit_required": True,
        "release_authorized": False,
    }


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--lock", type=Path, default=ROOT / "Cargo.lock")
    args = parser.parse_args(argv)
    try:
        report = inspect_lock(tomllib.loads(args.lock.read_text(encoding="utf-8")))
    except (OSError, ValueError, tomllib.TOMLDecodeError):
        print("RELEASE_BLOCKED: invalid or unavailable Cargo.lock", file=sys.stderr)
        return 2
    if report["blocking"]:
        ids = ", ".join(ADVISORIES)
        versions = ", ".join(v["version"] for v in report["blocking"])
        print(f"RELEASE_BLOCKED: quick-xml {versions}; {ids}; patch >=0.41.0", file=sys.stderr)
        return 2
    print("PRECHECK_OK: no known quick-xml blocker; full supply-chain qualification still required")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
