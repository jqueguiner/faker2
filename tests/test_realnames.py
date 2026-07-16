"""Tests for the parquet-backed real-name provider (faker2.naming.realnames)."""

import pytest

pytest.importorskip("pyarrow")

from faker2 import Faker  # noqa: E402
from faker2.naming import realnames as rn  # noqa: E402


def test_infer_gender_scopes():
    assert rn.infer_gender("Jacques", "FR") == "m"
    assert rn.infer_gender("Marie", "FR") == "f"
    assert rn.infer_gender("jacques", "fr") == "m"  # case-insensitive + lc code
    assert rn.infer_gender("Zzxqwv", "FR") is None  # unknown
    assert rn.infer_gender("Patrick") in ("m", "f", "u")  # global scope (no country)


def test_first_name_weighted():
    Faker.seed(1)
    assert isinstance(rn.first_name("JP", "f"), str)
    assert isinstance(rn.first_name("BR", "m"), str)
    assert rn.first_name(None, "m")  # global pool
    assert rn.first_name("ZZ", "m")  # unknown country -> global fallback


def test_first_name_like_preserves_gender():
    Faker.seed(42)
    for src in ("Jacques", "Julien", "Pierre"):
        for _ in range(40):
            repl = rn.first_name_like(src, "FR")
            assert rn.infer_gender(repl, "FR") in ("m", "u")
    assert rn.infer_gender(rn.first_name_like("Marie", "FR"), "FR") in ("f", "u")


def test_first_name_like_unknown_input():
    Faker.seed(7)
    # unknown name -> random gender, still returns a plausible name
    out = rn.first_name_like("Zzxqwv", "FR")
    assert isinstance(out, str) and out


def test_available_countries():
    ccs = rn.available_countries()
    assert len(ccs) >= 130
    assert "FR" in ccs and "US" in ccs
    assert ccs == sorted(ccs)


def test_unisex_detection_if_present():
    # find a name used by both genders in one country from the bank, expect "u"
    bank = rn._bank()
    found = None
    for (key, cc), counts in bank._gender_by.items():
        if cc and counts.get("m", 0) > 0 and counts.get("f", 0) > 0:
            minority = min(counts["m"], counts["f"])
            if minority >= 0.20 * (counts["m"] + counts["f"]):
                found = (key, cc)
                break
    if found:
        key, cc = found
        assert rn.infer_gender(key, cc) == "u"


def test_detect_country():
    yuki = rn.detect_country("Yuki", top=3)
    assert yuki and yuki[0][0] == "JP"  # top guess Japan
    assert all(0.0 <= s <= 1.0 for _cc, s in yuki)  # normalized scores
    assert abs(sum(s for _cc, s in rn.detect_country("Yuki", top=999)) - 1.0) < 1e-6
    bjorn = [cc for cc, _ in rn.detect_country("Bjorn", top=4)]
    assert "SE" in bjorn or "NO" in bjorn  # Nordic
    assert rn.detect_country("Zzxqwv") == []  # unknown -> empty
    assert len(rn.detect_country("Maria", top=2)) <= 2  # top-N respected


def test_homophones():
    h = rn.homophones("Dominique", "FR", top=6)
    names = [n for n, _p, _s in h]
    assert h[0][0] == "Dominique"  # most frequent variant first
    assert "Dominic" in names and "Dominik" in names  # same-sounding variants
    assert abs(sum(p for _n, p, _s in rn.homophones("Dominique", "FR", top=999)) - 1.0) < 1e-6
    # symmetric: Dominic sees the same group
    assert "Dominique" in [n for n, _, _ in rn.homophones("Dominic", "FR")]
    # exclude_self drops the query name
    assert "Dominique" not in [n for n, _, _ in rn.homophones("Dominique", "FR", include_self=False)]
    assert rn.homophones("Zzxqwv", "FR") == []  # unknown -> empty
    assert len(rn.homophones("Marc", "FR", top=3)) <= 3  # top-N respected


def test_homophones_methods():
    import pytest as _pytest

    # each method returns a normalized distribution containing the query
    for method in ("metaphone", "ipa", "levenshtein", "balanced"):
        h = rn.homophones("Dominique", "FR", method=method, top=999)
        assert h and h[0][0] == "Dominique"
        assert abs(sum(p for _n, p, _s in h) - 1.0) < 1e-6

    # IPA and balanced are more precise than metaphone: Xavier collides in
    # metaphone (coarse) but not in IPA or balanced.
    meta = [n for n, _, _ in rn.homophones("Sophie", "FR", method="metaphone", top=20)]
    ipa = [n for n, _, _ in rn.homophones("Sophie", "FR", method="ipa", top=20)]
    bal = [n for n, _, _ in rn.homophones("Sophie", "FR", method="balanced", top=20)]
    assert "Xavier" in meta and "Xavier" not in ipa and "Xavier" not in bal

    # levenshtein respects max_distance (all within edit distance 1 of "marc")
    from faker2.naming.realnames import _levenshtein

    lev = rn.homophones("Marc", "FR", method="levenshtein", max_distance=1)
    assert all(_levenshtein("marc", n.lower(), 1) <= 1 for n, _, _ in lev)

    assert rn.homophones("Zzxqwv", "FR", method="ipa") == []
    with _pytest.raises(ValueError):
        rn.homophones("Marc", "FR", method="nonsense")


def test_levenshtein_helper():
    from faker2.naming.realnames import _levenshtein

    assert _levenshtein("marc", "mark", 5) == 1
    assert _levenshtein("kitten", "sitting", 5) == 3
    assert _levenshtein("abc", "abc", 5) == 0
    assert _levenshtein("abc", "xyzuvw", 2) == 3  # capped early -> cap+1


def test_seed_reproducible():
    Faker.seed(99)
    a = [rn.first_name_like("Jacques", "FR") for _ in range(5)]
    Faker.seed(99)
    b = [rn.first_name_like("Jacques", "FR") for _ in range(5)]
    assert a == b
