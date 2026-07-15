# Name ground-truth data

Real first/last name data powering the gender-aware, country-aware faker
providers (`faker2.realnames` in Python, the `real-names` feature in Rust).

## Files

| File | Rows | Notes |
|------|------|-------|
| `first_names.parquet` | 1,433,891 | 139 countries, gender (`m`/`f`), relative `frequency`, `country_rank`, `continent` |
| `last_names.parquet` | 30 | small sample only — surnames are **not** comprehensive |

Format: Parquet, zstd-compressed, dictionary-encoded low-cardinality columns.
`first_names.parquet` is 11.5 MB and loads fully in ~0.1 s.

## Schema (`first_names`)

`name, name_ascii, country_code, country_name, continent, gender, unisex, country_rank, frequency, country_share, phonetic`

- `frequency` — relative share within `(country_code, gender)` (weighted sampling).
- `country_share` — relative share within `country_code` (cross-gender comparable; powers `detect_country` / `homophones` probabilities).
- `phonetic` — double-metaphone key (powers `homophones`).

`frequency` is a **relative share** (0..1) within each `(country_code, gender)`
group — it preserves the weighting used for sampling but deliberately carries
**no raw population counts**. Absolute headcounts are not exposed.

## Provenance

Extracted from a PostgreSQL 17 custom dump (`pg_dump -Fc`, 3.57 GB) restored
locally with PG 18 client + `pgvector`. Source table also carried phonetic
columns (`ipa`, `metaphone`, `double_metaphone`, `soundex`, `panphon_vector`)
dropped here as irrelevant to generation.

## Regenerating

```sql
-- frequency normalized to a relative share; raw counts never leave the DB
\copy (SELECT name, name_ascii, country_code, country_name, continent, gender,
              unisex, country_rank,
              (frequency::float8 / NULLIF(SUM(frequency)
                 OVER (PARTITION BY country_code, gender),0))::real AS frequency
       FROM first_names) TO 'first_names.csv' WITH (FORMAT csv, HEADER true)
```
then convert CSV -> parquet (pyarrow, zstd, dictionary-encode
`country_code/country_name/continent/gender`).
