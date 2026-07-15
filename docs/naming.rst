Name intelligence
=================

This fork adds a ``faker2.naming`` package for gender-aware, country-aware name
work, backed by a real dataset (``data/first_names.parquet`` — 1.43M names
across 139 countries with gender and relative frequency).

See also the repository ``README.md`` and ``PARITY.md`` (Python ↔ Rust parity).

Real, data-backed names
-----------------------

.. code-block:: python

    from faker2.naming import realnames

    realnames.infer_gender("Jacques", "FR")        # "m"
    realnames.first_name("JP", "f")                # weighted female Japanese name
    realnames.first_name_like("Jacques", "FR")     # weighted male FR replacement
    realnames.detect_country("Yuki")               # [("JP", 0.58), ("CN", 0.05), ...]
    realnames.available_countries()                # 139 ISO codes

Requires ``pyarrow`` (installed with the ``dev-requirements.txt``).

Homophones
----------

Same-sounding names in a country, with probabilities that sum to 1. Choose the
matching ``method``:

.. code-block:: python

    realnames.homophones("Dominique", "FR")                       # metaphone (default)
    realnames.homophones("Dominique", "FR", method="ipa")         # IPA edit-distance
    realnames.homophones("Marc", "FR", method="levenshtein")      # spelling edit-distance
    realnames.homophones("Sophie", "FR", method="balanced")       # per-country tuned consensus

* ``metaphone`` — double-metaphone group (fast, coarse).
* ``ipa`` — IPA transcription within ``max_distance`` edits (precise).
* ``levenshtein`` — spelling within ``max_distance`` edits (orthographic).
* ``balanced`` — IPA + spelling consensus with per-country weights swept offline
  (``scripts/sweep_balanced.py`` → ``data/balanced_params.json``).

Bundled-locale gender + grammar
-------------------------------

.. code-block:: python

    from faker2.naming import gender, grammar

    gender.infer_gender("Jacques", "fr_FR")        # "M"
    gender.first_name_like("Jacques", "fr_FR")     # same-gender replacement
    gender.full_name("fr_FR", "f")                 # gender-pure full name

    grammar.pluralize("baby")                       # "babies"
    grammar.agree(3, "dog")                         # "3 dogs"

Rust port
---------

The same name intelligence is implemented in Rust under ``rust/`` (crate
``faker2``); enable the data-backed features with ``--features real-names``.
Prebuilt CLI binaries for every platform are attached to each
`GitHub Release <https://github.com/jqueguiner/faker2/releases>`_.
