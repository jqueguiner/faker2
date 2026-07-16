//! Algorithmic/data formatters for the `user_agent` provider.
#![allow(unused_variables, clippy::all)]
use crate::faker::Faker;

// ── static data ────────────────────────────────────────────────────────────

const WINDOWS_PLATFORM_TOKENS: &[&str] = &[
    "Windows 95",
    "Windows 98",
    "Windows 98; Win 9x 4.90",
    "Windows CE",
    "Windows NT 4.0",
    "Windows NT 5.0",
    "Windows NT 5.01",
    "Windows NT 5.1",
    "Windows NT 5.2",
    "Windows NT 6.0",
    "Windows NT 6.1",
    "Windows NT 6.2",
    "Windows NT 10.0",
    "Windows NT 11.0",
];

const LINUX_PROCESSORS: &[&str] = &["i686", "x86_64"];

const MAC_PROCESSORS: &[&str] = &["Intel", "PPC", "U; Intel", "U; PPC"];

const ANDROID_VERSIONS: &[&str] = &[
    "1.0", "1.1", "1.5", "1.6", "2.0", "2.0.1", "2.1", "2.2", "2.2.1", "2.2.2", "2.2.3", "2.3",
    "2.3.1", "2.3.2", "2.3.3", "2.3.4", "2.3.5", "2.3.6", "2.3.7", "3.0", "3.1", "3.2", "3.2.1",
    "3.2.2", "3.2.3", "3.2.4", "3.2.5", "3.2.6", "4.0", "4.0.1", "4.0.2", "4.0.3", "4.0.4", "4.1",
    "4.1.1", "4.1.2", "4.2", "4.2.1", "4.2.2", "4.3", "4.3.1", "4.4", "4.4.1", "4.4.2", "4.4.3",
    "4.4.4", "5.0", "5.0.1", "5.0.2", "5.1", "5.1.1", "6.0", "6.0.1", "7.0", "7.1", "7.1.1",
    "7.1.2", "8.0.0", "8.1.0", "9", "10", "11", "12", "12.1", "13", "14",
];

const APPLE_DEVICES: &[&str] = &["iPhone", "iPad"];

const IOS_VERSIONS: &[&str] = &[
    "1.1.5", "2.2.1", "3.1.3", "3.2.2", "4.2.1", "4.3.5", "5.1.1", "6.1.6", "7.1.2", "8.4.1",
    "9.3.5", "9.3.6", "10.3.3", "10.3.4", "11.4.1", "12.4.4", "12.4.8", "12.5.7", "13.5.1", "13.7",
    "14.2", "14.2.1", "14.8.1", "15.8.2", "16.7.6", "16.7.7", "17.1", "17.1.1", "17.1.2", "17.2",
    "17.2.1", "17.3", "17.3.1", "17.4", "17.4.1",
];

// ── helpers ─────────────────────────────────────────────────────────────────

fn pick<'a>(f: &Faker, items: &[&'a str]) -> &'a str {
    items[f.rng.below(items.len())]
}

fn windows_platform_token(f: &Faker) -> String {
    pick(f, WINDOWS_PLATFORM_TOKENS).to_string()
}

fn linux_platform_token(f: &Faker) -> String {
    format!("X11; Linux {}", pick(f, LINUX_PROCESSORS))
}

fn mac_platform_token(f: &Faker) -> String {
    format!(
        "Macintosh; {} Mac OS X 10_{}_{}",
        pick(f, MAC_PROCESSORS),
        f.rng.random_int(5, 12, 1),
        f.rng.random_int(0, 9, 1),
    )
}

fn android_platform_token(f: &Faker) -> String {
    format!("Android {}", pick(f, ANDROID_VERSIONS))
}

fn ios_platform_token(f: &Faker) -> String {
    let device = pick(f, APPLE_DEVICES);
    let version = pick(f, IOS_VERSIONS).replace('.', "_");
    format!("{device}; CPU {device} OS {version} like Mac OS X")
}

