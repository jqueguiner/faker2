//! Algorithmic formatters for the `misc` provider.
//!
//! Implements: boolean, null_boolean, md5, sha1, sha256, uuid4, uuid1, uuid7, password.
//! Skipped (return non-String or need external I/O): binary, zip, tar, image,
//! dsv, csv, tsv, psv, json, json_bytes, xml, fixed_width.
#![allow(unused_variables, clippy::all)]
use crate::faker::Faker;

// ---- helpers ----------------------------------------------------------------

/// Generate `n` random hex chars (lowercase) using the RNG.
fn rand_hex(f: &Faker, n: usize) -> String {
    (0..n).map(|_| f.rng.hexify("^", false)).collect()
}

/// Password character sets.
const SPECIAL: &[u8] = b"!@#$%^&*()_+";
const DIGITS: &[u8] = b"0123456789";
const UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";

// ---- formatter impls --------------------------------------------------------

/// boolean — 50 % chance of true.
fn boolean(f: &Faker, _locale: &str) -> String {
    if f.rng.random_int(1, 100, 1) <= 50 {
        "True".to_string()
    } else {
        "False".to_string()
    }
}

/// null_boolean — None / True / False with equal probability.
fn null_boolean(f: &Faker, _locale: &str) -> String {
    match f.rng.below(3) {
        0 => "None".to_string(),
        1 => "True".to_string(),
        _ => "False".to_string(),
    }
}

/// md5 — 32-char lowercase hex string (PRNG-sourced, not crypto-hash).
fn md5(f: &Faker, _locale: &str) -> String {
    rand_hex(f, 32)
}

/// sha1 — 40-char lowercase hex string.
fn sha1(f: &Faker, _locale: &str) -> String {
    rand_hex(f, 40)
}

/// sha256 — 64-char lowercase hex string.
fn sha256(f: &Faker, _locale: &str) -> String {
    rand_hex(f, 64)
}

/// uuid4 — RFC-4122 v4 UUID string.
fn uuid4(f: &Faker, _locale: &str) -> String {
    format!(
        "{}-{}-4{}-{}{}-{}",
        rand_hex(f, 8),
        rand_hex(f, 4),
        rand_hex(f, 3),
        // variant bits: 8, 9, a, b
        ["8", "9", "a", "b"][f.rng.below(4)],
        rand_hex(f, 3),
        rand_hex(f, 12),
    )
}

/// uuid1 — time-based UUID v1 string (time from UNIX epoch approximation).
fn uuid1(f: &Faker, _locale: &str) -> String {
    // UUID1 timestamp is 100-ns intervals since 1582-10-15.
    // We approximate using a fixed large offset + random bits.
    // The constant 0x01B21DD213814000 is the offset from 1582 to 1970 epoch.
    let ts_low: u64 = f.rng.random_int(0, i64::MAX, 1) as u64 & 0xFFFF_FFFF;
    let ts_mid: u64 = f.rng.random_int(0, 0xFFFF, 1) as u64;
    let ts_hi: u64 = f.rng.random_int(0, 0x0FFF, 1) as u64;
    let clock_hi: u64 = 0x80 | (f.rng.random_int(0, 0x3F, 1) as u64); // variant RFC4122
    let clock_low: u64 = f.rng.random_int(0, 0xFF, 1) as u64;
    let node = rand_hex(f, 12);
    format!(
        "{:08x}-{:04x}-1{:03x}-{:02x}{:02x}-{}",
        ts_low, ts_mid, ts_hi, clock_hi, clock_low, node
    )
}

/// uuid7 — Unix-Epoch-time-ordered UUID v7 string (RFC 9562).
fn uuid7(f: &Faker, _locale: &str) -> String {
    // 48-bit ms timestamp (use PRNG as proxy for wall-clock portability).
    let ts_ms: u64 = f.rng.random_int(0, 0x0000_FFFF_FFFF_FFFFi64, 1) as u64;
    let rand_a: u64 = f.rng.random_int(0, 0x0FFF, 1) as u64;
    let rand_b_hi: u64 = f.rng.random_int(0, 0x3FFF, 1) as u64 | 0x8000; // variant 10xx
    let rand_b_lo: u64 = f.rng.random_int(0, i32::MAX as i64, 1) as u64;
    let rand_b_lo2: u64 = f.rng.random_int(0, i32::MAX as i64, 1) as u64;
    let node = format!("{:04x}{:08x}", rand_b_lo & 0xFFFF, rand_b_lo2 & 0xFFFF_FFFF);
    format!(
        "{:012x}-{:04x}-7{:03x}-{:04x}-{}",
        ts_ms,
        rand_a >> 4,
        rand_a & 0x0FFF,
        rand_b_hi,
        node
    )
}

/// password — 10 chars by default: special + digit + upper + lower guaranteed.
fn password(f: &Faker, _locale: &str) -> String {
    let length = 10usize;
    // Pool of all valid password characters.
    let mut pool: Vec<u8> = Vec::new();
    pool.extend_from_slice(SPECIAL);
    pool.extend_from_slice(DIGITS);
    pool.extend_from_slice(UPPER);
    pool.extend_from_slice(LOWER);

    // One required char from each category.
    let required: Vec<u8> = vec![
        SPECIAL[f.rng.below(SPECIAL.len())],
        DIGITS[f.rng.below(DIGITS.len())],
        UPPER[f.rng.below(UPPER.len())],
        LOWER[f.rng.below(LOWER.len())],
    ];

    // Fill remaining positions randomly from the full pool.
    let mut chars: Vec<u8> = (0..length).map(|_| pool[f.rng.below(pool.len())]).collect();

    // Place the required chars at distinct random positions.
    let mut positions: Vec<usize> = Vec::new();
    while positions.len() < required.len() {
        let idx = f.rng.below(length);
        if !positions.contains(&idx) {
            positions.push(idx);
        }
    }
    for (i, pos) in positions.into_iter().enumerate() {
        chars[pos] = required[i];
    }

    String::from_utf8(chars).unwrap_or_else(|_| "Password1!".to_string())
}

// ---- dispatch ---------------------------------------------------------------

/// Return `Some(value)` for a formatter this module implements, else `None`.
pub fn dispatch(f: &Faker, locale: &str, name: &str) -> Option<String> {
    Some(match name {
        "boolean" => boolean(f, locale),
        "null_boolean" => null_boolean(f, locale),
        "md5" => md5(f, locale),
        "sha1" => sha1(f, locale),
        "sha256" => sha256(f, locale),
        "uuid4" => uuid4(f, locale),
        "uuid1" => uuid1(f, locale),
        "uuid7" => uuid7(f, locale),
        "password" => password(f, locale),
        _ => return None,
    })
}
