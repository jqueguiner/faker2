//! Algorithmic formatters for the `passport` provider.
#![allow(unused_variables, clippy::all)]
use crate::faker::Faker;

const ASCII_UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// Generate a passport number from locale-specific formats.
/// `?` -> random uppercase letter, `#` -> random digit.
fn passport_number(f: &Faker, locale: &str) -> String {
    // Try locale-specific field first; it_IT uses a different field name.
    let formats = {
        let v = f.lfield(locale, "passport", "passport_number_formats");
        if v.is_empty() {
            f.lfield(locale, "passport", "electronic_passport_number_formats")
        } else {
            v
        }
    };

    let fmt = if formats.is_empty() {
        // Default fallback matching the base provider (en_US style)
        let defaults = ["?########", "#########"];
        defaults[f.rng.below(defaults.len())].to_string()
    } else {
        formats[f.rng.below(formats.len())].clone()
    };

    // Replace `?` with uppercase letter, `#` with digit (like Python's
    // re.sub + numerify approach in the base provider).
    let numerified = f.rng.numerify(&fmt);
    f.rng.lexify(&numerified, ASCII_UPPERCASE)
}

/// Generate a date-of-birth string (YYYY-MM-DD) for a passport.
/// Mirrors `passport_dob` which delegates to `date_of_birth`.
fn passport_dob(f: &Faker, locale: &str) -> String {
    // date_of_birth: age 0–115, mirrors date_time::date_of_birth
    const NOW_TS: i64 = 1_704_067_200; // 2024-01-01
    let min_ts = NOW_TS - 116_i64 * 365 * 86_400;
    let ts = f.rng.random_int(min_ts, NOW_TS, 1);

    // civil-from-days (same algorithm as date_time.rs)
    let secs_per_day: i64 = 86_400;
    let z = ts.div_euclid(secs_per_day) + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if m <= 2 { y + 1 } else { y } as i32;

    format!("{:04}-{:02}-{:02}", year, m as u32, d as u32)
}

/// Generate a passport owner as "GivenName Surname".
/// Picks gender randomly (M/F/X), mirrors `passport_owner`.
fn passport_owner(f: &Faker, locale: &str) -> String {
    // Pick gender: M ~49.3%, F ~49.3%, X ~1.4% (mirroring en_US weights)
    let r = f.rng.unit();
    let given_name = if r < 0.493 {
        // male
        f.lpick(locale, "person", "first_names_male")
            .or_else(|| f.lpick(locale, "person", "first_names"))
            .unwrap_or_else(|| "John".to_string())
    } else if r < 0.986 {
        // female
        f.lpick(locale, "person", "first_names_female")
            .or_else(|| f.lpick(locale, "person", "first_names"))
            .unwrap_or_else(|| "Jane".to_string())
    } else {
        // nonbinary
        f.lpick(locale, "person", "first_names_nonbinary")
            .or_else(|| f.lpick(locale, "person", "first_names"))
            .unwrap_or_else(|| "Alex".to_string())
    };

    let surname = f
        .lpick(locale, "person", "last_names")
        .unwrap_or_else(|| "Smith".to_string());

    format!("{} {}", given_name, surname)
}

/// Return `Some(value)` for a formatter this module implements, else `None`.
pub fn dispatch(f: &Faker, locale: &str, name: &str) -> Option<String> {
    Some(match name {
        "passport_dob" => passport_dob(f, locale),
        "passport_number" => passport_number(f, locale),
        "passport_owner" => passport_owner(f, locale),
        _ => return None,
    })
}