// ── date helper (civil calendar from days since epoch) ──────────────────────

fn days_to_date(days: i64) -> String {
    // Gregorian calendar arithmetic, days since 1970-01-01
    let z = days + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    format!("{y:04}-{m:02}-{d:02}")
}

fn random_gecko_date(f: &Faker) -> String {
    let base_days = 14610_i64; // 2010-01-01
    let range = 365 * 13; // through ~2023
    days_to_date(base_days + f.rng.random_int(0, range, 1))
}

fn lexify_upper(f: &Faker, s: &str) -> String {
    s.chars()
        .map(|c| {
            if c == '?' {
                (b'A' + (f.rng.below(26) as u8)) as char
            } else {
                c
            }
        })
        .collect()
}

fn lexify_lower(f: &Faker, s: &str) -> String {
    s.chars()
        .map(|c| {
            if c == '?' {
                (b'a' + (f.rng.below(26) as u8)) as char
            } else {
                c
            }
        })
        .collect()
}

// ── browser formatters ───────────────────────────────────────────────────────

fn chrome(f: &Faker, _locale: &str) -> String {
    let saf = format!(
        "{}.{}",
        f.rng.random_int(531, 536, 1),
        f.rng.random_int(0, 2, 1)
    );
    let bld = lexify_upper(f, &f.rng.numerify("##?###"));

    let version_from = 13_i64;
    let version_to = 63_i64;
    let build_from = 800_i64;
    let build_to = 899_i64;

    let tmplt = |platform: &str| {
        format!(
            "({platform}) AppleWebKit/{saf} (KHTML, like Gecko) Chrome/{}.0.{}.0 Safari/{saf}",
            f.rng.random_int(version_from, version_to, 1),
            f.rng.random_int(build_from, build_to, 1),
        )
    };
    let tmplt_ios = |platform: &str| {
        format!(
            "({platform}) AppleWebKit/{saf} (KHTML, like Gecko) CriOS/{}.0.{}.0 Mobile/{bld} Safari/{saf}",
            f.rng.random_int(version_from, version_to, 1),
            f.rng.random_int(build_from, build_to, 1),
        )
    };

    let platform_str = match f.rng.below(5) {
        0 => tmplt(&linux_platform_token(f)),
        1 => tmplt(&windows_platform_token(f)),
        2 => tmplt(&mac_platform_token(f)),
        3 => tmplt(&format!("Linux; {}", android_platform_token(f))),
        _ => tmplt_ios(&ios_platform_token(f)),
    };

    format!("Mozilla/5.0 {platform_str}")
}

fn firefox(f: &Faker, locale: &str) -> String {
    let saf = format!(
        "{}.{}",
        f.rng.random_int(531, 536, 1),
        f.rng.random_int(0, 2, 1)
    );
    let bld = lexify_upper(f, &f.rng.numerify("##?###"));
    let bld2 = lexify_lower(f, &f.rng.numerify("#?####"));

    let ver = match f.rng.below(3) {
        0 => format!(
            "Gecko/{} Firefox/{}.0",
            random_gecko_date(f),
            f.rng.random_int(4, 15, 1)
        ),
        1 => format!(
            "Gecko/{} Firefox/3.6.{}",
            random_gecko_date(f),
            f.rng.random_int(1, 20, 1)
        ),
        _ => format!("Gecko/{} Firefox/3.8", random_gecko_date(f)),
    };

    let locale_str = locale.replace('_', "-");
    let platform_str = match f.rng.below(5) {
        0 => format!(
            "({win}; {locale_str}; rv:1.9.{}.20) {ver}",
            f.rng.random_int(0, 2, 1),
            win = windows_platform_token(f),
        ),
        1 => format!(
            "({lin}; rv:1.9.{}.20) {ver}",
            f.rng.random_int(5, 7, 1),
            lin = linux_platform_token(f),
        ),
        2 => format!(
            "({mac}; rv:1.9.{}.20) {ver}",
            f.rng.random_int(2, 6, 1),
            mac = mac_platform_token(f),
        ),
        3 => {
            let rv = f.rng.random_int(5, 68, 1);
            format!(
                "({and}; Mobile; rv:{rv}.0) Gecko/{rv}.0 Firefox/{rv}.0",
                and = android_platform_token(f),
            )
        }
        _ => format!(
            "({ios}) AppleWebKit/{saf} (KHTML, like Gecko) FxiOS/{}.{bld2}.0 Mobile/{bld} Safari/{saf}",
            f.rng.random_int(9, 18, 1),
            ios = ios_platform_token(f),
        ),
    };

    format!("Mozilla/5.0 {platform_str}")
}

