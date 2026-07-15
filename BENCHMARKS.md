# faker2 (Rust port) vs Faker (Python) — per-formatter benchmark

Throughput in ops/s, `en_US`, seed 1, 0.25 s time-boxed per formatter, same machine.
Python = original pure-Python `faker2`; Rust = this port (`Faker::gen`, release).
`—` = formatter not yet ported.

| Formatter | Python ops/s | Rust ops/s | Speedup |
|---|---:|---:|---:|
| `name` | 17 K | 535 K | 31.3× |
| `first_name` | 38 K | 5.37 M | 142.0× |
| `last_name` | 27 K | 5.35 M | 200.7× |
| `prefix` | 261 K | — | n/a |
| `suffix` | 211 K | — | n/a |
| `email` | 18 K | 1.22 M | 69.4× |
| `free_email` | 17 K | 2.15 M | 123.8× |
| `safe_email` | 18 K | 2.20 M | 125.2× |
| `user_name` | 18 K | 2.72 M | 154.3× |
| `domain_name` | 12 K | 2.24 M | 183.7× |
| `domain_word` | 12 K | 2.59 M | 211.5× |
| `url` | 11 K | 1.90 M | 165.2× |
| `ipv4` | 44 K | 4.90 M | 112.4× |
| `ipv6` | 245 K | 2.39 M | 9.7× |
| `mac_address` | 299 K | 3.02 M | 10.1× |
| `tld` | 1.27 M | 10.81 M | 8.5× |
| `http_method` | 1.27 M | 8.66 M | 6.8× |
| `uri` | 10 K | 1.38 M | 143.4× |
| `slug` | 44 K | 7.44 M | 168.4× |
| `address` | 12 K | 160 K | 13.1× |
| `street_address` | 21 K | 397 K | 18.5× |
| `street_name` | 28 K | 2.05 M | 73.9× |
| `building_number` | 247 K | 3.51 M | 14.2× |
| `city` | 30 K | 1.06 M | 35.6× |
| `postcode` | 1.58 M | 2.42 M | 1.5× |
| `country` | 1.29 M | 5.36 M | 4.1× |
| `country_code` | 806 K | 5.69 M | 7.1× |
| `company` | 13 K | 1.92 M | 152.3× |
| `company_suffix` | 1.25 M | 5.41 M | 4.3× |
| `catch_phrase` | 456 K | 8.30 M | 18.2× |
| `bs` | 453 K | 8.19 M | 18.1× |
| `job` | 1.19 M | 5.52 M | 4.7× |
| `phone_number` | 115 K | 2.02 M | 17.5× |
| `msisdn` | 116 K | 2.10 M | 18.1× |
| `credit_card_number` | 84 K | 2.94 M | 35.0× |
| `credit_card_provider` | 848 K | 10.52 M | 12.4× |
| `credit_card_expire` | 115 K | 5.03 M | 43.7× |
| `credit_card_security_code` | 254 K | 9.51 M | 37.5× |
| `iban` | 68 K | 2.08 M | 30.5× |
| `bban` | 80 K | 7.05 M | 88.0× |
| `swift` | 118 K | 1.69 M | 14.4× |
| `aba` | 172 K | 4.21 M | 24.4× |
| `ean8` | 221 K | 7.66 M | 34.6× |
| `ean13` | 142 K | 6.79 M | 47.7× |
| `currency_code` | 1.20 M | 10.98 M | 9.2× |
| `currency_name` | 1.18 M | 10.29 M | 8.7× |
| `cryptocurrency_code` | 1.18 M | 10.91 M | 9.2× |
| `color_name` | 371 K | 10.08 M | 27.2× |
| `safe_color_name` | 1.31 M | 10.72 M | 8.2× |
| `hex_color` | 1.40 M | 7.98 M | 5.7× |
| `rgb_color` | 494 K | 5.87 M | 11.9× |
| `uuid4` | 533 K | 1.22 M | 2.3× |
| `md5` | 843 K | 1.50 M | 1.8× |
| `sha1` | 879 K | 1.23 M | 1.4× |
| `sha256` | 867 K | 853 K | 1.0× |
| `password` | 180 K | 3.95 M | 21.9× |
| `pyint` | 1.69 M | 9.43 M | 5.6× |
| `pyfloat` | 241 K | 5.33 M | 22.1× |
| `pystr` | 426 K | 6.51 M | 15.3× |
| `license_plate` | 149 K | 285 K | 1.9× |
| `vin` | 52 K | 3.88 M | 74.9× |
| `file_name` | 136 K | 6.56 M | 48.2× |
| `file_extension` | 586 K | 9.75 M | 16.6× |
| `mime_type` | 556 K | 9.41 M | 16.9× |
| `ssn` | 525 K | 2.43 M | 4.6× |
| `word` | 183 K | — | n/a |

**63/66** formatters ported; **62** are faster in Rust. Geo-mean speedup: **~20.5×** (range ~1.5× on already-fast formatters to ~140× on slow ones).

## Hardware — 500k `name()` calls (`/usr/bin/time -v`)

| Metric | Python | Rust |
|---|---:|---:|
| CPU + wall time | 29 s | **2 s** (~15× faster) |
| Peak RAM (RSS) | 26 MB | 120 MB |

Rust holds the full **151-locale** dataset resident (~120 MB, constant) for
instant locale switching; Python lazy-loads one locale. Multi-locale bulk work:
Rust stays flat, Python grows per locale.

Reproduce Rust: `cd rust && cargo run --release --features locales --example benchall`.
