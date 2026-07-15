//! Algorithmic formatters for the `bank` provider.
//!
//! Ports: aba, bban, iban, swift, swift8, swift11
#![allow(unused_variables, clippy::all)]
use crate::faker::Faker;

// ---------------------------------------------------------------------------
// Locale → (country_code, bban_format) table.
// Derived from faker2.providers.bank.* subclasses.
// ---------------------------------------------------------------------------
fn locale_bank_params(locale: &str) -> (&'static str, &'static str) {
    match locale {
        "az_AZ" => ("AZ", "????####################"),
        "bn_BD" => ("BD", "????#########"),
        "cs_CZ" => ("CZ", "####################"),
        "da_DK" => ("DK", "##############"),
        "de_AT" => ("AT", "################"),
        "de_CH" | "fr_CH" | "it_CH" => ("CH", "#################"),
        "de_DE" => ("DE", "##################"),
        "el_GR" => ("GR", "#######################"),
        "en_GB" => ("GB", "????##############"),
        "en_IE" => ("IE", "????##############"),
        "en_IN" => ("GB", "????##############"),
        "en_PH" | "fil_PH" | "tl_PH" => ("PH", "################"),
        "es_AR" => ("AR", "????####################"),
        "es_ES" => ("ES", "####################"),
        "es_MX" => ("GB", "????##############"),
        "fa_IR" => ("IR", "IR########################"),
        "fi_FI" => ("FI", "##############"),
        "fr_FR" => ("FR", "#######################"),
        "it_IT" => ("IT", "?######################"),
        "mk_MK" => ("MK", "###????????????##"),
        "nl_BE" => ("BE", "############"),
        "nl_NL" => ("NL", "????##########"),
        "no_NO" => ("NO", "###########"),
        "pl_PL" => ("PL", "########################"),
        "pt_BR" => ("BR", "#######################??"),
        "pt_PT" => ("PT", "#####################"),
        "ro_RO" => ("RO", "????################"),
        "ru_RU" => ("RU", "##############???????????????"),
        "sk_SK" => ("SK", "####################"),
        "th_TH" => ("TH", "##########"),
        "tr_TR" => ("TR", "######################"),
        "uk_UA" => ("UA", "######???????????????????"),
        "zh_CN" => ("GB", "????##############"),
        // default (en_US, en, en_GB base)
        _ => ("GB", "????#############"),
    }
}

// ---------------------------------------------------------------------------
// ALPHA map: A=10, B=11, … Z=35  (ord(c) % 55 in Python)
// ---------------------------------------------------------------------------
fn alpha_val(c: char) -> u8 {
    (c as u8).wrapping_sub(b'A').wrapping_add(10)
}

// ---------------------------------------------------------------------------
// Expand BBAN format: '?' → random uppercase letter, '#' → digit
// ---------------------------------------------------------------------------
fn expand_bban(f: &Faker, fmt: &str) -> String {
    let mut out = String::with_capacity(fmt.len());
    for ch in fmt.chars() {
        match ch {
            '?' => {
                let idx = f.rng.below(26) as u8;
                out.push((b'A' + idx) as char);
            }
            '#' => out.push(char::from_digit(f.rng.random_digit(), 10).unwrap_or('0')),
            other => out.push(other),
        }
    }
    out
}

// ---------------------------------------------------------------------------
// ABA routing transit number (9 digits with weighted checksum)
// ---------------------------------------------------------------------------
fn aba(f: &Faker, _locale: &str) -> String {
    let fed_num = f.rng.random_int(1, 12, 1) as u32;
    // Six random digits
    let d4 = f.rng.random_digit();
    let d5 = f.rng.random_digit();
    let d6 = f.rng.random_digit();
    let d7 = f.rng.random_digit();
    let d8 = f.rng.random_digit();
    let d9 = f.rng.random_digit();

    // aba prefix is fed_num zero-padded to 2 digits
    let d = [(fed_num / 10) % 10, fed_num % 10, d4, d5, d6, d7, d8, d9];

    // checksum = ceil((3*(d0+d3+d6) + 7*(d1+d4+d7) + d2+d5) / 10)*10 - sum
    let sum = 3 * (d[0] + d[3] + d[6]) + 7 * (d[1] + d[4] + d[7]) + d[2] + d[5];
    let chk = sum.div_ceil(10) * 10 - sum; // ceil(sum/10)*10 - sum

    format!(
        "{:02}{}{}{}{}{}{}{}",
        fed_num,
        d4,
        d5,
        d6,
        d7,
        d8,
        d9,
        chk % 10
    )
}

