"""faker2 — pure-Rust fake-data generator with a thin Python binder.

All logic lives in the compiled Rust core (``faker2._native``, built with
PyO3). This package only binds it to Python.

    >>> from faker2 import Faker, homophones, detect_country
    >>> f = Faker(42)
    >>> f.name()                       # doctest: +SKIP
    'Cheyenne Mills'
    >>> f.first_name_like_real("Jacques", "FR")   # doctest: +SKIP
    'Valentin'

Scope: the Rust core currently implements the engine, the en_US providers and
the 139-country name intelligence (gender / country / homophones). More locales
are ported into the Rust core over time.
"""

import os as _os
import pathlib as _pathlib

# Point the Rust core at the dataset bundled in this package (unless overridden).
_data = _pathlib.Path(__file__).parent / "data"
for _env, _name in (
    ("FAKER2_NAMES_PARQUET", "first_names.parquet"),
    ("FAKER2_BALANCED_PARAMS", "balanced_params.json"),
):
    if _env not in _os.environ:
        _f = _data / _name
        if _f.exists():
            _os.environ[_env] = str(_f)

from ._native import (  # noqa: E402
    Faker,
    available_countries,
    detect_country,
    homophones,
    infer_gender_real,
)

__all__ = [
    "Faker",
    "infer_gender_real",
    "detect_country",
    "homophones",
    "available_countries",
]

__version__ = "40.28.1.post1"
