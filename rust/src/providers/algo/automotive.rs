//! Algorithmic formatters for the `automotive` provider.
//!
//! Ports `faker2.providers.automotive.Provider` to Rust.
//! Ported formatters: `license_plate`, `vin`.
#![allow(unused_variables, clippy::all)]
use crate::faker::Faker;

// ---------------------------------------------------------------------------
// VIN helpers
// ---------------------------------------------------------------------------

/// Per the Python source: A=1..I=9, J=1..R=9, S=2..Z=9, digits => themselves.
fn vin_char_weight(c: char) -> i64 {
    let o = c as u32;
    if o <= 64 {
        // '0'-'9' (ASCII 48-57)
        (o - 48) as i64
    } else if o <= 73 {
        // A-I
        (o - 64) as i64
    } else if o <= 82 {
        // J-R
        (o - 73) as i64
    } else {
        // S-Z
        (o - 81) as i64
    }
}

fn vin_str_weight(s: &str, weights: &[i64]) -> i64 {
    s.chars()
        .zip(weights.iter())
        .map(|(c, &w)| vin_char_weight(c) * w)
        .sum()
}

/// Generate a VIN (Vehicle Identification Number) with a valid check digit.
///
/// Mirrors `faker2.providers.automotive.Provider.vin`.
fn vin(f: &Faker, _locale: &str) -> String {
    // Restricted charset: no I, O, Q
    const VIN_CHARS: &[u8] = b"1234567890ABCDEFGHJKLMNPRSTUVWXYZ";

    // front 8 chars: all from VIN_CHARS (bothify "????????")
    let mut front = String::with_capacity(8);
    for _ in 0..8 {
        front.push(VIN_CHARS[f.rng.below(VIN_CHARS.len())] as char);
    }

    // rear 8 chars: 4 letters + 4 digits ("????####")
    let mut rear = String::with_capacity(8);
    for _ in 0..4 {
        rear.push(VIN_CHARS[f.rng.below(VIN_CHARS.len())] as char);
    }
    for _ in 0..4 {
        rear.push((b'0' + f.rng.random_digit() as u8) as char);
    }

    let front_weight = vin_str_weight(&front, &[8, 7, 6, 5, 4, 3, 2, 10]);
    let rear_weight = vin_str_weight(&rear, &[9, 8, 7, 6, 5, 4, 3, 2]);
    let checksum = (front_weight + rear_weight) % 11;
    let check_char = if checksum == 10 {
        'X'
    } else {
        (b'0' + checksum as u8) as char
    };

    format!("{front}{check_char}{rear}")
}

/// Generate a license plate for the given locale.
///
/// Mirrors `faker2.providers.automotive.Provider.license_plate`.
/// Picks a random format from `license_formats`, replaces `?` with a random
/// uppercase ASCII letter, then numerifies `#` placeholders.
fn license_plate(f: &Faker, locale: &str) -> String {
    let formats = f.lfield(locale, "automotive", "license_formats");
    if formats.is_empty() {
        // Fallback: generic plate
        return f.rng.bothify("??-####");
    }
    let tmpl = formats[f.rng.below(formats.len())].clone();

    // Replace '?' with random uppercase letter (mirrors Python's re.sub + random_element(ascii_uppercase))
    const UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut with_letters = String::with_capacity(tmpl.len());
    for c in tmpl.chars() {
        if c == '?' {
            with_letters.push(UPPER[f.rng.below(UPPER.len())] as char);
        } else {
            with_letters.push(c);
        }
    }

    // Then numerify '#' -> digit
    f.rng.numerify(&with_letters)
}

// ---------------------------------------------------------------------------
// Dispatch
// ---------------------------------------------------------------------------

/// Return `Some(value)` for a formatter this module implements, else `None`.
pub fn dispatch(f: &Faker, locale: &str, name: &str) -> Option<String> {
    Some(match name {
        "license_plate" => license_plate(f, locale),
        "vin" => vin(f, locale),
        _ => return None,
    })
}
