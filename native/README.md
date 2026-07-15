# faker2 (pure Rust + Python binder)

The `faker2` PyPI package as a **pure-Rust engine with a thin Python binder**
(PyO3). `import faker2` loads the compiled Rust core (`faker2._native`); no
fake-data logic runs in Python.

```python
from faker2 import Faker, homophones, detect_country, infer_gender_real

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
maturin build --release        # abi3 wheel (bundles the dataset)
pip install dist/*.whl
```

## Scope

The Rust core (`../rust`) currently implements the engine + en_US providers +
the 139-country name intelligence. More locales are ported into the Rust core
over time; see the repo `README.md` and `PARITY.md`.
