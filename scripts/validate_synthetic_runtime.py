#!/usr/bin/env python3
"""End-to-end synthetic communication and counterfactual invariants.

Requires a built Rust CLI. Ground truth is construction metadata, not an
annotation of real-world correspondence.
"""
from pathlib import Path
import json
import subprocess
import sys
import tempfile

from synthetic_mime import write
from synthetic_metrics import impact


def run(binary: Path, *args: str) -> dict:
    completed = subprocess.run([str(binary), *args], capture_output=True, timeout=30)
    if completed.returncode:
        raise AssertionError("synthetic Rust conversion rejected valid fixture")
    return json.loads(completed.stdout)


def reply_edges(log: dict) -> set[tuple[str, str]]:
    return {(obj["id"], rel["objectId"]) for obj in log["objects"]
            for rel in obj["relationships"] if rel["qualifier"] == "reply_target_candidate"}


def main() -> None:
    root = Path(__file__).resolve().parent.parent
    binary = Path(sys.argv[1] if len(sys.argv) > 1 else root / "target/debug/mailweft").resolve()
    if not binary.is_file():
        raise SystemExit("BLOCKED: build the Rust CLI first")
    with tempfile.TemporaryDirectory() as directory:
        base = Path(directory)
        write(base)
        meta = json.loads((base / "manifest.json").read_text())
        def path(name):
            return str(base / "raw" / f"{name}.eml")
        def changed(name):
            return str(base / "perturbed" / f"{name}.eml")
        at = meta["observed_at"]
        original = run(binary, "batch", at, path("root"), path("reply"))
        assert len(original["events"]) == 2
        assert len(reply_edges(original)) == 1
        missing = run(binary, "batch", at, path("root"), changed("reply_target_missing"))
        assert len(reply_edges(missing)) == 0, "missing parent must not fabricate a reply edge"
        missing_impact = impact(original, missing, -1)
        assert missing_impact["expected_direction_met"] and missing_impact["event_count_delta"] == 0
        ambiguous = run(binary, "batch", at, path("root"), path("reply"), changed("message_id_collision"))
        assert len(reply_edges(ambiguous)) == 0, "ambiguous claimed root must abstain"
        assert impact(original, ambiguous, -1)["expected_direction_met"]
        quoted = run(binary, "batch", at, path("root"), changed("quoted_request"))
        assert reply_edges(quoted), "quote-depth perturbation must not remove independent header link"
        assert impact(original, quoted, 0)["expected_direction_met"]
        for doc in (original, missing, ambiguous, quoted):
            assert all(event["type"] == "source_copy_observed" for event in doc["events"])
            assert all(event["time"] == at for event in doc["events"])
        shifted = run(binary, changed("date_shift"), at, "fixture/date-shift")
        baseline = run(binary, path("root"), at, "fixture/date-shift")
        assert shifted["events"][0]["time"] == baseline["events"][0]["time"] == at
        assert shifted["events"][0]["attributes"] != baseline["events"][0]["attributes"]
        print("SYNTHETIC END-TO-END OK: unique, absent and ambiguous reply claims; quote and date perturbations")


if __name__ == "__main__":
    main()
