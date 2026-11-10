import unittest
from synthetic_metrics import impact


def log(parent=True):
    return {"objects": [
        {"id": "child", "relationships": [
            {"objectId": "parent", "qualifier": "reply_target_candidate"}] if parent else []},
        {"id": "parent", "relationships": []},
    ], "events": [{"id": "e1"}, {"id": "e2"}]}


class PerturbationMetricTests(unittest.TestCase):
    def test_missing_parent_measures_link_loss_without_new_observations(self):
        result = impact(log(), log(False), -1)
        self.assertEqual(result["reply_candidate_count_delta"], -1)
        self.assertEqual(result["event_count_delta"], 0)
        self.assertTrue(result["expected_direction_met"])

    def test_unchanged_link_preserves_structure(self):
        result = impact(log(), log(), 0)
        self.assertEqual(result["reply_candidate_count_delta"], 0)
        self.assertTrue(result["expected_direction_met"])

    def test_incorrect_expected_direction_is_flagged(self):
        self.assertFalse(impact(log(), log(False), 1)["expected_direction_met"])


if __name__ == "__main__":
    unittest.main()
