//! Algorithmic formatters for the `python` provider.
//!
//! Mirrors the ALGORITHMIC methods from `faker2.providers.python.Provider`:
//!   - `pyint`        – random integer 0–9999 (as string)
//!   - `pybool`       – "True" or "False" (50/50)
//!   - `pystr`        – 20 random ASCII letters
//!   - `pystr_format` – bothify of the default format pattern
//!   - `pyfloat`      – random float string (up to 15 significant digits)
//!   - `pydecimal`    – random decimal string (sign · integer part · fraction)
#![allow(unused_variables, clippy::all)]
use crate::faker::Faker;

const ASCII_LETTERS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

/// 20 random ASCII letters, like `pystr(max_chars=20)`.
fn pystr(f: &Faker, _locale: &str) -> String {
    (0..20)
        .map(|_| ASCII_LETTERS[f.rng.below(ASCII_LETTERS.len())] as char)
        .collect()
}

/// Random letter-and-digit pattern: `?#-###<random_int><random_letter>`.
/// Mirrors the default `string_format` in `pystr_format`.
fn pystr_format(f: &Faker, _locale: &str) -> String {
    // default: "?#-###{{random_int}}{{random_letter}}"
    let ri = f.rng.random_int(0, 9999, 1);
    let rl = ASCII_LETTERS[f.rng.below(ASCII_LETTERS.len())] as char;
    let base = format!("?#-###{ri}{rl}");
    f.rng.bothify(&base)
}

/// "True" or "False" with equal probability (truth_probability=50 default).
fn pybool(f: &Faker, _locale: &str) -> String {
    if f.rng.random_int(1, 100, 1) <= 50 {
        "True".to_string()
    } else {
        "False".to_string()
    }
}

/// Random integer in [0, 9999], formatted as a string.
fn pyint(f: &Faker, _locale: &str) -> String {
    f.rng.random_int(0, 9999, 1).to_string()
}

/// Random float with 1-6 decimal places, possibly negative.
/// Mirrors `pyfloat()` with default (no) constraints.
fn pyfloat(f: &Faker, _locale: &str) -> String {
    // left_digits chosen from 1..=10, right_digits from 1..=6
    let left_digits = f.rng.random_int(1, 10, 1) as u32;
    let right_digits = f.rng.random_int(1, 6, 1) as u32;
    let sign = if f.rng.below(2) == 0 { "" } else { "-" };
    let left_max = 10i64.pow(left_digits) - 1;
    let left_num = f.rng.random_int(0, left_max, 1);
    let right_num = f.rng.random_number(right_digits, false);
    format!(
        "{sign}{left_num}.{right_num:0>width$}",
        width = right_digits as usize
    )
}

/// Random decimal string with sign, integer part, and fractional part.
/// Mirrors `pydecimal()` with default (no) constraints; keeps scale short.
fn pydecimal(f: &Faker, _locale: &str) -> String {
    let left_digits = f.rng.random_int(1, 10, 1) as u32;
    let right_digits = f.rng.random_int(0, 20, 1) as u32;
    let sign = if f.rng.below(2) == 0 { "" } else { "-" };
    let left_max = 10i64.pow(left_digits) - 1;
    let left_num = f.rng.random_int(0, left_max, 1);
    let right_num = f.rng.random_number(right_digits, false);
    if right_digits == 0 {
        format!("{sign}{left_num}")
    } else {
        format!(
            "{sign}{left_num}.{right_num:0>width$}",
            width = right_digits as usize
        )
    }
}

/// Return `Some(value)` for a formatter this module implements, else `None`.
pub fn dispatch(f: &Faker, locale: &str, name: &str) -> Option<String> {
    Some(match name {
        "pystr" => pystr(f, locale),
        "pystr_format" => pystr_format(f, locale),
        "pybool" => pybool(f, locale),
        "pyint" => pyint(f, locale),
        "pyfloat" => pyfloat(f, locale),
        "pydecimal" => pydecimal(f, locale),
        "pytimezone" => pytimezone(f, locale),
        _ => return None,
    })
}

#[rustfmt::skip]
const TZ: &[&str] = &[
    "UTC","America/New_York","America/Chicago","America/Los_Angeles","America/Sao_Paulo",
    "Europe/London","Europe/Paris","Europe/Berlin","Europe/Madrid","Europe/Moscow",
    "Africa/Cairo","Africa/Lagos","Asia/Tokyo","Asia/Shanghai","Asia/Kolkata",
    "Asia/Dubai","Asia/Singapore","Australia/Sydney","Pacific/Auckland","America/Mexico_City",
];

/// A random IANA timezone name (Python returns a ZoneInfo; we return its key).
fn pytimezone(f: &Faker, _locale: &str) -> String {
    TZ[f.rng.below(TZ.len())].to_string()
}
