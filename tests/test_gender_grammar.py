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
