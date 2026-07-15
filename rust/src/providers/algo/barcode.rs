//! Algorithmic formatters for the `barcode` provider.
//!
//! Ports:
//!  - `ean` / `ean8` / `ean13`          — generic EAN-8 / EAN-13 barcodes
//!  - `localized_ean` / `localized_ean8` / `localized_ean13`
//!  - `upc_a` / `upc_e`                 — en_US / en_CA UPC barcodes
//!  - `jan` / `jan8` / `jan13`          — ja_JP JAN codes (localized EAN aliases)
#![allow(unused_variables, clippy::all)]
use crate::faker::Faker;

// ---- locale-specific prefix tables -----------------------------------------

/// en_US local_prefixes: product((0,), 0..10) + product((1,), 0..4)
/// Each entry is a 2-digit tuple (first_digit, second_digit).
const EN_US_PREFIXES: &[(u8, u8)] = &[
    (0, 0),
    (0, 1),
    (0, 2),
    (0, 3),
    (0, 4),
    (0, 5),
    (0, 6),
    (0, 7),
    (0, 8),
    (0, 9),
    (1, 0),
    (1, 1),
    (1, 2),
    (1, 3),
];

/// en_CA local_prefixes: product((0,), 0..2) + product((0,), 6..10) + (7,5)
const EN_CA_PREFIXES: &[(u8, u8)] = &[(0, 0), (0, 1), (0, 6), (0, 7), (0, 8), (0, 9), (7, 5)];

/// ja_JP local_prefixes: (4,5) and (4,9)
const JA_JP_PREFIXES: &[(u8, u8)] = &[(4, 5), (4, 9)];

/// es_ES local_prefixes: (8,4)
const ES_ES_PREFIXES: &[(u8, u8)] = &[(8, 4)];

// ---- EAN checksum logic ----------------------------------------------------

/// Generate an EAN-8 or EAN-13 barcode.
/// `prefix` is an optional 2-tuple `(digit0, digit1)` to force the start.
fn ean_inner(f: &Faker, length: usize, prefix: Option<(u8, u8)>) -> String {
    // Build code digits (without check digit)
    let mut code: Vec<u8> = (0..length - 1)
        .map(|_| f.rng.random_digit() as u8)
        .collect();

    // Overwrite prefix digits if supplied
    if let Some((d0, d1)) = prefix {
        code[0] = d0;
        code[1] = d1;
    }

    let weights: &[u8] = if length == 8 {
        &[3, 1, 3, 1, 3, 1, 3]
    } else {
        &[1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3]
    };

    let weighted_sum: u32 = code
        .iter()
        .zip(weights.iter())
        .map(|(&d, &w)| d as u32 * w as u32)
        .sum();
    let check_digit = ((10 - weighted_sum % 10) % 10) as u8;
    code.push(check_digit);

    code.iter().map(|d| (b'0' + d) as char).collect()
}

/// Pick a random prefix tuple from a slice.
fn pick_prefix(f: &Faker, prefixes: &[(u8, u8)]) -> Option<(u8, u8)> {
    if prefixes.is_empty() {
        None
    } else {
        Some(prefixes[f.rng.below(prefixes.len())])
    }
}

// ---- Generic EAN -----------------------------------------------------------

fn ean(f: &Faker, _locale: &str) -> String {
    ean_inner(f, 13, None)
}

fn ean8(f: &Faker, _locale: &str) -> String {
    ean_inner(f, 8, None)
}

fn ean13(f: &Faker, _locale: &str) -> String {
    ean_inner(f, 13, None)
}

// ---- Localized EAN ---------------------------------------------------------

fn locale_prefixes(locale: &str) -> &'static [(u8, u8)] {
    match locale {
        "en_US" => EN_US_PREFIXES,
        "en_CA" | "fr_CA" => EN_CA_PREFIXES,
        "ja_JP" => JA_JP_PREFIXES,
        "es_ES" => ES_ES_PREFIXES,
        _ => &[],
    }
}

fn localized_ean(f: &Faker, locale: &str) -> String {
    let prefix = pick_prefix(f, locale_prefixes(locale));
    ean_inner(f, 13, prefix)
}

fn localized_ean8(f: &Faker, locale: &str) -> String {
    let prefix = pick_prefix(f, locale_prefixes(locale));
    ean_inner(f, 8, prefix)
}

