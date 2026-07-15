//! Algorithmic formatters for the `isbn` provider.
//!
//! Implements ISBN-10 and ISBN-13 generation with proper check digits.
//! Rules data is hardcoded from faker2/providers/isbn/en_US/__init__.py and
//! faker2/providers/isbn/es_ES/__init__.py.
#![allow(unused_variables, clippy::all)]
use crate::faker::Faker;

/// A single registrant rule: (min, max, registrant_length).
/// min/max are 7-character strings compared lexicographically.
type Rule = (&'static str, &'static str, usize);

/// Hardcoded rules table: (ean, group, rules[])
/// Sourced from en_US and es_ES locale providers.
const RULES: &[(&str, &str, &[Rule])] = &[
    // en_US / group "0"
    (
        "978",
        "0",
        &[
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
        ],
    ),
    // en_US / group "1"
    (
        "978",
        "1",
        &[
            ("0000000", "0999999", 2),
            ("1000000", "3999999", 3),
            ("4000000", "5499999", 4),
            ("5500000", "7319999", 5),
            ("7320000", "7399999", 7),
            ("7400000", "8697999", 5),
            ("8698000", "9729999", 6),
            ("9730000", "9877999", 4),
            ("9878000", "9989999", 6),
            ("9990000", "9999999", 7),
        ],
    ),
    // es_ES / group "84"
    (
        "978",
        "84",
        &[
            ("0000000", "0999999", 2),
            ("1000000", "1049999", 5),
            ("1050000", "1199999", 4),
            ("1200000", "1299999", 6),
            ("1300000", "1399999", 4),
            ("1400000", "1499999", 3),
            ("1500000", "1999999", 5),
            ("2000000", "6999999", 3),
            ("7000000", "8499999", 4),
            ("8500000", "8999999", 5),
            ("9000000", "9199999", 4),
            ("9200000", "9239999", 6),
            ("9240000", "9299999", 5),
            ("9300000", "9499999", 6),
            ("9500000", "9699999", 5),
            ("9700000", "9999999", 4),
        ],
    ),
    // es_ES / group "13"
    (
        "978",
        "13",
        &[
            ("0000000", "0099999", 2),
            // rules with length 0 are undefined ranges; we skip them by
            // treating 0 as "use full remaining length" — but in practice
            // _registrant_publication will always find a matching rule ≠ 0.
            ("0100000", "5999999", 0),
            ("6000000", "6049999", 3),
            ("6050000", "6999999", 0),
            ("7000000", "7349999", 4),
            ("7350000", "8749999", 0),
            ("8750000", "8999999", 5),
            ("9000000", "9899999", 0),
            ("9900000", "9999999", 6),
        ],
    ),
];

/// Separate registrant from publication using the rule list.
/// Returns `None` if no rule matches or the matched rule has length 0 (undefined).
fn registrant_publication(reg_pub: &str, rules: &[Rule]) -> Option<(String, String)> {
    // Compare against reg_pub[..len-1] (all but the last char), mirroring
    // the Python `rule[0] <= reg_pub[:-1] <= rule[1]` slice.
    let key = &reg_pub[..reg_pub.len().saturating_sub(1)];
    for &(min, max, reg_len) in rules {
        if key >= min && key <= max {
            if reg_len == 0 {
                return None; // undefined range
            }
            let registrant = reg_pub[..reg_len].to_string();
            let publication = reg_pub[reg_len..].to_string();
            return Some((registrant, publication));
        }
    }
    None
}

/// Calculate the ISBN-13 check digit (EAN-13 algorithm).
fn isbn13_check_digit(ean: &str, group: &str, registrant: &str, publication: &str) -> char {
    let body = format!("{}{}{}{}", ean, group, registrant, publication);
    let sum: u32 = body
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let d = c.to_digit(10).unwrap_or(0);
            let w = if i % 2 == 0 { 1 } else { 3 };
            d * w
        })
        .sum();
    let remainder = sum % 10;
    let diff = (10 - remainder) % 10;
    char::from_digit(diff, 10).unwrap_or('0')
}

/// Calculate the ISBN-10 check digit.
fn isbn10_check_digit(group: &str, registrant: &str, publication: &str) -> char {
    let body = format!("{}{}{}", group, registrant, publication);
    // weights are 1..=9 for 9 digits
    let sum: u32 = body
        .chars()
        .zip(1u32..)
        .map(|(c, w)| c.to_digit(10).unwrap_or(0) * w)
        .sum();
    let remainder = sum % 11;
    if remainder == 10 {
        'X'
    } else {
        char::from_digit(remainder, 10).unwrap_or('0')
    }
}

/// Pick a random (ean, group, rules) entry, generate reg_pub, and split it.
/// Returns (ean, group, registrant, publication) or None if no valid split found.
fn generate_body(f: &Faker) -> Option<(String, String, String, String)> {
    // Try a bounded number of times to avoid infinite loops on 0-length rules
    for _ in 0..50 {
        let idx = f.rng.below(RULES.len());
        let (ean, group, rules) = RULES[idx];

        // MAX_LENGTH = 13; reg_pub_len = 13 - len(ean) - len(group) - 1
        let reg_pub_len = 13 - ean.len() - group.len() - 1;
        let pattern = "#".repeat(reg_pub_len);
        let reg_pub = f.rng.numerify(&pattern);

        if let Some((registrant, publication)) = registrant_publication(&reg_pub, rules) {
            return Some((ean.to_string(), group.to_string(), registrant, publication));
        }
    }
    None
}

fn isbn13(f: &Faker, _locale: &str) -> String {
    let separator = "-";
    if let Some((ean, group, registrant, publication)) = generate_body(f) {
        let check = isbn13_check_digit(&ean, &group, &registrant, &publication);
        format!(
            "{}{}{}{}{}{}{}{}{}",
            ean, separator, group, separator, registrant, separator, publication, separator, check
        )
    } else {
        // Fallback: minimal valid ISBN-13 body
        let body = f.rng.numerify("978#########");
        // Compute check digit over first 12 chars
        let sum: u32 = body
            .chars()
            .take(12)
            .enumerate()
            .map(|(i, c)| {
                let d = c.to_digit(10).unwrap_or(0);
                let w = if i % 2 == 0 { 1 } else { 3 };
                d * w
            })
            .sum();
        let check = char::from_digit((10 - sum % 10) % 10, 10).unwrap_or('0');
        format!("{}{}", &body[..12], check)
    }
}

fn isbn10(f: &Faker, _locale: &str) -> String {
    let separator = "-";
    if let Some((_ean, group, registrant, publication)) = generate_body(f) {
        let check = isbn10_check_digit(&group, &registrant, &publication);
        format!(
            "{}{}{}{}{}{}",
            group, separator, registrant, separator, publication, check
        )
    } else {
        // Fallback
        let body = f.rng.numerify("#########");
        let sum: u32 = body
            .chars()
            .zip(1u32..)
            .map(|(c, w)| c.to_digit(10).unwrap_or(0) * w)
            .sum();
        let remainder = sum % 11;
        let check = if remainder == 10 {
            'X'
        } else {
            char::from_digit(remainder, 10).unwrap_or('0')
        };
        format!("{}{}", body, check)
    }
}

/// Return `Some(value)` for a formatter this module implements, else `None`.
pub fn dispatch(f: &Faker, locale: &str, name: &str) -> Option<String> {
    Some(match name {
        "isbn13" => isbn13(f, locale),
        "isbn10" => isbn10(f, locale),
        _ => return None,
    })
}
