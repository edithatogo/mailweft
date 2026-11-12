"""Deterministic, source-scoped structural error-impact measures.

These metrics compare observed graph outputs to construction-defined synthetic
expectations. They are not population accuracy estimates or causal effects.
"""
from __future__ import annotations


def links(log: dict) -> set[tuple[str, str]]:
    return {(obj["id"], rel["objectId"]) for obj in log["objects"]
            for rel in obj["relationships"] if rel["qualifier"] == "reply_target_candidate"}


def impact(baseline: dict, changed: dict, expected_link_delta: int) -> dict:
    before = links(baseline)
    after = links(changed)
    event_delta = len(changed["events"]) - len(baseline["events"])
    link_delta = len(after) - len(before)
    return {
        "schema_version": "1.0.0",
        "evaluation_scope": "fictional_constructed_cases_only",
        "baseline_events": len(baseline["events"]),
        "changed_events": len(changed["events"]),
        "event_count_delta": event_delta,
        "baseline_reply_candidates": len(before),
        "changed_reply_candidates": len(after),
        "reply_candidate_count_delta": link_delta,
        "expected_link_delta": expected_link_delta,
        "expected_direction_met": link_delta == expected_link_delta,
        "baseline_candidate_edges": sorted([list(edge) for edge in before]),
        "changed_candidate_edges": sorted([list(edge) for edge in after]),
    }