fn localized_ean13(f: &Faker, locale: &str) -> String {
    let prefix = pick_prefix(f, locale_prefixes(locale));
    ean_inner(f, 13, prefix)
}

// ---- UPC-A / UPC-E (en_US, en_CA) -----------------------------------------

/// Build a 12-digit UPC-A code that is guaranteed to have a UPC-E equivalent.
/// `base` is 6 digits; the last digit of base controls the encoding pattern.
fn upc_ae_inner(f: &Faker, base: [u8; 6], number_system_digit: u8) -> String {
    let b = base;
    // Expand base -> 11 digits using UPC-E suppression rules
    let code11: [u8; 11] = if b[5] <= 2 {
        // pattern 1: mfr=b[0..2] + extra=b[5] + 0000 + product=b[2..5]
        [
            number_system_digit,
            b[0],
            b[1],
            b[5],
            0,
            0,
            0,
            0,
            b[2],
            b[3],
            b[4],
        ]
    } else if b[5] <= 4 {
        // pattern 2: mfr=b[0..b5] + 00000 + product=b[b5..5]
        // b[5] ∈ {3,4}: mfr is b5 digits
        let extra = b[5] as usize;
        let mut c = [0u8; 11];
        c[0] = number_system_digit;
        for i in 0..extra {
            c[1 + i] = b[i];
        }
        // 00000 occupies positions 1+extra .. 1+extra+5
        // product = b[extra..5]
        for i in 0..(5 - extra) {
            c[1 + extra + 5 + i] = b[extra + i];
        }
        c
    } else {
        // pattern 3: mfr=b[0..5] + 0000 + extra=b[5]
        [
            number_system_digit,
            b[0],
            b[1],
            b[2],
            b[3],
            b[4],
            0,
            0,
            0,
            0,
            b[5],
        ]
    };

    // Compute check digit using weights [3,1,3,1,3,1,3,1,3,1,3]
    let weights = [3u32, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3];
    let weighted_sum: u32 = code11
        .iter()
        .zip(weights.iter())
        .map(|(&d, &w)| d as u32 * w)
        .sum();
    let check_digit = ((10 - weighted_sum % 10) % 10) as u8;

    let mut result = String::with_capacity(12);
    for &d in &code11 {
        result.push((b'0' + d) as char);
    }
    result.push((b'0' + check_digit) as char);
    result
}

/// Convert a 12-digit UPC-A code to its 8-digit UPC-E equivalent.
/// Returns None if the UPC-A cannot be converted.
fn convert_upc_a2e(upc_a: &str) -> Option<String> {
    let digits: Vec<u8> = upc_a.bytes().map(|b| b - b'0').collect();
    if digits.len() != 12 {
        return None;
    }
    let nsd = digits[0]; // number_system_digit
    if nsd > 1 {
        return None;
    }
    let check = digits[11];

    // Pattern 1: digits[3] ∈ {0,1,2} and digits[4..8] == [0,0,0,0]
    if digits[3] <= 2 && digits[4] == 0 && digits[5] == 0 && digits[6] == 0 && digits[7] == 0 {
        // mfr=digits[1..3], extra=digits[3], product=digits[8..11]
        let mut s = String::with_capacity(8);
        s.push((b'0' + nsd) as char);
        s.push((b'0' + digits[1]) as char);
        s.push((b'0' + digits[2]) as char);
        s.push((b'0' + digits[8]) as char);
        s.push((b'0' + digits[9]) as char);
        s.push((b'0' + digits[10]) as char);
        s.push((b'0' + digits[3]) as char);
        s.push((b'0' + check) as char);
        return Some(s);
    }

    // Pattern 2a: mfr=3 digits + 00000 + product=2 digits
    // digits[1..4] = mfr, digits[4..9] = 00000, digits[9..11] = product
    if digits[4] == 0 && digits[5] == 0 && digits[6] == 0 && digits[7] == 0 && digits[8] == 0 {
        let mut s = String::with_capacity(8);
        s.push((b'0' + nsd) as char);
        s.push((b'0' + digits[1]) as char);
        s.push((b'0' + digits[2]) as char);
        s.push((b'0' + digits[3]) as char);
        s.push((b'0' + digits[9]) as char);
        s.push((b'0' + digits[10]) as char);
        s.push(b'3' as char); // extra = 3
        s.push((b'0' + check) as char);
        return Some(s);
    }

    // Pattern 2b: mfr=4 digits + 00000 + product=1 digit
    // digits[1..5] = mfr, digits[5..10] = 00000, digits[10] = product
    if digits[5] == 0 && digits[6] == 0 && digits[7] == 0 && digits[8] == 0 && digits[9] == 0 {
        let mut s = String::with_capacity(8);
        s.push((b'0' + nsd) as char);
        s.push((b'0' + digits[1]) as char);
        s.push((b'0' + digits[2]) as char);
        s.push((b'0' + digits[3]) as char);
        s.push((b'0' + digits[4]) as char);
        s.push((b'0' + digits[10]) as char);
        s.push(b'4' as char); // extra = 4
        s.push((b'0' + check) as char);
        return Some(s);
    }

    // Pattern 3: mfr=5 digits + 0000 + extra ∈ {5..9}
    // digits[1..6] = mfr, digits[6..10] = 0000, digits[10] ∈ {5..9}
    if digits[6] == 0 && digits[7] == 0 && digits[8] == 0 && digits[9] == 0 && digits[10] >= 5 {
        let mut s = String::with_capacity(8);
        s.push((b'0' + nsd) as char);
        s.push((b'0' + digits[1]) as char);
        s.push((b'0' + digits[2]) as char);
        s.push((b'0' + digits[3]) as char);
        s.push((b'0' + digits[4]) as char);
        s.push((b'0' + digits[5]) as char);
        s.push((b'0' + digits[10]) as char);
        s.push((b'0' + check) as char);
        return Some(s);
    }

    None
}