fn safari(f: &Faker, locale: &str) -> String {
    let saf = format!(
        "{}.{}.{}",
        f.rng.random_int(531, 535, 1),
        f.rng.random_int(1, 50, 1),
        f.rng.random_int(1, 7, 1),
    );
    let ver = if f.rng.unit() < 0.5 {
        format!(
            "{}.{}",
            f.rng.random_int(4, 5, 1),
            f.rng.random_int(0, 1, 1)
        )
    } else {
        format!(
            "{}.0.{}",
            f.rng.random_int(4, 5, 1),
            f.rng.random_int(1, 5, 1)
        )
    };

    let locale_str = locale.replace('_', "-");
    let platform_str = match f.rng.below(3) {
        0 => format!(
            "(Windows; U; {win}) AppleWebKit/{saf} (KHTML, like Gecko) Version/{ver} Safari/{saf}",
            win = windows_platform_token(f),
        ),
        1 => format!(
            "({mac} rv:{}.0; {locale_str}) AppleWebKit/{saf} (KHTML, like Gecko) Version/{ver} Safari/{saf}",
            f.rng.random_int(2, 6, 1),
            mac = mac_platform_token(f),
        ),
        _ => format!(
            "(iPod; U; CPU iPhone OS {}_{} like Mac OS X; {locale_str}) AppleWebKit/{saf} (KHTML, like Gecko) Version/{}.0.5 Mobile/8B{} Safari/6{saf}",
            f.rng.random_int(3, 4, 1),
            f.rng.random_int(0, 3, 1),
            f.rng.random_int(3, 4, 1),
            f.rng.random_int(111, 119, 1),
        ),
    };

    format!("Mozilla/5.0 {platform_str}")
}

fn opera(f: &Faker, locale: &str) -> String {
    let token = if f.rng.unit() < 0.5 {
        linux_platform_token(f)
    } else {
        windows_platform_token(f)
    };
    let locale_str = locale.replace('_', "-");
    format!(
        "Opera/{}.{}.({token}; {locale_str}) Presto/2.9.{} Version/{}.00",
        f.rng.random_int(8, 9, 1),
        f.rng.random_int(10, 99, 1),
        f.rng.random_int(160, 190, 1),
        f.rng.random_int(10, 12, 1),
    )
}

fn internet_explorer(f: &Faker, _locale: &str) -> String {
    format!(
        "Mozilla/5.0 (compatible; MSIE {}.0; {}; Trident/{}.{})",
        f.rng.random_int(5, 9, 1),
        windows_platform_token(f),
        f.rng.random_int(3, 5, 1),
        f.rng.random_int(0, 1, 1),
    )
}

// ── dispatch ─────────────────────────────────────────────────────────────────

pub fn dispatch(f: &Faker, locale: &str, name: &str) -> Option<String> {
    Some(match name {
        "chrome" => chrome(f, locale),
        "firefox" => firefox(f, locale),
        "safari" => safari(f, locale),
        "opera" => opera(f, locale),
        "internet_explorer" => internet_explorer(f, locale),
        "android_platform_token" => android_platform_token(f),
        "ios_platform_token" => ios_platform_token(f),
        "linux_platform_token" => linux_platform_token(f),
        "mac_platform_token" => mac_platform_token(f),
        _ => return None,
    })
}
