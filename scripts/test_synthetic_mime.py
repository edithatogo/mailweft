"""Reproducibility, byte integrity and counterfactual-control tests."""
import hashlib
import json
from pathlib import Path
import tempfile
import unittest

from synthetic_mime import cases, manifest, perturbations, write


class SyntheticMimeTests(unittest.TestCase):
    def test_byte_reproducibility_and_no_real_corpus_dependency(self):
        with tempfile.TemporaryDirectory() as a, tempfile.TemporaryDirectory() as b:
            write(Path(a))
            write(Path(b))
            first = sorted(p.relative_to(a) for p in Path(a).rglob("*") if p.is_file())
            second = sorted(p.relative_to(b) for p in Path(b).rglob("*") if p.is_file())
            self.assertEqual(first, second)
            for name in first:
                self.assertEqual((Path(a) / name).read_bytes(), (Path(b) / name).read_bytes())
            meta = json.loads((Path(a) / "manifest.json").read_text())
            self.assertTrue(meta["synthetic_only"])
            for record in meta["originals"] + meta["perturbations"]:
                raw = (Path(a) / record["file"]).read_bytes()
                self.assertEqual(record["sha256"], hashlib.sha256(raw).hexdigest())
                self.assertEqual(record["bytes"], len(raw))
                self.assertIn(b"example.invalid", raw)

    def test_every_perturbation_changes_exactly_one_source(self):
        originals = {name: raw for name, raw, _ in cases()}
        expected = {x[0] for x in perturbations()}
        self.assertEqual(len(expected), len(perturbations()))
        for name, source, changed, claim in perturbations():
            self.assertNotEqual(originals[source], changed, name)
            self.assertTrue(claim)
            self.assertIn(b"Message-ID:", changed)

    def test_manifest_has_no_external_or_private_data(self):
        meta = manifest()
        self.assertEqual(len(meta["originals"]), 4)
        self.assertEqual(len(meta["perturbations"]), 5)
        self.assertNotIn("enron", json.dumps(meta).lower())
        self.assertNotIn("fyi.org.nz", json.dumps(meta).lower())

    def test_quoted_request_not_an_independent_new_action(self):
        before = next(raw for name, raw, _ in cases() if name == "reply")
        after = next(raw for name, source, raw, _ in perturbations() if name == "quoted_request")
        self.assertIn(b"> Please review", before)
        self.assertIn(b">> Please review", after)
        self.assertEqual(before.count(b"Please review"), after.count(b"Please review"))


if __name__ == "__main__":
    unittest.main()
