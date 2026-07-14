"""The distribution version and `faker2.VERSION` must never drift apart."""

from pathlib import Path

import faker2


def test_version_matches_version_file():
    version_file = Path(__file__).resolve().parent.parent / "VERSION"
    assert faker2.VERSION == version_file.read_text(encoding="utf-8").strip()


def test_version_is_a_post_release_of_an_upstream_version():
    """faker2 is versioned as <upstream faker version>.postN."""
    base, _, post = faker2.VERSION.partition(".post")
    assert post.isdigit(), f"{faker2.VERSION!r} must end with .postN"
    assert len(base.split(".")) == 3, f"{base!r} is not an upstream x.y.z version"
