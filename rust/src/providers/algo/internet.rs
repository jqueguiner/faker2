//! Algorithmic formatters for the `internet` provider.
#![allow(unused_variables, clippy::all)]
use crate::faker::Faker;

// ---------------------------------------------------------------------------
// Static data tables (mirroring Python Provider class attributes)
// ---------------------------------------------------------------------------

static SAFE_DOMAIN_NAMES: &[&str] = &["example.org", "example.com", "example.net"];

static FREE_EMAIL_DOMAINS: &[&str] = &["gmail.com", "yahoo.com", "hotmail.com"];

static TLDS: &[&str] = &[
    "com", "com", "com", "com", "com", "com", "biz", "info", "net", "org",
];

static HOSTNAME_PREFIXES: &[&str] = &["db", "srv", "desktop", "laptop", "lt", "email", "web"];

static URI_PAGES: &[&str] = &[
    "index", "home", "search", "main", "post", "homepage", "category", "register", "login", "faq",
    "about", "terms", "privacy", "author",
];

static URI_PATHS: &[&str] = &[
    "app",
    "main",
    "wp-content",
    "search",
    "category",
    "tag",
    "categories",
    "tags",
    "blog",
    "posts",
    "list",
    "explore",
];

static URI_EXTENSIONS: &[&str] = &[
    ".html", ".html", ".html", ".htm", ".htm", ".php", ".php", ".jsp", ".asp",
];

static HTTP_METHODS: &[&str] = &[
    "GET", "HEAD", "POST", "PUT", "DELETE", "CONNECT", "OPTIONS", "TRACE", "PATCH",
];

static HTTP_ASSIGNED_CODES: &[i64] = &[
    100, 101, 102, 103, 200, 201, 202, 203, 204, 205, 206, 207, 208, 226, 300, 301, 302, 303, 304,
    305, 307, 308, 400, 401, 402, 403, 404, 405, 406, 407, 408, 409, 410, 411, 412, 413, 414, 415,
    416, 417, 421, 422, 423, 424, 425, 426, 428, 429, 431, 451, 500, 501, 502, 503, 504, 505, 506,
    507, 508, 510, 511,
];

static IMAGE_PLACEHOLDER_SERVICES: &[&str] = &[
    "https://picsum.photos/{width}/{height}",
    "https://dummyimage.com/{width}x{height}",
    "https://placekittens.com/{width}/{height}",
];

// ---------------------------------------------------------------------------
// Helper functions
// ---------------------------------------------------------------------------

fn slugify(s: &str) -> String {
    s.chars()
        .filter_map(|c| {
            if c.is_ascii_alphanumeric() {
                Some(c.to_ascii_lowercase())
            } else if c == ' ' || c == '-' || c == '_' {
                Some('-')
            } else {
                None
            }
        })
        .collect()
}

fn safe_domain_name(f: &Faker) -> String {
    SAFE_DOMAIN_NAMES[f.rng.below(SAFE_DOMAIN_NAMES.len())].to_string()
}

fn tld(f: &Faker) -> String {
    TLDS[f.rng.below(TLDS.len())].to_string()
}

fn free_email_domain(f: &Faker) -> String {
    FREE_EMAIL_DOMAINS[f.rng.below(FREE_EMAIL_DOMAINS.len())].to_string()
}

fn domain_word(f: &Faker, locale: &str) -> String {
    // Try to get a company word from locale data, fall back to last_name
    let company = f
        .lpick(locale, "company", "companies")
        .or_else(|| f.lpick(locale, "person", "last_names"))
        .unwrap_or_else(|| "example".to_string());
    let word = company.split_whitespace().next().unwrap_or(&company);
    slugify(word)
}

fn domain_name(f: &Faker, locale: &str) -> String {
    format!("{}.{}", domain_word(f, locale), tld(f))
}

