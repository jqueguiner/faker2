faker2
======

|ci| |coverage| |pypi| |pyversions| |license|

**faker2** is a fork of `joke2k/Faker <https://github.com/joke2k/faker>`_ — the
reference Python library for generating fake data — extended for one specific job:
**pseudonymizing personally identifiable information (PII) in speech and text
datasets, in every language a speech model transcribes.**

Install it alongside upstream Faker if you want — the import name is ``faker2``, so
both can coexist::

    pip install faker2

    from faker2 import Faker
    Faker("ml_IN").name()   # ശ്രീദേവി വാസുദേവൻ

The upstream project's own README is kept, unmodified, as ``README.upstream.rst``.

----

Why this fork exists
--------------------

We build multilingual ASR and PII-redaction models at `Gladia <https://gladia.io>`_.
Training data must be **pseudonymized**: every real person and company name is
replaced by a plausible surrogate, in the *same language and script* as the source
text. Upstream Faker is excellent, but it was not designed for that use case, and
three gaps blocked us:

1. **Missing languages.** Whisper transcribes 100 languages; upstream Faker ships a
   person provider for only 56 of them. A Malayalam or Amharic transcript had no
   locale to draw surrogates from, so names either leaked through or were replaced
   by Latin-script placeholders — both unacceptable.

2. **Name pools too small for dataset scale.** Pseudonymizing a corpus draws tens of
   thousands of surrogates. ``ja_JP`` ships **51 given names**: 223 Japanese person
   entities could only yield 50 distinct surrogates, so the same fake name recurred
   dozens of times. A NER model trained on that data **memorizes the surrogate list**
   instead of learning the pattern — the dataset teaches the wrong thing.

3. **Company names collapsing.** Several locales had two or three company suffixes
   and a single format, so a few hundred draws produced barely a hundred distinct
   companies — the same memorization problem, for organizations.

None of this is a criticism of upstream: generating a handful of fake records for a
test fixture — Faker's core use case — has entirely different requirements from
generating a hundred thousand surrogates that a neural network will then train on.

What this fork adds
-------------------

**48 new locales** — person + company providers with authentic native-script data,
bringing coverage to **all 100 Whisper languages**:

  ``af_ZA`` ``am_ET`` ``as_IN`` ``ba_RU`` ``be_BY`` ``bo_CN`` ``br_FR`` ``bs_BA``
  ``cy_GB`` ``eu_ES`` ``fo_FO`` ``gl_ES`` ``haw_US`` ``ht_HT`` ``jw_ID`` ``kk_KZ``
  ``km_KH`` ``kn_IN`` ``la`` ``lb_LU`` ``ln_CD`` ``lo_LA`` ``mg_MG`` ``mi_NZ``
  ``ml_IN`` ``mn_MN`` ``ms_MY`` ``mt_MT`` ``my_MM`` ``oc_FR`` ``pa_IN`` ``ps_AF``
  ``sa_IN`` ``sd_PK`` ``si_LK`` ``sn_ZW`` ``so_SO`` ``sq_AL`` ``sr_RS`` ``su_ID``
  ``te_IN`` ``tg_TJ`` ``tk_TM`` ``tl_PH`` ``tt_RU`` ``ur_PK`` ``yi_DE`` ``yue_HK``

**Larger given-name pools** where upstream's were too small for dataset-scale
pseudonymization: ``ja_JP`` 51 → 160, ``ko_KR`` 121 → 186, ``pl_PL`` 153 → 205, plus
120+ entries per list across the new locales.

**Diversity-hardened company providers**: a sector component was added to eight
locales whose output space was collapsing, lifting unique-company ratios from 65–79 %
to 94–97 % over 300 draws.

**Tests that check what matters for this use case** — 288 of them, on top of
upstream's suite (2 742 passing in total):

- names are emitted in the **expected script** (a Malayalam locale must not produce
  Latin names);
- **pool sizes** stay above the memorization threshold;
- **≥ 85 % unique full names** over 400 seeded draws, **≥ 70 % unique companies**;
- ``ja_JP`` kana/romaji completeness;
- every Whisper language resolves to a working locale.

**Merged upstream pull requests** that were open and unreleased — mostly IBAN/bank
correctness fixes (``uk_UA``, ``ru_RU``, ``no_NO``, ``nl_BE``, ``en_IE``, ``pt_BR``,
``da_DK``, ``es_ES``, ``en_IN``/``es_MX``/``zh_CN``), the ``en_IN`` company / IFSC /
license-plate providers, and a ``choices_distribution`` ``IndexError`` fix.

Relationship with upstream
--------------------------

- **All credit for Faker goes to** `joke2k <https://github.com/joke2k>`_ **and its
  contributors.** This fork exists to add locales and scale properties — not to
  replace or compete with the original project. If you do not need the above, use
  upstream Faker: ``pip install Faker``.
- The fork tracks upstream ``master``, keeps its **MIT license** and its architecture
  unchanged; new locales follow the project's exact provider conventions.
- Locale contributions made here are intended to be **offered back upstream** where
  they fit the project's scope.
- ``faker2`` uses its own import name (``faker2``), so it **can be installed side by
  side** with upstream ``Faker`` in the same environment.

Usage
-----

Identical to upstream — see the `Faker documentation <https://faker.readthedocs.io>`_.
The only difference is the set of available locales::

    from faker2 import Faker

    Faker("yue_HK").name()      # 陳嘉敏
    Faker("sr_RS").name()       # Милена Чолић
    Faker("am_ET").company()    # company name in Ge'ez script

Pseudonymization tip — seed per source item so the mapping is reproducible without
storing any real→fake table::

    import hashlib
    from faker2 import Faker

    def surrogate(original: str, text: str, locale: str) -> str:
        seed = int(hashlib.sha1(f"{text}\0{original}".encode()).hexdigest()[:12], 16)
        fake = Faker(locale)
        fake.seed_instance(seed)
        return fake.name()

License
-------

MIT, unchanged from upstream. See ``LICENSE.txt``.

.. |ci| image:: https://github.com/jqueguiner/faker2/actions/workflows/ci.yml/badge.svg?branch=master
   :target: https://github.com/jqueguiner/faker2/actions/workflows/ci.yml
   :alt: CI
.. |coverage| image:: https://codecov.io/gh/jqueguiner/faker2/branch/master/graph/badge.svg
   :target: https://codecov.io/gh/jqueguiner/faker2
   :alt: Coverage
.. |pypi| image:: https://img.shields.io/pypi/v/faker2.svg
   :target: https://pypi.org/project/faker2/
   :alt: PyPI version
.. |pyversions| image:: https://img.shields.io/pypi/pyversions/faker2.svg
   :target: https://pypi.org/project/faker2/
   :alt: Supported Python versions
.. |license| image:: https://img.shields.io/badge/license-MIT-blue.svg
   :target: https://github.com/jqueguiner/faker2/blob/master/LICENSE.txt
   :alt: License: MIT
