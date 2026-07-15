# faker2 — fork with name intelligence + a Rust port

This fork of [faker2](README.upstream.rst) adds three things on top of upstream:

1. **Name intelligence** (`faker2/naming/`) — gender inference, gender-preserving
   name replacement, and English grammatical-number agreement.
2. **A real name ground-truth dataset** (`data/`) — 1.43M first names across 139
   countries with gender + relative frequency, as Parquet.
3. **A Rust port** (`rust/`) — the core engine + key providers + the same name
   intelligence, re-implemented in Rust.

Upstream faker docs: [`README.upstream.rst`](README.upstream.rst).

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

grammar.pluralize("baby")                     # "babies"
grammar.agree(3, "dog")                       # "3 dogs"
```

`realnames` needs `pyarrow` (in `dev-requirements.txt`). The bundled-locale
variant `faker2.naming.gender` has no extra dependency.

## Rust port

```
cd rust
cargo test                          # zero-dependency core
cargo run -- --seed 42 name
cargo run -- like Jacques fr        # gender-preserving replacement
cargo test --features real-names    # opt-in parquet ground truth
```

See [`rust/README.md`](rust/README.md).

## Benchmarks

Rust (release) vs Python, 1,000,000 operations each. The real-names rows run the
**same algorithm** on both sides (apples-to-apples); "basic name gen" compares
the Rust port against upstream faker's heavier code path, so treat it as
indicative only.

| Task (1M ops) | Python | Rust (release) | Speedup |
|---|---:|---:|---:|
| `infer_gender` (real, 139 countries) | 2.79 M ops/s | 12.79 M ops/s | ~4.6× |
| `first_name_like` (real, freq-weighted) | 0.84 M ops/s | 3.79 M ops/s | ~4.5× |
| parquet bank load (1.43M rows, one-time) | 8.9 s | 2.3 s | ~3.9× |
| basic name generation | 0.037 M ops/s | 11.90 M ops/s | ~320× \* |

\* Not identical logic — Rust port vs upstream faker `first_name`.

Reproduce:

```bash
# Rust
cd rust && cargo run --release --features real-names --example bench
# Python
PYTHONPATH=. python3 scripts/bench_naming.py
```

## Data provenance

The name dataset was extracted from a PostgreSQL dump and normalized so
`frequency` is a **relative share** — no raw population counts are exposed. See
[`data/README.md`](data/README.md).