fn user_name(f: &Faker, locale: &str) -> String {
    let first = f
        .lpick(locale, "person", "first_names")
        .unwrap_or_else(|| "user".to_string());
    let last = f
        .lpick(locale, "person", "last_names")
        .unwrap_or_else(|| "name".to_string());
    let first = slugify(&first);
    let last = slugify(&last);
    match f.rng.below(4) {
        0 => format!("{}.{}", last, first),
        1 => format!("{}.{}", first, last),
        2 => format!("{}{}", first, f.rng.random_int(10, 99, 1)),
        _ => format!("{}{}", first, last),
    }
}

fn email(f: &Faker, locale: &str) -> String {
    format!("{}@{}", user_name(f, locale), domain_name(f, locale)).to_lowercase()
}

fn safe_email(f: &Faker, locale: &str) -> String {
    format!("{}@{}", user_name(f, locale), safe_domain_name(f)).to_lowercase()
}

fn free_email(f: &Faker, locale: &str) -> String {
    format!("{}@{}", user_name(f, locale), free_email_domain(f)).to_lowercase()
}

fn company_email(f: &Faker, locale: &str) -> String {
    format!("{}@{}", user_name(f, locale), domain_name(f, locale)).to_lowercase()
}

fn ascii_email(f: &Faker, locale: &str) -> String {
    // ASCII-only: slugify already strips non-ASCII
    email(f, locale)
}

fn ascii_safe_email(f: &Faker, locale: &str) -> String {
    safe_email(f, locale)
}

fn ascii_free_email(f: &Faker, locale: &str) -> String {
    free_email(f, locale)
}

fn ascii_company_email(f: &Faker, locale: &str) -> String {
    company_email(f, locale)
}

fn hostname(f: &Faker, locale: &str) -> String {
    let prefix = HOSTNAME_PREFIXES[f.rng.below(HOSTNAME_PREFIXES.len())];
    let num = f.rng.random_int(1, 99, 1);
    format!("{}-{:02}.{}", prefix, num, domain_name(f, locale))
}

/// DGA — Domain Generation Algorithm
fn dga(f: &Faker) -> String {
    let mut year = f.rng.random_int(1, 9999, 1);
    let mut month = f.rng.random_int(1, 12, 1);
    let mut day = f.rng.random_int(1, 30, 1);
    let tld_val = tld(f);
    let length = f.rng.random_int(2, 63, 1) as usize;

    let mut domain = String::with_capacity(length);
    for _ in 0..length {
        year = ((year ^ (8 * year)) >> 11)
            ^ (((year & 0xFFFFFFF0i64) << 17) & 0x7FFF_FFFF_FFFF_FFFFi64);
        month = ((month ^ (4 * month)) >> 25)
            ^ ((16 * (month & 0xFFFFFFF8i64)) & 0x7FFF_FFFF_FFFF_FFFFi64);
        day = ((day ^ (day << 13)) >> 19)
            ^ (((day & 0xFFFFFFFEi64) << 12) & 0x7FFF_FFFF_FFFF_FFFFi64);
        let c = (((year ^ month ^ day) % 25) + 97) as u8;
        domain.push(c as char);
    }
    format!("{}.{}", domain, tld_val)
}

fn http_method(f: &Faker) -> String {
    HTTP_METHODS[f.rng.below(HTTP_METHODS.len())].to_string()
}

fn http_status_code(f: &Faker) -> String {
    f.rng.random_int(100, 599, 1).to_string()
}

fn http_status_code_assigned(f: &Faker) -> String {
    HTTP_ASSIGNED_CODES[f.rng.below(HTTP_ASSIGNED_CODES.len())].to_string()
}

fn url(f: &Faker, locale: &str) -> String {
    let scheme = if f.rng.below(2) == 0 { "http" } else { "https" };
    if f.rng.below(2) == 0 {
        format!("{}://www.{}/", scheme, domain_name(f, locale))
    } else {
        format!("{}://{}/", scheme, domain_name(f, locale))
    }
}

fn ipv4(f: &Faker) -> String {
    format!(
        "{}.{}.{}.{}",
        f.rng.random_int(1, 255, 1),
        f.rng.random_int(0, 255, 1),
        f.rng.random_int(0, 255, 1),
        f.rng.random_int(1, 254, 1)
    )
}

