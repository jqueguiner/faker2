"""Gender inference + gender-preserving name replacement.

Reverse-indexes a locale's ``first_names_male`` / ``first_names_female`` data
into a name -> gender lookup, then generates a replacement first name of the
*same* inferred gender and locale.

    >>> from faker2.gender import first_name_like, infer_gender
    >>> infer_gender("Jacques", "fr_FR")
    'M'
    >>> first_name_like("Jacques", "fr_FR")   # -> another male French name
    'Patrick'

Because it reads the live provider data, this works for **every** locale that
ships gendered name lists -- no per-language table to maintain here.
"""

from functools import lru_cache
from typing import Optional

from faker2 import Faker

# Gender codes: "M" male, "F" female, "U" unisex (in both pools), None unknown.
MALE = "M"
FEMALE = "F"
UNISEX = "U"


class GenderResolver:
    """Name <-> gender resolver for a single locale, backed by faker data."""

    def __init__(self, locale: str = "en_US") -> None:
        self.locale = locale
        self.fake = Faker(locale)
        self._male, self._female = self._collect()

    def _collect(self):
        male, female = set(), set()
        for provider in self.fake.get_providers():
            for attr, dest in (
                ("first_names_male", male),
                ("first_names_female", female),
            ):
                names = getattr(provider, attr, None)
                if names:
                    dest.update(n.lower() for n in names)
        return male, female

    def infer(self, name: str) -> Optional[str]:
        """Return ``"M"``, ``"F"``, ``"U"`` (unisex) or ``None`` (unknown)."""
        key = name.strip().lower()
        in_m, in_f = key in self._male, key in self._female
        if in_m and in_f:
            return UNISEX
        if in_m:
            return MALE
        if in_f:
            return FEMALE
        return None

    def replace(self, name: str) -> str:
        """A different first name of the same inferred gender/locale."""
        gender = self.infer(name)
        for _ in range(8):
            candidate = self._draw(gender)
            if candidate.lower() != name.strip().lower():
                return candidate
        return self._draw(gender)

    def _draw(self, gender: Optional[str]) -> str:
        if gender == MALE:
            return self.fake.first_name_male()
        if gender == FEMALE:
            return self.fake.first_name_female()
        # unisex / unknown -> either pool
        return self.fake.first_name()


@lru_cache(maxsize=None)
def _resolver(locale: str) -> GenderResolver:
    return GenderResolver(locale)


def infer_gender(name: str, locale: str = "en_US") -> Optional[str]:
    """Infer the gender code of ``name`` within ``locale``."""
    return _resolver(locale).infer(name)


def first_name_like(name: str, locale: str = "en_US") -> str:
    """Replace ``name`` with another first name of the same gender/locale.

    This is the headline helper::

        first_name_like("Jacques", "fr_FR")  # -> "Patrick"
    """
    return _resolver(locale).replace(name)


# Alias matching the shape the user asked for: replace(field_value, locale).
def replace_first_name(name: str, locale: str = "en_US") -> str:
    return first_name_like(name, locale)
