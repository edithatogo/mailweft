"""Negative mutation tests for the independent OCEL structural checker."""
from copy import deepcopy
import unittest
from validate_ocel_integrity import validate_ocel


def fixture():
    return {
        "objectTypes": [
            {"name": "message", "attributes": [{"name": "source", "type": "string"}]}
        ],
        "eventTypes": [
            {"name": "observed", "attributes": [{"name": "digest", "type": "string"}]}
        ],
        "objects": [
            {"id": "o1", "type": "message",
             "attributes": [{"name": "source", "value": "known", "time": "2026-11-09T00:00:00Z"}],
             "relationships": []}
        ],
        "events": [
            {"id": "e1", "type": "observed", "time": "2026-11-09T00:00:00Z",
             "attributes": [{"name": "digest", "value": "abc"}],
             "relationships": [{"objectId": "o1", "qualifier": "message"}]}
        ],
    }


class IntegrityTests(unittest.TestCase):
    def test_valid_fixture(self):
        validate_ocel(fixture())

    def test_dangling_event_link_fails(self):
        log = fixture()
        log["events"][0]["relationships"][0]["objectId"] = "missing"
        with self.assertRaisesRegex(ValueError, "dangling"):
            validate_ocel(log)

    def test_undeclared_object_type_fails(self):
        log = fixture()
        log["objects"][0]["type"] = "attachment"
        with self.assertRaisesRegex(ValueError, "undeclared"):
            validate_ocel(log)

    def test_undeclared_object_attribute_fails(self):
        log = fixture()
        log["objects"][0]["attributes"][0]["name"] = "new-field"
        with self.assertRaisesRegex(ValueError, "undeclared"):
            validate_ocel(log)

    def test_duplicate_object_id_fails(self):
        log = fixture()
        log["objects"].append(deepcopy(log["objects"][0]))
        with self.assertRaisesRegex(ValueError, "duplicate"):
            validate_ocel(log)

    def test_duplicate_link_fails(self):
        log = fixture()
        log["events"][0]["relationships"].append(deepcopy(log["events"][0]["relationships"][0]))
        with self.assertRaisesRegex(ValueError, "duplicate"):
            validate_ocel(log)

    def test_missing_qualifier_fails(self):
        log = fixture()
        log["events"][0]["relationships"][0]["qualifier"] = ""
        with self.assertRaisesRegex(ValueError, "qualifier"):
            validate_ocel(log)


if __name__ == "__main__":
    unittest.main()
