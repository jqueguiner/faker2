"""Real, data-backed names — the "strong ground" for faker.

Loads ``data/first_names.parquet`` (1.43M names across 139 countries, each with
a gender and real-world ``frequency``) and exposes:

  * :func:`infer_gender` -- gender of a name, optionally scoped to a country
  * :func:`first_name`   -- frequency-weighted name for a country/gender
  * :func:`first_name_like` -- gender-preserving replacement using real data

Sampling is **frequency-weighted**: common names appear proportionally more
often. Seeding goes through faker's own RNG (``faker2.generator.random``) so
``Faker.seed(n)`` stays reproducible.

The parquet loads lazily on first use (~0.1s) and is cached process-wide.
"""

import bisect
import os
from functools import lru_cache
from typing import Dict, List, Optional, Tuple

from faker2.generator import random as _fkr_random

# repo_root/data/first_names.parquet  (this file lives in faker2/naming/)
_DATA = os.path.join(
    os.path.dirname(os.path.dirname(os.path.dirname(__file__))),
    "data",
    "first_names.parquet",
)

MALE, FEMALE, UNISEX = "m", "f", "u"


class _NameBank:
    """In-memory indexes built once from the parquet."""

    def __init__(self, path: str = _DATA) -> None:
        import pyarrow.parquet as pq  # local import: optional dependency

        tbl = pq.read_table(
            path, columns=["name", "name_ascii", "country_code", "gender", "frequency"]
        )
        names = tbl.column("name").to_pylist()
        asciis = tbl.column("name_ascii").to_pylist()
        ccs = tbl.column("country_code").to_pylist()
        genders = tbl.column("gender").to_pylist()
        freqs = tbl.column("frequency").to_pylist()

        # (country, gender) -> (names[], cumulative_weights[]).
        # Weights are *relative* frequencies (shares), never raw counts.
        pools: Dict[Tuple[Optional[str], str], Tuple[List[str], List[float]]] = {}
        # name_key -> {gender: summed relative weight}, at country + global scope
        gender_by: Dict[Tuple[str, Optional[str]], Dict[str, float]] = {}
        # name_key -> {country: summed within-country share} for origin detection
        country_by: Dict[str, Dict[str, float]] = {}

        acc: Dict[Tuple[Optional[str], str], List] = {}
        for name, asc, cc, g, fr in zip(names, asciis, ccs, genders, freqs):
            if not g or g == " ":
                continue
            w = float(fr) if fr else 1e-6
            for scope in (cc, None):  # per-country and global pools
                key = (scope, g)
                lst = acc.setdefault(key, [[], []])
                lst[0].append(name)
                lst[1].append(w)
            # index under both accented and ascii forms so lookups round-trip
            for akey in {name.lower(), (asc or name).lower()}:
                for scope in (cc, None):
                    d = gender_by.setdefault((akey, scope), {})
                    d[g] = d.get(g, 0) + w
                if cc:
                    c = country_by.setdefault(akey, {})
                    c[cc] = c.get(cc, 0.0) + w

        # finalize cumulative weights for O(log n) weighted draw
        for key, (nm, wt) in acc.items():
            cum, running = [], 0
            for w in wt:
                running += w
                cum.append(running)
            pools[key] = (nm, cum)

        self._pools = pools
        self._gender_by = gender_by
        self._country_by = country_by

    def infer(self, name: str, country: Optional[str] = None) -> Optional[str]:
        key = name.strip().lower()
        counts = self._gender_by.get((key, country)) or (
            self._gender_by.get((key, None)) if country else None
        )
        if not counts:
            return None
        m, f = counts.get(MALE, 0), counts.get(FEMALE, 0)
        if m and f:
            minority = min(m, f)
            if minority >= 0.20 * (m + f):  # meaningfully used by both -> unisex
                return UNISEX
            return MALE if m > f else FEMALE
        return MALE if m else FEMALE

    def draw(self, country: Optional[str], gender: str, avoid: Optional[str] = None) -> Optional[str]:
        pool = self._pools.get((country, gender)) or self._pools.get((None, gender))
        if not pool:
            return None
        names, cum = pool
        total = cum[-1]
        for _ in range(8):
            r = _fkr_random.random() * total
            idx = bisect.bisect_right(cum, r)
            if idx >= len(names):
                idx = len(names) - 1
            pick = names[idx]
            if not avoid or pick.lower() != avoid.strip().lower():
                return pick
        return pick

    def countries(self) -> List[str]:
        return sorted({cc for (cc, _g) in self._pools if cc})

    def detect_country(self, name: str, top: int = 5) -> List[Tuple[str, float]]:
        """Rank the countries where ``name`` is most characteristic.

        Scores are that name's within-country share, normalized across the
        countries it appears in (so they sum to 1). Returns ``[(cc, score)]``
        highest first, ``[]`` if the name is unknown.
        """
        counts = self._country_by.get(name.strip().lower())
        if not counts:
            return []
        total = sum(counts.values()) or 1.0
        ranked = sorted(counts.items(), key=lambda kv: kv[1], reverse=True)
        return [(cc, w / total) for cc, w in ranked[:top]]


@lru_cache(maxsize=1)
def _bank() -> _NameBank:
    return _NameBank()


def infer_gender(name: str, country: Optional[str] = None) -> Optional[str]:
    """Gender of ``name`` from real data: ``"m"``/``"f"``/``"u"``/``None``.

    ``country`` is an ISO-3166 alpha-2 code (e.g. ``"FR"``); omit for global.
    """
    return _bank().infer(name, country.upper() if country else None)


def detect_country(name: str, top: int = 5) -> List[Tuple[str, float]]:
    """Guess the country/countries a first name most likely comes from.

        detect_country("Giovanni")  # [("IT", 0.34), ("AR", 0.12), ...]

    Returns ``[(country_code, score)]`` ranked high-to-low (scores sum to 1),
    or ``[]`` for an unknown name.
    """
    return _bank().detect_country(name, top)


def first_name(country: Optional[str] = None, gender: str = MALE) -> Optional[str]:
    """Frequency-weighted first name for a country + gender."""
    return _bank().draw(country.upper() if country else None, gender)


def first_name_like(name: str, country: Optional[str] = None) -> str:
    """Replace ``name`` with a frequency-weighted name of the **same** gender.

        first_name_like("Jacques", "FR")  # -> "Nicolas" (common male FR name)
    """
    cc = country.upper() if country else None
    g = _bank().infer(name, cc)
    if g in (None, UNISEX):
        # unknown/unisex -> keep it plausible: draw male or female by chance
        g = MALE if _fkr_random.random() < 0.5 else FEMALE
    repl = _bank().draw(cc, g, avoid=name)
    return repl if repl is not None else name


def available_countries() -> List[str]:
    """ISO codes with name data (139)."""
    return _bank().countries()
