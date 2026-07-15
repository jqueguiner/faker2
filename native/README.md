# faker2-rs

A thin **Python wrapper over the Rust `faker2` engine** (PyO3). All fake-data
generation runs in Rust; Python only exposes the API.

```python
from faker2_rs import Faker, homophones, detect_country, infer_gender_real

f = Faker(42)                              # seedable, deterministic
f.name(); f.email(); f.address()
f.first_name_like_real("Jacques", "FR")    # gender-preserving, weighted
f.first_name_real("JP", "f")               # frequency-weighted

infer_gender_real("Mohammed", "EG")        # "m"
detect_country("Yuki", 3)                  # [("JP", 0.58), ...]
homophones("Dominique", "FR", "balanced")  # [("Dominique", 0.95), ...]
```

## Build

```bash
cd native
maturin build --release        # produces an abi3 wheel
pip install dist/*.whl
```

## Scope

Wraps the Rust port (`../rust`): core engine + en_US providers + the 139-country
name intelligence. More locales are ported onto the same engine over time — see
the repo `README.md` and `PARITY.md`.

Note: the dataset (`data/first_names.parquet`) must be reachable at runtime; set
`FAKER2_NAMES_PARQUET` to point at it if not co-located.
```
