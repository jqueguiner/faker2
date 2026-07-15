"""English grammatical-number agreement helpers.

pluralize / singularize, indefinite article, and count agreement -- rule-based
with a small irregular table. Enough to make generated text read correctly.

    >>> from faker2.grammar import agree, pluralize
    >>> pluralize("baby")
    'babies'
    >>> agree(1, "apple")
    'an apple'
    >>> agree(3, "dog")
    '3 dogs'
"""

_IRREGULAR = {
    "man": "men",
    "woman": "women",
    "child": "children",
    "person": "people",
    "foot": "feet",
    "tooth": "teeth",
    "goose": "geese",
    "mouse": "mice",
    "ox": "oxen",
}
_IRREGULAR_INV = {v: k for k, v in _IRREGULAR.items()}

_UNCOUNTABLE = {
    "sheep",
    "fish",
    "series",
    "species",
    "money",
    "information",
    "equipment",
    "rice",
}

# -ves words whose singular ends in -fe (not -f).
_VES_TO_FE = {"knives": "knife", "wives": "wife", "lives": "life"}


def pluralize(word: str) -> str:
    low = word.lower()
    if low in _UNCOUNTABLE:
        return word
    if low in _IRREGULAR:
        return _IRREGULAR[low]
    if low.endswith("y") and len(low) >= 2 and low[-2] not in "aeiou":
        return word[:-1] + "ies"
    if low.endswith(("s", "x", "z", "ch", "sh")):
        return word + "es"
    if low.endswith("fe"):
        return word[:-2] + "ves"
    if low.endswith("f"):
        return word[:-1] + "ves"
    return word + "s"


def singularize(word: str) -> str:
    low = word.lower()
    if low in _UNCOUNTABLE:
        return word
    if low in _IRREGULAR_INV:
        return _IRREGULAR_INV[low]
    if low in _VES_TO_FE:
        return _VES_TO_FE[low]
    if low.endswith("ies") and len(low) > 3:
        return word[:-3] + "y"
    if low.endswith("ves"):
        return word[:-3] + "f"
    if low.endswith("es") and low[:-2].endswith(("s", "x", "z", "ch", "sh")):
        return word[:-2]
    if low.endswith("s") and not low.endswith("ss"):
        return word[:-1]
    return word


def indefinite_article(word: str) -> str:
    """ "a" or "an" from the leading letter (vowel heuristic)."""
    return "an" if word[:1].lower() in "aeiou" else "a"


def agree(count: int, singular_noun: str) -> str:
    """1 -> "a dog"/"an apple", n -> "3 dogs"."""
    if count == 1:
        return f"{indefinite_article(singular_noun)} {singular_noun}"
    return f"{count} {pluralize(singular_noun)}"


def is_are(count: int) -> str:
    return "is" if count == 1 else "are"
