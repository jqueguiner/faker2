# faker2 — a Rust port of Python Faker (+ improvements)

[![CI](https://github.com/jqueguiner/faker2/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/jqueguiner/faker2/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/jqueguiner/faker2/badges/coverage.json)](https://github.com/jqueguiner/faker2/actions/workflows/coverage-badge.yml)
[![codecov](https://codecov.io/gh/jqueguiner/faker2/branch/master/graph/badge.svg)](https://codecov.io/gh/jqueguiner/faker2)
[![PyPI](https://img.shields.io/pypi/v/faker2.svg?cache=2)](https://pypi.org/project/faker2/)
[![Python versions](https://img.shields.io/pypi/pyversions/faker2.svg?cache=2)](https://pypi.org/project/faker2/)
[![Rust release](https://github.com/jqueguiner/faker2/actions/workflows/rust-release.yml/badge.svg)](https://github.com/jqueguiner/faker2/releases)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.txt)

**`faker2` is a port of the Python [Faker](https://github.com/joke2k/faker) project to
Rust, with a thin Python binder** — `import faker2` runs the compiled Rust engine
(via PyO3); no fake-data logic lives in Python. It keeps Faker's **151 locales**,
generates the same kinds of data **~3–18× faster**, and adds name-intelligence
features the original doesn't have.

```python
import faker2
f = faker2.Faker(42)                  # seedable, deterministic
f.gen("fr_FR", "name")                # 'Édouard Guichard'
f.gen("ja_JP", "address")             # Japanese address
f.gen("en_US", "credit_card_number")  # Luhn-valid
f.homophones("Dominique", "FR")       # improvement: same-sounding names + probabilities
```

Upstream Faker docs: [`README.upstream.rst`](README.upstream.rst).
Python ↔ Rust parity matrix: [`PARITY.md`](PARITY.md).

## Repository layout

```
faker2/                 Python package (upstream faker fork)
  naming/               ← added: name intelligence
    gender.py             gender infer + preserving replace (bundled locales)
    realnames.py          same, backed by data/ ground truth (needs pyarrow)
    grammar.py            pluralize / singularize / agreement
data/                   ← added: name ground truth
  first_names.parquet     1.43M names, 139 countries, gender, relative frequency
  last_names.parquet      small sample (surnames NOT comprehensive)
  README.md               schema + provenance
rust/                   ← added: Rust port
  src/                    engine, providers, gender, grammar, realnames
  README.md               crate docs
tests/                  Python tests (incl. test_gender_grammar.py)
```

## Python — name intelligence

```python
from faker2.naming import realnames, grammar

realnames.infer_gender("Jacques", "FR")       # "m"
realnames.first_name_like("Jacques", "FR")    # frequency-weighted male FR name
realnames.first_name("JP", "f")               # weighted female Japanese name
realnames.detect_country("Yuki")              # [("JP", 0.58), ("CN", 0.05), ...]
realnames.detect_country("Bjorn")             # [("SE", 0.29), ("NO", 0.22), ...]
realnames.homophones("Dominique", "FR")       # [("Dominique", 0.91), ("Dominic", 0.03), ...]

grammar.pluralize("baby")                     # "babies"
grammar.agree(3, "dog")                       # "3 dogs"
```

`realnames` needs `pyarrow` (in `dev-requirements.txt`). The bundled-locale
variant `faker2.naming.gender` has no extra dependency.

`detect_country` ranks where a name is most **characteristic** (its within-country
frequency share), not where the most *people* with that name live — raw
population counts are intentionally not in the dataset.

`homophones` returns same-sounding names in a country with probabilities
(weighted by frequency share, summing to 1). Pick the matching `method`:

- `"metaphone"` (default) — double-metaphone group; fast but coarse (may pull in
  near-homophones, e.g. Sophie↔Xavier).
- `"ipa"` — articulatory phonetic similarity via [g2p2](https://github.com/jqueguiner/g2p2)
  (feature-weighted phoneme alignment), precise.
- `"levenshtein"` — spelling within `max_distance` edits; orthographic variants.
- `"balanced"` — IPA + spelling consensus with **per-country weights** swept
  offline (`scripts/sweep_balanced.py` → `data/balanced_params.json`, 119
  countries). Drops metaphone-only collisions (Sophie↛Xavier).

```python
realnames.homophones("Dominique", "FR", method="ipa")          # Dominique .97, Dominik .02 …
realnames.homophones("Marc", "FR", method="levenshtein", max_distance=1)
realnames.homophones("Sophie", "FR", method="balanced")        # per-country tuned; no Xavier
```

The balanced weights were tuned by sweeping thousands of names per country
against a spelling+metaphone consensus objective (IPA kept independent, since
the dataset's IPA is a single English G2P). Regenerate with
`PYTHONPATH=. python3 scripts/sweep_balanced.py`.

## Rust port

```
cd rust
cargo test                          # zero-dependency core
cargo run -- --seed 42 name
cargo run -- like Jacques fr        # gender-preserving replacement
cargo test --features real-names    # opt-in parquet ground truth
```

See [`rust/README.md`](rust/README.md).

## faker2 (this port) vs Faker (Python)

Original Faker = the pure-Python package (`from faker2 import Faker`, the fork of
[joke2k/Faker](https://github.com/joke2k/faker)). This port = the Rust engine
(`import faker2`, or the crate directly). Same machine, `en_US`.

**Full per-formatter table (63 formatters): [`BENCHMARKS.md`](BENCHMARKS.md)** —
62/63 are faster in Rust, geo-mean **~20×**.

### Speed — throughput (higher is better)

| Formatter | Python | Rust | Speedup |
|---|---:|---:|---:|
| `name` | 0.017 M ops/s | 0.315 M ops/s | **~18×** |
| `email` | 0.017 M ops/s | 0.296 M ops/s | **~17×** |
| `address` | 0.012 M ops/s | 0.162 M ops/s | **~13×** |
| `credit_card_number` (Luhn) | 0.084 M ops/s | 0.292 M ops/s | **~3.5×** |
| `iban` (mod-97) | 0.068 M ops/s | 0.344 M ops/s | **~5×** |

### Hardware — 500k names, `/usr/bin/time -v`

| Metric | Python | Rust | Note |
|---|---:|---:|---|
| CPU / wall time | 29 s | **2 s** | ~15× faster |
| Peak RAM (RSS) | 26 MB | 120 MB | Rust eagerly loads **all 151 locales** (flat); Python lazy-loads one |

RAM is the one honest trade-off: the Rust engine holds the entire 151-locale
dataset resident (constant ~120 MB) for instant locale switching, whereas Python
loads a locale lazily. For multi-locale bulk work Rust's footprint stays flat
while Python's grows per locale.

### Accuracy — generated data validity (2000 samples each)

| Check | Python | Rust |
|---|---:|---:|
| Credit-card Luhn | 100% | 100% |
| IBAN ISO-7064 mod-97 | 100% | 100% |
| EAN-13 GS1 checksum | 100% | 100% |

Identical: both produce valid, checksum-correct data. The Rust checksums are
independently verified in `rust/tests/algo.rs`.

### Coverage

| | Python (original) | Rust (this port) |
|---|---:|---:|
| Locales | 151 | **151** |
| Formatters | ~230 | ~177 (**77%**) |

Every locale is covered; ~77% of formatters are ported (the tail — a few
binary/collection outputs, some per-locale `ssn`/`passport` checksums — is
in progress).

### Improvements over the original Faker

Name intelligence backed by a real **1.43M-name / 139-country** dataset — not in
upstream Faker:

- `infer_gender_real(name, country)` — gender from real data.
- `first_name_like(name, country)` — gender-preserving, frequency-weighted replacement.
- `detect_country(name)` — most likely origin country of a name.
- `homophones(name, country, method=…)` — same-sounding names + probabilities
  (metaphone / IPA / levenshtein / balanced, per-country tuned).
- grammar agreement (pluralize / singularize / count agreement).

### Reproduce

```bash
# Python (original)
PYTHONPATH=. python3 - <<'PY'
from faker2 import Faker; import time
f=Faker("en_US"); n=200000; t=time.perf_counter()
for _ in range(n): f.name()
print(n/(time.perf_counter()-t)/1e6, "M ops/s")
PY
# Rust (this port)
cd rust && cargo run --release --features locales --example benchfull   # if kept
```

## Data provenance

The name dataset was extracted from a PostgreSQL dump and normalized so
`frequency` is a **relative share** — no raw population counts are exposed. See
[`data/README.md`](data/README.md).
