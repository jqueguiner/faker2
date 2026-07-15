"""Name intelligence added by this fork.

Three cooperating pieces:

* :mod:`faker2.naming.gender` -- gender inference + gender-preserving name
  replacement using the bundled locale name lists.
* :mod:`faker2.naming.realnames` -- the same, backed by the real
  ``data/first_names.parquet`` ground truth (1.43M names, 139 countries,
  frequency-weighted). Requires ``pyarrow``.
* :mod:`faker2.naming.grammar` -- English number agreement
  (pluralize / singularize / article / count agreement).

Quick start::

    from faker2.naming import realnames, grammar

    realnames.infer_gender("Jacques", "FR")      # "m"
    realnames.first_name_like("Jacques", "FR")   # weighted male FR replacement
    grammar.agree(3, "dog")                       # "3 dogs"
"""

from . import gender, grammar, realnames
from .grammar import agree, indefinite_article, is_are, pluralize, singularize

__all__ = [
    "gender",
    "grammar",
    "realnames",
    "pluralize",
    "singularize",
    "agree",
    "indefinite_article",
    "is_are",
]
