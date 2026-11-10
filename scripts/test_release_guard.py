"""Synthetic tests for release blocker detection (no advisory ignores)."""
import unittest
import release_guard


class ReleaseGuardTests(unittest.TestCase):
    def test_vulnerable_locked_quick_xml_blocks(self):
        result = release_guard.inspect_lock({"package": [
            {"name": "process_mining", "version": "0.6.2"},
            {"name": "quick-xml", "version": "0.37.5"}
        ]})
        self.assertEqual(result["status"], "blocked")
        self.assertFalse(result["release_authorized"])
        self.assertEqual(set(result["blocking"][0]["advisories"]), set(release_guard.ADVISORIES))

    def test_patched_version_does_not_get_falsely_blocked(self):
        result = release_guard.inspect_lock({"package": [
            {"name": "quick-xml", "version": "0.41.0"}
        ]})
        self.assertFalse(result["blocking"])
        self.assertFalse(result["release_authorized"])
        self.assertTrue(result["full_advisory_scan_required"])

    def test_other_dependencies_still_require_full_scan(self):
        result = release_guard.inspect_lock({"package": [
            {"name": "other-crate", "version": "1.0.0"}
        ]})
        self.assertEqual(result["status"], "no_known_quick_xml_blocker")
        self.assertFalse(result["release_authorized"])

    def test_multiple_versions_cannot_hide_vulnerable_version(self):
        result = release_guard.inspect_lock({"package": [
            {"name": "quick-xml", "version": "0.41.0"},
            {"name": "quick-xml", "version": "0.37.5"},
        ]})
        self.assertEqual(len(result["blocking"]), 1)

    def test_unparseable_version_fails_closed(self):
        result = release_guard.inspect_lock({"package": [
            {"name": "quick-xml", "version": "unknown"}
        ]})
        self.assertEqual(result["status"], "blocked")

    def test_missing_packages_is_not_interpreted_as_secure(self):
        with self.assertRaises(ValueError):
            release_guard.inspect_lock({})


if __name__ == "__main__":
    unittest.main()
