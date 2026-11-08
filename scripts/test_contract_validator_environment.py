"""Check documentation validator dependencies, not application behavior."""
import contextlib
import io
from pathlib import Path
import unittest
from unittest.mock import patch

import validate_contract_examples as validator

ROOT = Path(__file__).resolve().parent.parent

class ContractValidatorEnvironmentTests(unittest.TestCase):
    def test_missing_datetime_checker_is_explicitly_blocked(self):
        supported = {key: value for key, value in validator.FormatChecker.checkers.items()
                     if key != 'date-time'}
        output = io.StringIO()
        with patch.object(validator.FormatChecker, 'checkers', supported):
            with contextlib.redirect_stdout(output), contextlib.redirect_stderr(output):
                result = validator.main()
        self.assertEqual(result, 2)
        self.assertIn('BLOCKED', output.getvalue())
        self.assertIn('date-time', output.getvalue())

    def test_format_dependencies_are_exact_pinned(self):
        lines = set((ROOT / 'requirements-planning.txt').read_text().splitlines())
        self.assertIn('rfc3339-validator==0.1.4', lines)
        self.assertIn('six==1.17.0', lines)

if __name__ == '__main__':
    unittest.main()
