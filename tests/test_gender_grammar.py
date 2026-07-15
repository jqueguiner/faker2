"""Tests for the gender-preserving name replacement + grammar helpers."""

from faker2 import Faker
from faker2.naming.gender import first_name_like, infer_gender, _resolver
from faker2.naming.grammar import agree, is_are, pluralize, singularize


def test_infer_gender_by_locale():
    assert infer_gender("Jacques", "fr_FR") == "M"
    assert infer_gender("Marie", "fr_FR") == "F"
    assert infer_gender("jacques", "fr_FR") == "M"  # case-insensitive
    assert infer_gender("Zzxq", "fr_FR") is None


def test_replacement_preserves_gender():
    Faker.seed(3)
    resolver = _resolver("fr_FR")
    for name in ("Jacques", "Patrick", "Alain"):
        for _ in range(50):
            repl = first_name_like(name, "fr_FR")
            assert resolver.infer(repl) == "M", f"{name} -> {repl}"
            assert repl.lower() != name.lower()
    assert resolver.infer(first_name_like("Marie", "fr_FR")) == "F"


def test_cross_locale_english():
    assert infer_gender("Patrick", "en_US") == "M"
    repl = first_name_like("Patrick", "en_US")
    assert infer_gender(repl, "en_US") in ("M", "U")


def test_grammar_agreement():
    assert pluralize("baby") == "babies"
    assert pluralize("box") == "boxes"
    assert pluralize("knife") == "knives"
    assert pluralize("person") == "people"
    assert singularize("cities") == "city"
    assert singularize("knives") == "knife"
    assert singularize("people") == "person"
    assert agree(1, "apple") == "an apple"
    assert agree(1, "dog") == "a dog"
    assert agree(3, "dog") == "3 dogs"
    assert is_are(1) == "is" and is_are(2) == "are"


def test_grammar_pluralize_branches():
    from faker2.naming.grammar import indefinite_article
    assert pluralize("sheep") == "sheep"          # uncountable
    assert pluralize("wife") == "wives"           # -fe
    assert pluralize("wolf") == "wolves"          # -f
    assert pluralize("bus") == "buses"            # -s
    assert pluralize("church") == "churches"      # -ch
    assert pluralize("day") == "days"             # vowel+y (not -ies)
    assert pluralize("cat") == "cats"             # default
    assert pluralize("MAN") == "men"              # irregular, case-insensitive
    assert indefinite_article("hour") == "a"      # vowel heuristic (letter-based)
    assert indefinite_article("egg") == "an"


def test_grammar_singularize_branches():
    assert singularize("sheep") == "sheep"        # uncountable
    assert singularize("wives") == "wife"         # -ves -> -fe map
    assert singularize("wolves") == "wolf"        # -ves -> -f
    assert singularize("boxes") == "box"          # -es after sibilant
    assert singularize("dogs") == "dog"           # plain -s
    assert singularize("class") == "class"        # -ss unchanged
    assert singularize("fish") == "fish"          # no trailing s


def test_gender_unisex_and_alias():
    from faker2.naming.gender import UNISEX, replace_first_name, _resolver
    r = _resolver("en_US")
    both = r._male & r._female
    if both:                                       # a name in both pools -> unisex
        name = next(iter(both))
        assert r.infer(name) == UNISEX
    # alias delegates to first_name_like
    Faker.seed(5)
    assert isinstance(replace_first_name("John", "en_US"), str)
    # female-only name infers FEMALE; unknown name -> either-pool draw
    from faker2.naming.gender import FEMALE
    fem_only = next(iter(r._female - r._male), None)
    if fem_only:
        assert r.infer(fem_only) == FEMALE
    assert isinstance(r.replace("Zzxqwv"), str)   # unknown -> _draw either pool
