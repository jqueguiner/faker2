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

# homophone matching methods
METAPHONE, IPA, LEVENSHTEIN, BALANCED = "metaphone", "ipa", "levenshtein", "balanced"
_DEFAULT_DIST = {IPA: 1, LEVENSHTEIN: 2, BALANCED: 1}
# balanced blend: IPA + spelling drive the score, metaphone is a small bonus.
# Metaphone alone must NOT pass the threshold (else coarse collisions survive).
# Per-country (w_ipa, min, cap) come from scripts/sweep_balanced.py; the values
# below are the swept global default (used when a country is untuned).
_BAL_META_BONUS = 0.05
_BAL_DEFAULT = {"w_ipa": 0.3, "min": 0.55, "cap": 2}
_PARAMS_PATH = os.path.join(
    os.path.dirname(os.path.dirname(os.path.dirname(__file__))),
    "data",
    "balanced_params.json",
)


@lru_cache(maxsize=1)
def _balanced_params() -> dict:
    try:
        import json

        with open(_PARAMS_PATH) as f:
            return json.load(f)
    except (OSError, ValueError):
        return {"_default": _BAL_DEFAULT, "countries": {}}


def _bal_cfg(country: str) -> dict:
    p = _balanced_params()
    return p["countries"].get(country) or p.get("_default") or _BAL_DEFAULT


# IPA diacritics to drop before comparing phonemes (stress, length, spaces)
_IPA_STRIP = str.maketrans("", "", "ˈˌːˑ ./")


def _norm_ipa(s: str) -> str:
    return s.translate(_IPA_STRIP)


def _sim(a: str, b: str) -> float:
    """Normalized similarity in [0,1] from edit distance; 0 if either empty."""
    if not a or not b:
        return 0.0
    m = max(len(a), len(b))
    return 1.0 - _levenshtein(a, b, m) / m


def _levenshtein(a: str, b: str, cap: int) -> int:
    """Edit distance, short-circuiting once it provably exceeds ``cap``."""
    if abs(len(a) - len(b)) > cap:
        return cap + 1
    prev = list(range(len(b) + 1))
    for i, ca in enumerate(a, 1):
        cur = [i]
        best = i
        for j, cb in enumerate(b, 1):
            cost = 0 if ca == cb else 1
            v = min(prev[j] + 1, cur[j - 1] + 1, prev[j - 1] + cost)
            cur.append(v)
            best = min(best, v)
        if best > cap:
            return cap + 1
        prev = cur
    return prev[-1]


