# Python ↔ Rust feature parity

The name-intelligence feature set is implemented in both languages. This is the
authoritative parity matrix.

## Real name ground truth (`faker2.naming.realnames` / `real-names` feature)

| Capability | Python | Rust |
|---|---|---|
| Infer gender from real data | `realnames.infer_gender(name, country)` | `Faker::infer_gender_real(name, country)` |
| Frequency-weighted first name | `realnames.first_name(country, gender)` | `Faker::first_name_real(country, gender)` |
| Gender-preserving replacement | `realnames.first_name_like(name, country)` | `Faker::first_name_like_real(name, country)` |
| Country detection | `realnames.detect_country(name, top)` | `Faker::detect_country(name, top)` |
| Homophones (metaphone/ipa/levenshtein/balanced) | `realnames.homophones(...)` | `Faker::homophones(...)` |
| Per-country balanced weights | `data/balanced_params.json` | same file (via `serde_json`) |
| List countries | `realnames.available_countries()` | `Faker::available_countries()` |

## Bundled-locale gender + grammar (`faker2.naming.gender` / `gender.rs`)

| Capability | Python | Rust |
|---|---|---|
| Infer gender (bundled lists) | `gender.infer_gender(name, locale)` | `Faker::infer_gender(name, Locale)` |
| Gender-preserving replacement | `gender.first_name_like(name, locale)` | `Faker::first_name_like(name, Locale)` |
| Gendered full name | `gender.full_name(locale, gender)` | `Faker::name_of(gender, Locale)` |
| pluralize / singularize | `grammar.pluralize` / `singularize` | `grammar::pluralize` / `singularize` |
| indefinite_article / agree / is_are | `grammar.*` | `grammar::*` |

### Known asymmetries (intentional)

- **Bundled-locale breadth**: Python `gender.*` works over every faker locale
  (`Faker(locale)`), Rust `gender.rs` ships only `Locale::{EnUS, FrFR}` embedded
  lists. For broad coverage use the **realnames** path in both — it spans the
  same 139 countries either side.
- **Gender code case**: `realnames` uses lowercase `"m"/"f"`; the bundled
  `gender` module uses `"M"/"F"`. `full_name` accepts either.
- **Seeding**: both are deterministic within a language but not byte-identical
  across languages (different PRNGs).
