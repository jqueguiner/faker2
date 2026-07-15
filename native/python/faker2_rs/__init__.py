"""faker2-rs — a thin Python wrapper over the Rust ``faker2`` engine.

Everything here delegates to the compiled Rust core (``faker2_rs._native``,
built with PyO3). No fake-data logic lives in Python.

    >>> from faker2_rs import Faker, homophones, detect_country
    >>> f = Faker(42)
    >>> f.name()                       # doctest: +SKIP
    'Cheyenne Mills'
    >>> f.first_name_like_real("Jacques", "FR")   # doctest: +SKIP
    'Valentin'
    >>> detect_country("Yuki", 3)[0][0]           # doctest: +SKIP
    'JP'

Scope: the Rust port currently implements the core engine, the en_US providers
and the data-backed name intelligence (139-country gender / country / homophone
tools). Additional locales are ported onto the same engine over time.
"""

import os as _os
import pathlib as _pathlib

# Point the Rust engine at the dataset bundled inside this package (unless the
# user overrode it). Env is read lazily on first use, so setting it here is safe.
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

__version__ = "0.1.0"
