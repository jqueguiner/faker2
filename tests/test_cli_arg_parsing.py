import unittest

from unittest.mock import patch

from faker2.cli import Command


class TestCLIArgParsing(unittest.TestCase):
    def test_cli_include_argument_parsing(self):
        """Test that the include flag correctly differentiates between a provider and the fake command."""
        cmd = Command(["faker2", "-i", "my.provider", "profile"])

        with patch("faker2.cli.Faker"), patch("faker2.cli.print_doc") as mock_print_doc:
            cmd.execute()

            # Verify that 'my.provider' is treated as an include and 'profile' is not.
            call_args = mock_print_doc.call_args
            kwargs = call_args[1]
            includes = kwargs.get("includes")

            self.assertIn("my.provider", includes)
            self.assertNotIn("profile", includes)

    def test_cli_multiple_includes(self):
        """Test that multiple include flags are correctly accumulated."""
        cmd = Command(["faker2", "-i", "p1", "-i", "p2", "profile"])

        with patch("faker2.cli.Faker"), patch("faker2.cli.print_doc") as mock_print_doc:
            cmd.execute()
            kwargs = mock_print_doc.call_args[1]
            includes = kwargs.get("includes")
            self.assertEqual(includes, ["p1", "p2"])


if __name__ == "__main__":
    unittest.main()