fn ipv4_private(f: &Faker) -> String {
    // Pick from three common private ranges
    match f.rng.below(3) {
        0 => format!(
            "10.{}.{}.{}",
            f.rng.random_int(0, 255, 1),
            f.rng.random_int(0, 255, 1),
            f.rng.random_int(1, 254, 1)
        ),
        1 => format!(
            "172.{}.{}.{}",
            f.rng.random_int(16, 31, 1),
            f.rng.random_int(0, 255, 1),
            f.rng.random_int(1, 254, 1)
        ),
        _ => format!(
            "192.168.{}.{}",
            f.rng.random_int(0, 255, 1),
            f.rng.random_int(1, 254, 1)
        ),
    }
}

fn ipv4_public(f: &Faker) -> String {
    // Generate a public IP avoiding reserved ranges; keep it simple
    loop {
        let a = f.rng.random_int(1, 223, 1) as u8;
        let b = f.rng.random_int(0, 255, 1) as u8;
        let c = f.rng.random_int(0, 255, 1) as u8;
        let d = f.rng.random_int(1, 254, 1) as u8;
        // Skip loopback, private, link-local, reserved
        if a == 10 {
            continue;
        }
        if a == 127 {
            continue;
        }
        if a == 169 && b == 254 {
            continue;
        }
        if a == 172 && (16..=31).contains(&b) {
            continue;
        }
        if a == 192 && b == 168 {
            continue;
        }
        if a == 192 && b == 0 {
            continue;
        }
        if a >= 224 {
            continue;
        }
        return format!("{}.{}.{}.{}", a, b, c, d);
    }
}

fn ipv4_network_class(f: &Faker) -> String {
    let classes = ["a", "b", "c"];
    classes[f.rng.below(3)].to_string()
}

fn ipv6(f: &Faker) -> String {
    let groups: Vec<String> = (0..8)
        .map(|_| format!("{:04x}", f.rng.random_int(0, 0xFFFF, 1)))
        .collect();
    groups.join(":")
}

fn mac_address(f: &Faker) -> String {
    let mut parts = Vec::with_capacity(6);
    // First octet: even (unicast)
    parts.push(format!("{:02x}", f.rng.random_int(0, 127, 1) * 2));
    for _ in 1..6 {
        parts.push(format!("{:02x}", f.rng.random_int(0, 255, 1)));
    }
    parts.join(":")
}

fn port_number(f: &Faker) -> String {
    f.rng.random_int(0, 65535, 1).to_string()
}

fn port_number_system(f: &Faker) -> String {
    f.rng.random_int(0, 1023, 1).to_string()
}

fn port_number_user(f: &Faker) -> String {
    f.rng.random_int(1024, 49151, 1).to_string()
}

fn port_number_dynamic(f: &Faker) -> String {
    f.rng.random_int(49152, 65535, 1).to_string()
}

fn uri_page(f: &Faker) -> String {
    URI_PAGES[f.rng.below(URI_PAGES.len())].to_string()
}

fn uri_path(f: &Faker) -> String {
    let deep = f.rng.random_int(1, 3, 1) as usize;
    let parts: Vec<&str> = (0..deep)
        .map(|_| URI_PATHS[f.rng.below(URI_PATHS.len())])
        .collect();
    parts.join("/")
}

fn uri_extension(f: &Faker) -> String {
    URI_EXTENSIONS[f.rng.below(URI_EXTENSIONS.len())].to_string()
}

fn uri(f: &Faker, locale: &str) -> String {
    let scheme = if f.rng.below(2) == 0 { "http" } else { "https" };
    let base = if f.rng.below(2) == 0 {
        format!("www.{}", domain_name(f, locale))
    } else {
        domain_name(f, locale)
    };
    format!(
        "{}://{}/{}/{}{}",
        scheme,
        base,
        uri_path(f),
        uri_page(f),
        uri_extension(f)
    )
}

