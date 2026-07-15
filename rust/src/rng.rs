//! Seedable PRNG + the `BaseProvider` primitives.
//!
//! Mirrors `faker2.generator.Generator` (per-instance seedable random) and the
//! low-level helpers on `faker2.providers.BaseProvider`
//! (`random_int`, `random_element`, `numerify`, `lexify`, `bothify`, `hexify`).
//!
//! NOTE: seeding is reproducible within this Rust implementation but is NOT
//! byte-identical to the Python (Mersenne-Twister) version.

use std::cell::Cell;

const ASCII_LETTERS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const HEX_LOWER: &[u8] = b"0123456789abcdef";
const HEX_UPPER: &[u8] = b"0123456789ABCDEF";

/// SplitMix64 — small, fast, fully deterministic given a seed.
pub struct Rng {
    state: Cell<u64>,
}

impl Rng {
    /// Unseeded generator (seeded from wall-clock, like `random.seed(None)`).
    pub fn new() -> Self {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0x9E3779B97F4A7C15);
        Rng {
            state: Cell::new(nanos ^ 0xD1B54A32D192ED03),
        }
    }

    /// Deterministic generator — equivalent to `Faker.seed_instance(seed)`.
    pub fn seeded(seed: u64) -> Self {
        Rng {
            state: Cell::new(seed.wrapping_add(0x9E3779B97F4A7C15)),
        }
    }

    /// Re-seed in place — equivalent to `generator.seed_instance()`.
    pub fn seed(&self, seed: u64) {
        self.state.set(seed.wrapping_add(0x9E3779B97F4A7C15));
    }

    #[inline]
    fn next_u64(&self) -> u64 {
        let mut z = self.state.get().wrapping_add(0x9E3779B97F4A7C15);
        self.state.set(z);
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }

    /// Uniform in `[0, n)`. `n == 0` returns 0.
    #[inline]
    pub fn below(&self, n: usize) -> usize {
        if n == 0 {
            return 0;
        }
        (self.next_u64() % n as u64) as usize
    }

    /// Uniform float in `[0.0, 1.0)`.
    #[inline]
    pub fn unit(&self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }

    // ---- BaseProvider primitives -------------------------------------------

    /// Inclusive `[min, max]` with `step`. Mirrors `random_int`.
    pub fn random_int(&self, min: i64, max: i64, step: i64) -> i64 {
        if step <= 0 || max < min {
            return min;
        }
        let n = ((max - min) / step) + 1;
        min + self.below(n as usize) as i64 * step
    }

    pub fn random_digit(&self) -> u32 {
        self.below(10) as u32
    }

    pub fn random_digit_not_null(&self) -> u32 {
        1 + self.below(9) as u32
    }

    pub fn random_digit_above_two(&self) -> u32 {
        2 + self.below(8) as u32
    }

    /// Random number with `digits` length; if `fix_len`, no leading-zero collapse.
    pub fn random_number(&self, digits: u32, fix_len: bool) -> u64 {
        if digits == 0 {
            return 0;
        }
        if fix_len {
            let min = 10u64.pow(digits - 1);
            let max = 10u64.pow(digits) - 1;
            self.random_int(min as i64, max as i64, 1) as u64
        } else {
            let max = 10u64.pow(digits) - 1;
            self.random_int(0, max as i64, 1) as u64
        }
    }

    /// Pick one element (mirrors `random_element`).
    pub fn choice<'a, T>(&self, items: &'a [T]) -> &'a T {
        &items[self.below(items.len())]
    }

    pub fn choice_str<'a>(&self, items: &'a [&'a str]) -> &'a str {
        items[self.below(items.len())]
    }

    pub fn random_letter(&self) -> char {
        ASCII_LETTERS[self.below(ASCII_LETTERS.len())] as char
    }

    pub fn random_lowercase_letter(&self) -> char {
        LOWER[self.below(LOWER.len())] as char
    }

    /// `#` -> digit, `%` -> 1-9, `$` -> 2-9. Mirrors `numerify` (subset).
    pub fn numerify(&self, text: &str) -> String {
        let mut out = String::with_capacity(text.len());
        for c in text.chars() {
            match c {
                '#' => out.push_str(&self.random_digit().to_string()),
                '%' => out.push_str(&self.random_digit_not_null().to_string()),
                '$' => out.push_str(&self.random_digit_above_two().to_string()),
                '!' => {
                    if self.below(2) == 1 {
                        out.push_str(&self.random_digit().to_string())
                    }
                }
                '@' => {
                    if self.below(2) == 1 {
                        out.push_str(&self.random_digit_not_null().to_string())
                    }
                }
                _ => out.push(c),
            }
        }
        out
    }

    /// `?` -> random letter from `letters`. Mirrors `lexify`.
    pub fn lexify(&self, text: &str, letters: &[u8]) -> String {
        let mut out = String::with_capacity(text.len());
        for c in text.chars() {
            if c == '?' {
                out.push(letters[self.below(letters.len())] as char);
            } else {
                out.push(c);
            }
        }
        out
    }

    /// Combine `numerify` + `lexify`. Mirrors `bothify`.
    pub fn bothify(&self, text: &str) -> String {
        self.lexify(&self.numerify(text), ASCII_LETTERS)
    }

    /// `^` -> hex digit. Mirrors `hexify`.
    pub fn hexify(&self, text: &str, upper: bool) -> String {
        let set = if upper { HEX_UPPER } else { HEX_LOWER };
        let mut out = String::with_capacity(text.len());
        for c in text.chars() {
            if c == '^' {
                out.push(set[self.below(set.len())] as char);
            } else {
                out.push(c);
            }
        }
        out
    }
}

impl Default for Rng {
    fn default() -> Self {
        Rng::new()
    }
}
