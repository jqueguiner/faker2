#![cfg(feature = "locales")]
#![allow(clippy::all)]
use faker2::Faker;

fn digits(s: &str) -> Vec<u32> {
    s.chars().filter_map(|c| c.to_digit(10)).collect()
}
fn luhn_ok(s: &str) -> bool {
    let d = digits(s);
    let mut sum = 0u32;
    for (i, &x) in d.iter().rev().enumerate() {
        sum += if i % 2 == 1 {
            let y = x * 2;
            if y > 9 {
                y - 9
            } else {
                y
            }
        } else {
            x
        };
    }
    !d.is_empty() && sum % 10 == 0
}
fn ean_ok(s: &str) -> bool {
    let d = digits(s);
    if d.len() != 13 {
        return false;
    }
    let mut sum = 0u32;
    for (i, &x) in d.iter().enumerate().take(12) {
        sum += x * if i % 2 == 0 { 1 } else { 3 };
    }
    let check = (10 - sum % 10) % 10;
    check == d[12]
}
fn iban_ok(s: &str) -> bool {
    let s: String = s.chars().filter(|c| c.is_alphanumeric()).collect();
    if s.len() < 8 {
        return false;
    }
    let rearranged = format!("{}{}", &s[4..], &s[..4]);
    let mut rem = 0u32;
    for c in rearranged.chars() {
        let v = if c.is_ascii_digit() {
            c as u32 - '0' as u32
        } else {
            c.to_ascii_uppercase() as u32 - 'A' as u32 + 10
        };
        for dg in v.to_string().chars() {
            rem = (rem * 10 + (dg as u32 - '0' as u32)) % 97;
        }
    }
    rem == 1
}

#[test]
fn credit_card_numbers_pass_luhn() {
    let f = Faker::seeded(11);
    for _ in 0..200 {
        let n = f.gen("en_US", "credit_card_number").unwrap();
        assert!(luhn_ok(&n), "not Luhn-valid: {n}");
    }
}

#[test]
fn ean13_checksums_valid() {
    let f = Faker::seeded(12);
    for loc in ["en_US", "ja_JP", "es_ES"] {
        for _ in 0..100 {
            let n = f.gen(loc, "ean13").unwrap();
            assert!(ean_ok(&n), "bad EAN-13 ({loc}): {n}");
        }
    }
}

#[test]
fn ibans_pass_mod97() {
    let f = Faker::seeded(13);
    for loc in ["en_GB", "fr_FR", "de_DE"] {
        for _ in 0..100 {
            let n = f.gen(loc, "iban").unwrap();
            assert!(iban_ok(&n), "bad IBAN mod-97 ({loc}): {n}");
        }
    }
}

#[test]
fn currency_and_uuid_shapes() {
    let f = Faker::seeded(14);
    assert_eq!(f.gen("en_US", "currency_code").unwrap().len(), 3);
    assert_eq!(f.gen("en_US", "uuid4").unwrap().len(), 36);
}
