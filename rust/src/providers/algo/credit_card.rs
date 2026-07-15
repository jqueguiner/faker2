//! Algorithmic formatters for the `credit_card` provider.
//!
//! Ports `faker2.providers.credit_card.Provider` — Luhn-valid card numbers,
//! expiry dates, security codes, provider names, and full card details.
#![allow(unused_variables, clippy::all)]
use crate::faker::Faker;

// ---------------------------------------------------------------------------
// Card type constants (mirrors Python class-level lists)
// ---------------------------------------------------------------------------

struct CardType {
    name: &'static str,
    prefixes: &'static [&'static str],
    length: usize,
    security_code: &'static str,
    security_code_length: usize,
}

static CARD_TYPES: &[(&str, CardType)] = &[
    (
        "maestro",
        CardType {
            name: "Maestro",
            prefixes: &[
                "5018", "5020", "5038", "56##", "57##", "58##", "6304", "6759", "6761", "6762",
                "6763", "0604", "6390",
            ],
            length: 12,
            security_code: "CVV",
            security_code_length: 3,
        },
    ),
    (
        "mastercard",
        CardType {
            name: "Mastercard",
            prefixes: &[
                "51", "52", "53", "54", "55", "222%", "223", "224", "225", "226", "227", "228",
                "229", "23", "24", "25", "26", "270", "271", "2720",
            ],
            length: 16,
            security_code: "CVV",
            security_code_length: 3,
        },
    ),
    (
        "visa16",
        CardType {
            name: "VISA 16 digit",
            prefixes: &["4"],
            length: 16,
            security_code: "CVV",
            security_code_length: 3,
        },
    ),
    (
        "visa13",
        CardType {
            name: "VISA 13 digit",
            prefixes: &["4"],
            length: 13,
            security_code: "CVV",
            security_code_length: 3,
        },
    ),
    (
        "visa19",
        CardType {
            name: "VISA 19 digit",
            prefixes: &["4"],
            length: 19,
            security_code: "CVV",
            security_code_length: 3,
        },
    ),
    (
        "amex",
        CardType {
            name: "American Express",
            prefixes: &["34", "37"],
            length: 15,
            security_code: "CID",
            security_code_length: 4,
        },
    ),
    (
        "discover",
        CardType {
            name: "Discover",
            prefixes: &["6011", "65"],
            length: 16,
            security_code: "CVV",
            security_code_length: 3,
        },
    ),
    (
        "diners",
        CardType {
            name: "Diners Club / Carte Blanche",
            prefixes: &["300", "301", "302", "303", "304", "305", "36", "38"],
            length: 14,
            security_code: "CVV",
            security_code_length: 3,
        },
    ),
    (
        "jcb15",
        CardType {
            name: "JCB 15 digit",
            prefixes: &["2131", "1800"],
            length: 15,
            security_code: "CVV",
            security_code_length: 3,
        },
    ),
    (
        "jcb16",
        CardType {
            name: "JCB 16 digit",
            prefixes: &["35"],
            length: 16,
            security_code: "CVV",
            security_code_length: 3,
        },
    ),
    // aliases
    (
        "visa",
        CardType {
            name: "VISA 16 digit",
            prefixes: &["4"],
            length: 16,
            security_code: "CVV",
            security_code_length: 3,
        },
    ),
    (
        "jcb",
        CardType {
            name: "JCB 16 digit",
            prefixes: &["35"],
            length: 16,
            security_code: "CVV",
            security_code_length: 3,
        },
    ),
];

/// Luhn double-digit lookup table: index is the digit (0-9),
/// value is the sum contribution when doubled.
const LUHN_DOUBLE: [u32; 10] = [0, 2, 4, 6, 8, 1, 3, 5, 7, 9];

// ---------------------------------------------------------------------------
// Helper: pick a random card type
// ---------------------------------------------------------------------------
fn pick_card_type<'a>(f: &Faker) -> &'a CardType {
    let idx = f.rng.below(CARD_TYPES.len());
    &CARD_TYPES[idx].1
}

