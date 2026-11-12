#!/usr/bin/env python3
"""Independent structural integrity checks for generated OCEL 2.0 JSON.

This checks referential integrity, declaration completeness and qualifier
preservation. It is NOT full OCEL normative schema certification.
"""
from __future__ import annotations


def _types(items: list, description: str) -> dict:
    if not isinstance(items, list):
        raise ValueError(f"{description}: not a list")
    types = {}
    for item in items:
        if not isinstance(item, dict) or not isinstance(item.get("name"), str):
            raise ValueError(f"{description}: malformed type")
        name = item["name"]
        if not name or name in types:
            raise ValueError(f"{description}: duplicate/empty type")
        attrs = item.get("attributes")
        if not isinstance(attrs, list):
            raise ValueError(f"{description}: missing attribute catalogue")
        names = set()
        for att in attrs:
            if not isinstance(att, dict) or not att.get("name") or not att.get("type"):
                raise ValueError(f"{description}: malformed attribute definition")
            if att["name"] in names:
                raise ValueError(f"{description}: duplicate attribute definition")
            names.add(att["name"])
        types[name] = names
    return types


def validate_ocel(log: dict) -> None:
    if not isinstance(log, dict):
        raise ValueError("not an OCEL document")
    object_types = _types(log.get("objectTypes"), "objectTypes")
    event_types = _types(log.get("eventTypes"), "eventTypes")
    objects = log.get("objects")
    events = log.get("events")
    if not isinstance(objects, list) or not isinstance(events, list):
        raise ValueError("objects/events missing")
    object_ids = set()
    for obj in objects:
        if not isinstance(obj, dict):
            raise ValueError("malformed object")
        object_id, typ = obj.get("id"), obj.get("type")
        if not isinstance(object_id, str) or not object_id or object_id in object_ids:
            raise ValueError("duplicate/missing object ID")
        object_ids.add(object_id)
        if typ not in object_types:
            raise ValueError("undeclared object type")

    def check_attributes(item: dict, catalogue: set, object_values: bool):
        attrs = item.get("attributes")
        if not isinstance(attrs, list):
            raise ValueError("attributes missing")
        signatures = set()
        for att in attrs:
            if not isinstance(att, dict) or att.get("name") not in catalogue:
                raise ValueError("undeclared attribute")
            if not isinstance(att.get("value"), str):
                raise ValueError("malformed attribute value")
            if object_values and not isinstance(att.get("time"), str):
                raise ValueError("object attribute missing time")
            key = (att["name"], att.get("time"))
            if key in signatures:
                raise ValueError("duplicate timestamped attribute")
            signatures.add(key)

    def check_links(item: dict):
        relationships = item.get("relationships")
        if not isinstance(relationships, list):
            raise ValueError("relationships missing")
        seen = set()
        for rel in relationships:
            if not isinstance(rel, dict) or rel.get("objectId") not in object_ids:
                raise ValueError("dangling object reference")
            if not isinstance(rel.get("qualifier"), str) or not rel["qualifier"]:
                raise ValueError("missing relationship qualifier")
            key = (rel["objectId"], rel["qualifier"])
            if key in seen:
                raise ValueError("duplicate qualified link")
            seen.add(key)

    for obj in objects:
        check_attributes(obj, object_types[obj["type"]], True)
        check_links(obj)
    event_ids = set()
    for event in events:
        if not isinstance(event, dict) or event.get("type") not in event_types:
            raise ValueError("undeclared event type")
        event_id = event.get("id")
        if not isinstance(event_id, str) or not event_id or event_id in event_ids:
            raise ValueError("duplicate/missing event ID")
        event_ids.add(event_id)
        if not isinstance(event.get("time"), str) or not event["time"]:
            raise ValueError("event timestamp missing")
        check_attributes(event, event_types[event["type"]], False)
        check_links(event)