class _NameBank:
    """In-memory indexes built once from the parquet."""

    def __init__(self, path: str = _DATA) -> None:
        import pyarrow.parquet as pq  # local import: optional dependency

        tbl = pq.read_table(
            path,
            columns=[
                "name",
                "name_ascii",
                "country_code",
                "gender",
                "frequency",
                "country_share",
                "phonetic",
                "ipa",
            ],
        )
        names = tbl.column("name").to_pylist()
        asciis = tbl.column("name_ascii").to_pylist()
        ccs = tbl.column("country_code").to_pylist()
        genders = tbl.column("gender").to_pylist()
        freqs = tbl.column("frequency").to_pylist()
        shares = tbl.column("country_share").to_pylist()
        phonetics = tbl.column("phonetic").to_pylist()
        ipas = tbl.column("ipa").to_pylist()

        # (country, gender) -> (names[], cumulative_weights[]).
        # Weights are *relative* frequencies (shares), never raw counts.
        pools: Dict[Tuple[Optional[str], str], Tuple[List[str], List[float]]] = {}
        # name_key -> {gender: summed relative weight}, at country + global scope
        gender_by: Dict[Tuple[str, Optional[str]], Dict[str, float]] = {}
        # name_key -> {country: summed within-country share} for origin detection
        country_by: Dict[str, Dict[str, float]] = {}
        # (name_key, country) -> phonetic key; (country, phonetic) -> {name: share}
        phon_key: Dict[Tuple[str, str], str] = {}
        homo: Dict[Tuple[str, str], Dict[str, float]] = {}
        # per-country name table for ipa / levenshtein scans, deduped by name:
        #   cc -> {name: [ascii_lower, ipa_norm, share]}
        by_country: Dict[str, Dict[str, list]] = {}
        ipa_key: Dict[Tuple[str, str], str] = {}

        acc: Dict[Tuple[Optional[str], str], List] = {}
        for name, asc, cc, g, fr, cs, phon, ipa in zip(names, asciis, ccs, genders, freqs, shares, phonetics, ipas):
            if not g or g == " ":
                continue
            w = float(fr) if fr else 1e-6
            csh = float(cs) if cs else 1e-9
            if cc and phon:
                homo.setdefault((cc, phon), {})
                homo[(cc, phon)][name] = homo[(cc, phon)].get(name, 0.0) + csh
            if cc:
                ipa_n = _norm_ipa(ipa) if ipa else ""
                row = by_country.setdefault(cc, {}).get(name)
                if row is None:
                    # [ascii_lower, ipa_norm, share, metaphone]
                    by_country[cc][name] = [
                        (asc or name).lower(),
                        ipa_n,
                        csh,
                        phon or "",
                    ]
                else:
                    row[2] += csh  # accumulate share across genders
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
                    if phon:
                        phon_key[(akey, cc)] = phon
                    if ipa_n:
                        ipa_key[(akey, cc)] = ipa_n

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
        self._phon_key = phon_key
        self._homo = homo
        self._by_country = by_country
        self._ipa_key = ipa_key

    def infer(self, name: str, country: Optional[str] = None) -> Optional[str]:
        key = name.strip().lower()
        counts = self._gender_by.get((key, country)) or (self._gender_by.get((key, None)) if country else None)
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

    def homophones(
        self,
        name: str,
        country: str,
        method: str = METAPHONE,
        top: int = 10,
        include_self: bool = True,
        max_distance: Optional[int] = None,
    ) -> List[Tuple[str, float]]:
        """Names that sound like ``name`` in ``country``, with probabilities.

        ``method`` selects how candidates are matched:

        * ``"metaphone"`` — exact double-metaphone group (fast, coarse).
        * ``"ipa"`` — IPA transcription within ``max_distance`` edits (precise).
        * ``"levenshtein"`` — spelling within ``max_distance`` edits (orthographic).

        Each candidate is weighted by its country-wide frequency share; the
        returned probabilities sum to 1, highest first. ``[]`` if unknown.
        """
        key = name.strip().lower()
        if method == METAPHONE:
            phon = self._phon_key.get((key, country))
            group = self._homo.get((country, phon)) if phon else None
            items = list(group.items()) if group else []
        elif method == BALANCED:
            items = self._balanced(key, country, max_distance)
        else:
            items = self._fuzzy(key, country, method, max_distance)
        if not items:
            return []
        if not include_self:
            items = [(n, w) for n, w in items if n.lower() != key]
        total = sum(w for _n, w in items) or 1.0
        ranked = sorted(items, key=lambda kv: kv[1], reverse=True)
        return [(n, w / total) for n, w in ranked[:top]]

    def _balanced(self, key: str, country: str, max_distance: Optional[int]) -> List[Tuple[str, float]]:
        """Consensus of IPA + metaphone + spelling, weighted by frequency.

        A candidate scores high only when several signals agree, so
        method-specific false positives (e.g. a metaphone-only collision that
        is IPA-far) fall below threshold and drop out.
        """
        table = self._by_country.get(country)
        if not table:
            return []
        q_ipa = self._ipa_key.get((key, country), "")
        q_phon = self._phon_key.get((key, country), "")
        cfg = _bal_cfg(country)
        w_ipa, w_spell, bal_min = cfg["w_ipa"], 1.0 - cfg["w_ipa"], cfg["min"]
        cap = max_distance if max_distance is not None else cfg["cap"]

        # candidate pool: metaphone group ∪ IPA-close names (bounds the scan)
        pool: set = set()
        if q_phon:
            grp = self._homo.get((country, q_phon))
            if grp:
                pool.update(grp)
        if q_ipa:
            for nm, row in table.items():
                if row[1] and _levenshtein(q_ipa, row[1], cap) <= cap:
                    pool.add(nm)

        out = []
        for nm in pool:
            r = table.get(nm)
            if not r:
                continue
            ascii_n, ipa_n, share, phon = r
            ipa_sim = _sim(q_ipa, ipa_n)
            spell_sim = _sim(key, ascii_n)
            meta = 1.0 if q_phon and phon == q_phon else 0.0
            if q_ipa and ipa_n:
                base = w_ipa * ipa_sim + w_spell * spell_sim
            else:  # no IPA to compare -> lean on spelling (+ metaphone signal)
                base = 0.5 * spell_sim + 0.5 * meta
            sim = min(1.0, base + _BAL_META_BONUS * meta)
            if sim >= bal_min or nm.lower() == key:
                out.append((nm, share * sim))
        return out

    def _fuzzy(self, key: str, country: str, method: str, max_distance: Optional[int]) -> List[Tuple[str, float]]:
        table = self._by_country.get(country)
        if not table:
            return []
        cap = max_distance if max_distance is not None else _DEFAULT_DIST.get(method, 2)
        if method == IPA:
            query = self._ipa_key.get((key, country))
            field = 1
        elif method == LEVENSHTEIN:
            query = key
            field = 0
        else:
            raise ValueError(f"unknown method {method!r}")
        if not query:
            return []
        out = []
        for nm, row in table.items():
            target = row[field]
            if not target:
                continue
            if _levenshtein(query, target, cap) <= cap:
                out.append((nm, row[2]))
        return out

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