// ---------------------------------------------------------------------------
// BBAN
// ---------------------------------------------------------------------------
fn bban(f: &Faker, locale: &str) -> String {
    let (_, fmt) = locale_bank_params(locale);
    expand_bban(f, fmt)
}

// ---------------------------------------------------------------------------
// IBAN  (ISO 7064 MOD-97-10)
// ---------------------------------------------------------------------------
fn iban(f: &Faker, locale: &str) -> String {
    let (country_code, fmt) = locale_bank_params(locale);
    let bban_str = expand_bban(f, fmt);

    // check = bban + country_code + "00"
    // convert each char: digit → itself, letter → alpha_val (2 digits)
    let check_input = format!("{}{}{}", bban_str, country_code, "00");
    // Build a big numeric string
    let mut numeric = String::with_capacity(check_input.len() * 2);
    for ch in check_input.chars() {
        if ch.is_ascii_uppercase() {
            let v = alpha_val(ch);
            numeric.push_str(&v.to_string());
        } else {
            numeric.push(ch);
        }
    }

    // Compute numeric mod 97 in chunks (avoid overflow)
    let mut remainder: u64 = 0;
    for ch in numeric.chars() {
        let digit = ch.to_digit(10).unwrap_or(0) as u64;
        remainder = (remainder * 10 + digit) % 97;
    }
    let check_digits = 98u64.wrapping_sub(remainder) % 97;

    format!("{}{:02}{}", country_code, check_digits, bban_str)
}

// ---------------------------------------------------------------------------
// SWIFT helpers
// ---------------------------------------------------------------------------
const ALPHA_UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const ALNUM_UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

fn random_alpha(f: &Faker, n: usize) -> String {
    (0..n)
        .map(|_| ALPHA_UPPER[f.rng.below(26)] as char)
        .collect()
}

fn random_alnum(f: &Faker, n: usize) -> String {
    (0..n)
        .map(|_| ALNUM_UPPER[f.rng.below(36)] as char)
        .collect()
}

fn swift_inner(f: &Faker, locale: &str, length: usize, primary: bool) -> String {
    let (country_code, _) = locale_bank_params(locale);

    // Bank code: 4 uppercase letters (use dataset if available)
    let bank_code = f
        .lpick(locale, "bank", "swift_bank_codes")
        .unwrap_or_else(|| random_alpha(f, 4));

    // Location code: 2 alphanumeric (use dataset if available)
    let location_code = f
        .lpick(locale, "bank", "swift_location_codes")
        .unwrap_or_else(|| random_alnum(f, 2));

    if length == 8 {
        format!("{}{}{}", bank_code, country_code, location_code)
    } else {
        // length == 11
        let branch_code = if primary {
            "XXX".to_string()
        } else {
            f.lpick(locale, "bank", "swift_branch_codes")
                .unwrap_or_else(|| random_alnum(f, 3))
        };
        format!(
            "{}{}{}{}",
            bank_code, country_code, location_code, branch_code
        )
    }
}

fn swift(f: &Faker, locale: &str) -> String {
    let length = if f.rng.below(2) == 0 { 8 } else { 11 };
    swift_inner(f, locale, length, false)
}

fn swift8(f: &Faker, locale: &str) -> String {
    swift_inner(f, locale, 8, false)
}

fn swift11(f: &Faker, locale: &str) -> String {
    swift_inner(f, locale, 11, false)
}

// ---------------------------------------------------------------------------
// Dispatch
// ---------------------------------------------------------------------------
pub fn dispatch(f: &Faker, locale: &str, name: &str) -> Option<String> {
    Some(match name {
        "aba" => aba(f, locale),
        "bban" => bban(f, locale),
        "iban" => iban(f, locale),
        "swift" => swift(f, locale),
        "swift8" => swift8(f, locale),
        "swift11" => swift11(f, locale),
        _ => return None,
    })
}
