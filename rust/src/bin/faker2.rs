//! Minimal CLI, mirroring `python -m faker2`.
//!
//! Usage:
//!   faker2 [--seed N] [--repeat N] [FAKE]
//!   faker2 --list
//!
//! FAKE is a formatter name (default: `address`), e.g. `name`, `email`, `text`.

use faker2::{Faker, Locale};

const FORMATTERS: &[&str] = &[
    "name",
    "first_name",
    "last_name",
    "prefix",
    "suffix",
    "address",
    "street_address",
    "street_name",
    "city",
    "state",
    "state_abbr",
    "postcode",
    "country",
    "email",
    "free_email",
    "user_name",
    "domain_name",
    "url",
    "ipv4",
    "mac_address",
    "word",
    "sentence",
    "paragraph",
    "company",
    "company_suffix",
    "catch_phrase",
    "bs",
    "phone_number",
    "color_name",
    "hex_color",
    "uuid4",
];

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut seed: Option<u64> = None;
    let mut repeat: usize = 1;
    let mut fake = "address".to_string();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--list" | "-l" => {
                for f in FORMATTERS {
                    println!("{f}");
                }
                return;
            }
            "-h" | "--help" => {
                eprintln!(
                    "Usage: faker2 [--seed N] [--repeat N] [FAKE]\n       \
                     faker2 --list\n       \
                     faker2 [--seed N] like NAME LOCALE   # gender-preserving replacement"
                );
                return;
            }
            "like" => {
                let name = args.get(i + 1).cloned().unwrap_or_default();
                let loc_code = args.get(i + 2).map(|s| s.as_str()).unwrap_or("en_US");
                let locale = Locale::from_code(loc_code).unwrap_or_else(|| {
                    eprintln!("Unknown locale {loc_code:?} (try en_US, fr_FR).");
                    std::process::exit(1);
                });
                let f = match seed {
                    Some(s) => Faker::seeded(s),
                    None => Faker::new(),
                };
                let g = Faker::infer_gender(&name, locale);
                for _ in 0..repeat.max(1) {
                    println!("{}\t({:?})", f.first_name_like(&name, locale), g);
                }
                return;
            }
            "--seed" | "-s" => {
                i += 1;
                seed = args.get(i).and_then(|v| v.parse().ok());
            }
            "--repeat" | "-r" => {
                i += 1;
                repeat = args.get(i).and_then(|v| v.parse().ok()).unwrap_or(1);
            }
            other => fake = other.to_string(),
        }
        i += 1;
    }

    let f = match seed {
        Some(s) => Faker::seeded(s),
        None => Faker::new(),
    };

    for _ in 0..repeat.max(1) {
        match f.format(&fake) {
            Some(v) => println!("{v}"),
            None => {
                eprintln!("Unknown formatter {fake:?}. Try --list.");
                std::process::exit(1);
            }
        }
    }
}
