//! Algorithmic formatters for the `sbn` provider.
//!
//! SBN (Standard Book Number) is the precursor to the ISBN-10.
//! Reference: https://en.wikipedia.org/wiki/International_Standard_Book_Number
#![allow(unused_variables, clippy::all)]
use crate::faker::Faker;

// ── registrant rules (same as ISBN EAN-978, reg_group 0) ──────────────────────
// Each entry: (min_str, max_str, registrant_length)
// The rules operate on a 7-digit string (the first 7 digits of the 8-digit
// registrant+publication body).
const RULES: &[(&str, &str, usize)] = &[
    ("0000000", "1999999", 2),
    ("2000000", "2279999", 3),
    ("2280000", "2289999", 4),
    ("2290000", "6479999", 3),
    ("6480000", "6489999", 7),
    ("6490000", "6999999", 3),
    ("7000000", "8499999", 4),
    ("8500000", "8999999", 5),
    ("9000000", "9499999", 6),
    ("9500000", "9999999", 7),
];

/// Split an 8-digit reg_pub string into (registrant, publication) by looking
/// up the first 7 digits in the RULES table.
fn split_registrant_publication(reg_pub: &str) -> Option<(&str, &str)> {
    // key is the first 7 chars (all but the last digit of the 8-char body)
    let key = &reg_pub[..7];
    for &(min, max, reg_len) in RULES {
        if key >= min && key <= max {
            let (registrant, publication) = reg_pub.split_at(reg_len);
            return Some((registrant, publication));
        }
    }
    None
}

/// Compute the SBN-9 check digit.
/// Weights 1-8 applied to the 8-digit body; remainder mod 11;
/// 10 -> "X", else the digit as a string.
fn sbn9_check_digit(body: &str) -> char {
    let remainder: u32 = body
        .chars()
        .zip(1u32..=8)
        .map(|(c, w)| c.to_digit(10).unwrap_or(0) * w)
        .sum::<u32>()
        % 11;
    if remainder == 10 {
        'X'
    } else {
        char::from_digit(remainder, 10).unwrap_or('0')
    }
}

/// Generate an SBN-9 formatted as `registrant-publication-check`.
fn sbn9(f: &Faker, _locale: &str) -> String {
    // SBN body is 8 digits (MAX_LENGTH - 1 = 9 - 1 = 8)
    let reg_pub: String = (0..8)
        .map(|_| char::from_digit(f.rng.random_digit(), 10).unwrap_or('0'))
        .collect();

    let (registrant, publication) = match split_registrant_publication(&reg_pub) {
        Some(pair) => pair,
        // Fallback: use a known-valid split (2-digit registrant)
        None => reg_pub.split_at(2),
    };

    let body = format!("{}{}", registrant, publication);
    let check = sbn9_check_digit(&body);

    format!("{}-{}-{}", registrant, publication, check)
}

/// Return `Some(value)` for a formatter this module implements, else `None`.
pub fn dispatch(f: &Faker, locale: &str, name: &str) -> Option<String> {
    Some(match name {
        "sbn9" => sbn9(f, locale),
        _ => return None,
    })
}
