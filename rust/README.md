# faker2 (Rust)

A Rust port of the [`faker2`](../) Python fake-data generator.

This port implements the **core engine** and the most-used **`en_US` providers**.
Data lists are extracted verbatim from the Python source, so output is real
faker data — not placeholders.

## What's ported

| Piece | Python source | Rust |
|-------|---------------|------|
| Seedable RNG + `random_int` / `random_element` | `generator.py`, `providers/__init__.py` | `src/rng.rs` |
| `numerify` `lexify` `bothify` `hexify` | `BaseProvider` | `src/rng.rs` |
| `{{token}}` templating (`parse`/`format`) | `generator.parse` | `src/faker.rs` |
| person, address, internet, lorem, company, phone, color, misc/python | `providers/*` (en_US) | `src/providers/*.rs` |

Not ported: the ~130 non-`en_US` locales, and the long tail of providers
(bank, ssn, credit_card, isbn, automotive, …). The architecture is extensible —
add a `data.rs` list + an `impl Faker` method + a `format()` arm.

## Library

```rust
use faker2::Faker;

let fake = Faker::seeded(42);      // deterministic; Faker::new() for random
println!("{}", fake.name());       // Cheyenne Mills
println!("{}", fake.email());      // cheyenne2008@owens.net
println!("{}", fake.address());    // 40258 Herrera Manor Suite 240\nGrayburgh, MD 67882
println!("{}", fake.parse("{{first_name}} works at {{company}}"));
```

## CLI

```
cargo run -- name                  # single value
cargo run -- --seed 42 email       # deterministic
cargo run -- --repeat 3 address    # 3 rows
cargo run -- --list                # available formatters
```

## Gender-preserving name replacement

Infers a first name's gender from the locale's name lists, then returns a
different name of the **same** gender + locale:

```rust
use faker2::{Faker, Gender, Locale};

let f = Faker::seeded(1);
assert_eq!(Faker::infer_gender("Jacques", Locale::FrFR), Gender::Male);
let repl = f.first_name_like("Jacques", Locale::FrFR);   // e.g. "Patrick"
// repl is guaranteed male-FR.

f.name_of(Gender::Female, Locale::EnUS);   // prefix/first/suffix all agree
```

CLI:

```
cargo run -- --seed 3 like Jacques fr        # -> Auguste  (Male)
cargo run -- --repeat 3 like Marie fr_FR
```

Locales with name data: `en_US`, `fr_FR` (extend by adding arrays to
`data.rs` + a `Locale` arm).

## Real name ground truth (opt-in `real-names` feature)

Backs name generation with **1.43M real names across 139 countries**, each with
a gender and real-world frequency (`../data/first_names.parquet`). Off by
default (keeps the core zero-dep); enable with `--features real-names`.

```rust
use faker2::{Faker, Gender};

Faker::infer_gender_real("Jacques", Some("FR"));      // Gender::Male
Faker::infer_gender_real("Mohammed", Some("EG"));     // Gender::Male

let f = Faker::seeded(42);
f.first_name_like_real("Jacques", Some("FR"));        // freq-weighted male FR name
f.first_name_real(Some("JP"), Gender::Female);        // weighted female JP name

Faker::detect_country("Yuki", 3);                     // [("JP", 0.58), ...]
Faker::homophones("Dominique", "FR", "ipa", 6, true, None);
// method: "metaphone" | "ipa" | "levenshtein" | "balanced"  (full parity with Python)
```

- Frequency-weighted: common names appear proportionally more often.
- Pulls in `parquet` + `arrow` (only under this feature).
- Dataset path overridable via `FAKER2_NAMES_PARQUET`.

### Benchmark

`cargo run --release --features real-names --example bench` (1M ops):

| Task | Rust (release) | Python |
|---|---:|---:|
| `infer_gender_real` | 12.79 M ops/s | 2.79 M ops/s |
| `first_name_like_real` | 3.79 M ops/s | 0.84 M ops/s |
| bank load (1.43M rows) | 2.3 s | 8.9 s |

~4–5× faster on identical logic. Whole-process footprint (load bank + 3M ops):
peak RAM ~0.43 GB vs Python ~1.27 GB (~3× less), CPU time 3.0s vs 45.5s
(~15× less). See top-level `README.md` for the full tables.

## Grammatical number agreement

```rust
use faker2::grammar;
grammar::pluralize("baby");        // "babies"
grammar::singularize("knives");    // "knife"
grammar::agree(1, "apple");        // "an apple"
grammar::agree(3, "dog");          // "3 dogs"
grammar::is_are(2);                // "are"
```

## Install

Prebuilt `faker2` CLI binaries for Linux (x86_64/aarch64, gnu + musl), macOS
(Intel + Apple Silicon) and Windows are attached to each
[GitHub Release](https://github.com/jqueguiner/faker2/releases) — download the
archive for your platform and extract the binary. Releases are built by
`.github/workflows/rust-release.yml` on every `v*` tag.

Or build from source (below).

## Build & test

```
cargo build
cargo test
```

## Fidelity notes

- Seeding is deterministic **within this crate** but is **not** byte-identical
  to Python's Mersenne-Twister sequence — same seed ≠ same string across langs.
- Name/last-name weighting (Python `OrderedDict` weights) is flattened to
  uniform choice; the value pool is the real faker list.
- Zero external dependencies (own SplitMix64 PRNG, std only).