fn random_base(f: &Faker) -> [u8; 6] {
    [
        f.rng.random_digit() as u8,
        f.rng.random_digit() as u8,
        f.rng.random_digit() as u8,
        f.rng.random_digit() as u8,
        f.rng.random_digit() as u8,
        f.rng.random_digit() as u8,
    ]
}

fn upc_a(f: &Faker, locale: &str) -> String {
    // Default mode: EAN-13 with leading zero, strip first digit
    let prefix = pick_prefix(f, EN_US_PREFIXES);
    let ean13 = ean_inner(f, 13, Some(prefix.unwrap_or((0, 0))));
    // Force leading zero
    if ean13.starts_with('0') {
        ean13[1..].to_string()
    } else {
        // Build UPC-A from EAN-13 with leading 0
        let mut with_zero = String::from("0");
        with_zero.push_str(&ean13[1..]);
        // Recompute: just generate a new one with leading zero
        let e = ean_inner(f, 13, Some((0, f.rng.random_digit() as u8)));
        e[1..].to_string()
    }
}

fn upc_e(f: &Faker, locale: &str) -> String {
    let base = random_base(f);
    let nsd = f.rng.below(2) as u8;
    let upc_a_str = upc_ae_inner(f, base, nsd);
    convert_upc_a2e(&upc_a_str).unwrap_or_else(|| {
        // Fallback: produce a safe UPC-E using pattern 3 (extra=5..9)
        let mut b = random_base(f);
        b[5] = 5 + f.rng.below(5) as u8; // ensure pattern 3
        let ua = upc_ae_inner(f, b, nsd);
        convert_upc_a2e(&ua).unwrap_or_else(|| {
            // Last resort
            format!(
                "{}{}{}",
                nsd,
                b.iter().map(|d| (b'0' + d) as char).collect::<String>(),
                0
            )
        })
    })
}

// ---- JAN codes (ja_JP aliases for localized EAN) ---------------------------

fn jan(f: &Faker, locale: &str) -> String {
    localized_ean13(f, locale)
}

fn jan8(f: &Faker, locale: &str) -> String {
    localized_ean8(f, locale)
}

fn jan13(f: &Faker, locale: &str) -> String {
    localized_ean13(f, locale)
}

// ---- dispatch --------------------------------------------------------------

/// Return `Some(value)` for a formatter this module implements, else `None`.
pub fn dispatch(f: &Faker, locale: &str, name: &str) -> Option<String> {
    Some(match name {
        "ean" => ean(f, locale),
        "ean8" => ean8(f, locale),
        "ean13" => ean13(f, locale),
        "localized_ean" => localized_ean(f, locale),
        "localized_ean8" => localized_ean8(f, locale),
        "localized_ean13" => localized_ean13(f, locale),
        "upc_a" => upc_a(f, locale),
        "upc_e" => upc_e(f, locale),
        "jan" => jan(f, locale),
        "jan8" => jan8(f, locale),
        "jan13" => jan13(f, locale),
        _ => return None,
    })
}