fn slug(f: &Faker, locale: &str) -> String {
    // Pick a few lorem words from locale or fallback English words
    static WORDS: &[&str] = &[
        "lorem",
        "ipsum",
        "dolor",
        "sit",
        "amet",
        "consectetur",
        "adipiscing",
        "elit",
        "sed",
        "do",
        "eiusmod",
        "tempor",
        "incididunt",
        "ut",
        "labore",
    ];
    let n = f.rng.random_int(2, 5, 1) as usize;
    let parts: Vec<&str> = (0..n).map(|_| WORDS[f.rng.below(WORDS.len())]).collect();
    parts.join("-")
}

fn image_url(f: &Faker) -> String {
    let width = f.rng.random_int(100, 1024, 1);
    let height = f.rng.random_int(100, 1024, 1);
    let tpl = IMAGE_PLACEHOLDER_SERVICES[f.rng.below(IMAGE_PLACEHOLDER_SERVICES.len())];
    tpl.replace("{width}", &width.to_string())
        .replace("{height}", &height.to_string())
}

fn iana_id(f: &Faker) -> String {
    f.rng.random_int(1, 8_888_888, 1).to_string()
}

fn ripe_id(f: &Faker) -> String {
    let lex_len = f.rng.random_int(2, 4, 1) as usize;
    let num_len = f.rng.random_int(1, 5, 1) as usize;
    let lex: String = (0..lex_len).map(|_| "?").collect::<Vec<_>>().join("");
    let num: String = (0..num_len).map(|_| "%").collect::<Vec<_>>().join("");
    let pattern = format!("ORG-{}{}-RIPE", lex, num);
    f.rng.bothify(&pattern).to_uppercase()
}

fn nic_handle(f: &Faker) -> String {
    let lex_len = f.rng.random_int(2, 4, 1) as usize;
    let num_len = f.rng.random_int(1, 5, 1) as usize;
    let lex: String = (0..lex_len).map(|_| "?").collect::<Vec<_>>().join("");
    let num: String = (0..num_len).map(|_| "%").collect::<Vec<_>>().join("");
    let pattern = format!("{}{}-FAKE", lex, num);
    f.rng.bothify(&pattern).to_uppercase()
}

// ---------------------------------------------------------------------------
// Dispatch
// ---------------------------------------------------------------------------

/// Return `Some(value)` for a formatter this module implements, else `None`.
pub fn dispatch(f: &Faker, locale: &str, name: &str) -> Option<String> {
    Some(match name {
        "safe_domain_name" => safe_domain_name(f),
        "free_email_domain" => free_email_domain(f),
        "tld" => tld(f),
        "domain_word" => domain_word(f, locale),
        "domain_name" => domain_name(f, locale),
        "user_name" => user_name(f, locale),
        "email" => email(f, locale),
        "safe_email" => safe_email(f, locale),
        "free_email" => free_email(f, locale),
        "company_email" => company_email(f, locale),
        "ascii_email" => ascii_email(f, locale),
        "ascii_safe_email" => ascii_safe_email(f, locale),
        "ascii_free_email" => ascii_free_email(f, locale),
        "ascii_company_email" => ascii_company_email(f, locale),
        "hostname" => hostname(f, locale),
        "dga" => dga(f),
        "http_method" => http_method(f),
        "http_status_code" => http_status_code(f),
        "http_status_code_assigned" => http_status_code_assigned(f),
        "url" => url(f, locale),
        "ipv4" => ipv4(f),
        "ipv4_private" => ipv4_private(f),
        "ipv4_public" => ipv4_public(f),
        "ipv4_network_class" => ipv4_network_class(f),
        "ipv6" => ipv6(f),
        "mac_address" => mac_address(f),
        "port_number" => port_number(f),
        "port_number_system" => port_number_system(f),
        "port_number_user" => port_number_user(f),
        "port_number_dynamic" => port_number_dynamic(f),
        "uri_page" => uri_page(f),
        "uri_path" => uri_path(f),
        "uri_extension" => uri_extension(f),
        "uri" => uri(f, locale),
        "slug" => slug(f, locale),
        "image_url" => image_url(f),
        "iana_id" => iana_id(f),
        "ripe_id" => ripe_id(f),
        "nic_handle" => nic_handle(f),
        _ => return None,
    })
}