// ---------------------------------------------------------------------------
// Helper: generate a Luhn-valid number from a prefix + total length
// ---------------------------------------------------------------------------
fn generate_number(f: &Faker, prefix: &str, length: usize) -> String {
    // expand '#' placeholders in prefix (e.g. "56##" -> "5634")
    let expanded_prefix = f.rng.numerify(prefix);

    // how many more digits we need (leave 1 for the check digit)
    let fill = if length > expanded_prefix.len() + 1 {
        length - expanded_prefix.len() - 1
    } else {
        0
    };
    let filler: String = (0..fill)
        .map(|_| char::from_digit(f.rng.random_digit(), 10).unwrap_or('0'))
        .collect();

    let partial = format!("{}{}", expanded_prefix, filler);

    // Luhn checksum over partial (reversed, starting from position 0)
    let digits: Vec<u32> = partial.chars().filter_map(|c| c.to_digit(10)).collect();
    let mut tot: u32 = 0;
    let n = digits.len();
    for (pos, &d) in digits.iter().rev().enumerate() {
        if pos % 2 == 0 {
            // positions 0,2,4,... from right in the partial => double these
            tot += LUHN_DOUBLE[d as usize];
        } else {
            tot += d;
        }
    }
    let check = (10 - (tot % 10)) % 10;
    format!("{}{}", partial, check)
}

// ---------------------------------------------------------------------------
// Formatter implementations
// ---------------------------------------------------------------------------

fn credit_card_provider(f: &Faker, locale: &str) -> String {
    pick_card_type(f).name.to_string()
}

fn credit_card_number(f: &Faker, locale: &str) -> String {
    let card = pick_card_type(f);
    let prefix_idx = f.rng.below(card.prefixes.len());
    let prefix = card.prefixes[prefix_idx];
    generate_number(f, prefix, card.length)
}

fn credit_card_security_code(f: &Faker, locale: &str) -> String {
    let card = pick_card_type(f);
    let len = card.security_code_length;
    (0..len)
        .map(|_| char::from_digit(f.rng.random_digit(), 10).unwrap_or('0'))
        .collect()
}

/// Expiry date as MM/YY — "now" to "+10y".
/// We compute from a unix timestamp using civil-from-days math.
fn credit_card_expire(f: &Faker, locale: &str) -> String {
    // Use system time for "now" base
    let now_secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(1_700_000_000);

    // Random offset: 0 to 10 years in seconds
    let offset_secs = f.rng.random_int(0, 10 * 365 * 24 * 3600, 1);
    let ts = now_secs + offset_secs;

    // Civil-from-days: compute year/month from unix timestamp
    let days = (ts / 86400) as i32;
    let (year, month, _day) = civil_from_days(days);

    format!("{:02}/{:02}", month, year % 100)
}

/// Convert days-since-Unix-epoch to (year, month, day).
/// Algorithm: Euclidean affine transform from Howard Hinnant.
fn civil_from_days(z: i32) -> (i32, u32, u32) {
    let z = z + 719_468;
    let era: i32 = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u32; // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365; // [0, 399]
    let y = yoe as i32 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let d = doy - (153 * mp + 2) / 5 + 1; // [1, 31]
    let m = if mp < 10 { mp + 3 } else { mp - 9 }; // [1, 12]
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

fn credit_card_full(f: &Faker, locale: &str) -> String {
    let card = pick_card_type(f);
    let prefix_idx = f.rng.below(card.prefixes.len());
    let prefix = card.prefixes[prefix_idx];
    let number = generate_number(f, prefix, card.length);
    let expire = credit_card_expire(f, locale);
    let sec_code: String = (0..card.security_code_length)
        .map(|_| char::from_digit(f.rng.random_digit(), 10).unwrap_or('0'))
        .collect();
    // Generate owner name using lparse if available, else fallback
    let owner = f.lparse(locale, "{{first_name}} {{last_name}}");
    format!(
        "{}\n{}\n{} {}\n{}: {}\n",
        card.name, owner, number, expire, card.security_code, sec_code
    )
}

// ---------------------------------------------------------------------------
// Dispatch
// ---------------------------------------------------------------------------

/// Return `Some(value)` for a formatter this module implements, else `None`.
pub fn dispatch(f: &Faker, locale: &str, name: &str) -> Option<String> {
    Some(match name {
        "credit_card_provider" => credit_card_provider(f, locale),
        "credit_card_number" => credit_card_number(f, locale),
        "credit_card_expire" => credit_card_expire(f, locale),
        "credit_card_security_code" => credit_card_security_code(f, locale),
        "credit_card_full" => credit_card_full(f, locale),
        _ => return None,
    })
}
