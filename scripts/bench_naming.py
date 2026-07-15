"""Python perf harness — mirror of rust/examples/bench.rs."""

import time

from faker2 import Faker
from faker2.naming import realnames as rn

f = Faker()
Faker.seed(42)

# 1) basic name generation (upstream faker)
n = 1_000_000
t = time.perf_counter()
sink = 0
for _ in range(n):
    sink += len(f.first_name())
d = time.perf_counter() - t
print(f"basic first_name(): {n} in {d:.3f}s = {n/d/1e6:.3f} M ops/s (sink {sink})")

# 2) real-names bank load
t = time.perf_counter()
rn.infer_gender("Jacques", "FR")  # triggers load+build
print(f"real bank load: {time.perf_counter()-t:.3f}s")

# 3) gender-preserving replacement
m = 1_000_000
t = time.perf_counter()
s2 = 0
for _ in range(m):
    s2 += len(rn.first_name_like("Jacques", "FR"))
d = time.perf_counter() - t
print(f"first_name_like: {m} in {d:.3f}s = {m/d/1e6:.3f} M ops/s (sink {s2})")

# 4) infer only
t = time.perf_counter()
s3 = 0
for _ in range(m):
    if rn.infer_gender("Jacques", "FR") == "m":
        s3 += 1
d = time.perf_counter() - t
print(f"infer_gender: {m} in {d:.3f}s = {m/d/1e6:.3f} M ops/s (sink {s3})")