def homophones(
    name: str,
    country: str,
    method: str = METAPHONE,
    top: int = 10,
    include_self: bool = True,
    max_distance: Optional[int] = None,
) -> List[Tuple[str, float]]:
    """Same-sounding names in a country, with probabilities.

        homophones("Dominique", "FR")                    # metaphone (default)
        homophones("Dominique", "FR", method="ipa")      # precise, IPA edit-distance
        homophones("Dominique", "FR", method="levenshtein")  # spelling edit-distance
        homophones("Dominique", "FR", method="balanced") # consensus of all three

    ``method`` is ``"metaphone"`` | ``"ipa"`` | ``"levenshtein"`` | ``"balanced"``.
    ``"balanced"`` blends IPA + metaphone + spelling similarity so only names that
    agree across signals survive. Candidates are weighted by country-wide
    frequency share; probabilities sum to 1. ``[]`` if the name is unknown.
    """
    return _bank().homophones(
        name,
        country.upper() if country else "",
        method,
        top,
        include_self,
        max_distance,
    )


def first_name(country: Optional[str] = None, gender: str = MALE) -> Optional[str]:
    """Frequency-weighted first name for a country + gender."""
    return _bank().draw(country.upper() if country else None, gender)


def first_name_like(name: str, country: Optional[str] = None) -> str:
    """Replace ``name`` with a frequency-weighted name of the **same** gender.

    first_name_like("Jacques", "FR")  # -> "Nicolas" (common male FR name)
    """
    cc = country.upper() if country else None
    inferred = _bank().infer(name, cc)
    if inferred is None or inferred == UNISEX:
        # unknown/unisex -> keep it plausible: draw male or female by chance
        g = MALE if _fkr_random.random() < 0.5 else FEMALE
    else:
        g = inferred
    repl = _bank().draw(cc, g, avoid=name)
    return repl if repl is not None else name


def available_countries() -> List[str]:
    """ISO codes with name data (139)."""
    return _bank().countries()
