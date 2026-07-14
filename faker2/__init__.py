from faker2.factory import Factory
from faker2.generator import Generator
from faker2.proxy import Faker

# Tracks upstream Faker's version; `.postN` is this fork's own release counter
# on top of that upstream base. Must stay equal to the VERSION file (setup.py
# reads that file), which tests/test_version.py enforces.
VERSION = "40.28.1.post1"

__all__ = ("Factory", "Generator", "Faker")
