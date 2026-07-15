//! `Faker` — the aggregate proxy over all providers.
//!
//! Mirrors `faker2.proxy.Faker`: one object exposing every provider method,
//! backed by a single seedable RNG. Provider methods are implemented in
//! `impl Faker` blocks under `src/providers/`.

use crate::rng::Rng;

pub struct Faker {
    pub rng: Rng,
}

impl Faker {
    /// Unseeded (non-deterministic) instance — like `Faker()`.
    pub fn new() -> Self {
        Faker { rng: Rng::new() }
    }

    /// Deterministic instance — like `Faker(); Faker.seed(n)`.
    pub fn seeded(seed: u64) -> Self {
        Faker {
            rng: Rng::seeded(seed),
        }
    }

    /// Re-seed in place — like `fake.seed_instance(n)`.
    pub fn seed(&self, seed: u64) -> &Self {
        self.rng.seed(seed);
        self
    }

    /// Replace `{{token}}` placeholders by calling the matching formatter.
    /// Mirrors `generator.parse()`. Unknown tokens are left untouched.
    pub fn parse(&self, template: &str) -> String {
        let mut out = String::with_capacity(template.len());
        let bytes = template.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            if i + 1 < bytes.len() && bytes[i] == b'{' && bytes[i + 1] == b'{' {
                if let Some(end) = template[i + 2..].find("}}") {
                    let name = template[i + 2..i + 2 + end].trim();
                    match self.format(name) {
                        Some(v) => out.push_str(&v),
                        None => {
                            out.push_str("{{");
                            out.push_str(name);
                            out.push_str("}}");
                        }
                    }
                    i = i + 2 + end + 2;
                    continue;
                }
            }
            out.push(bytes[i] as char);
            i += 1;
        }
        out
    }

    /// Dispatch a formatter name to its method. Mirrors `generator.format()`.
    pub fn format(&self, name: &str) -> Option<String> {
        Some(match name {
            // person
            "name" => self.name(),
            "first_name" => self.first_name(),
            "first_name_male" => self.first_name_male().to_string(),
            "first_name_female" => self.first_name_female().to_string(),
            "last_name" => self.last_name().to_string(),
            "prefix" => self.prefix().to_string(),
            "suffix" => self.suffix().to_string(),
            // address
            "address" => self.address(),
            "street_address" => self.street_address(),
            "street_name" => self.street_name(),
            "street_suffix" => self.street_suffix().to_string(),
            "building_number" => self.building_number(),
            "secondary_address" => self.secondary_address(),
            "city" => self.city(),
            "city_prefix" => self.city_prefix().to_string(),
            "city_suffix" => self.city_suffix().to_string(),
            "state" => self.state().to_string(),
            "state_abbr" => self.state_abbr().to_string(),
            "postcode" => self.postcode(),
            "country" => self.country().to_string(),
            // internet
            "email" => self.email(),
            "free_email" => self.free_email(),
            "user_name" => self.user_name(),
            "domain_name" => self.domain_name(),
            "domain_word" => self.domain_word(),
            "free_email_domain" => self.free_email_domain().to_string(),
            "tld" => self.tld().to_string(),
            "url" => self.url(),
            "ipv4" => self.ipv4(),
            "mac_address" => self.mac_address(),
            // lorem
            "word" => self.word().to_string(),
            "sentence" => self.sentence(6),
            "paragraph" => self.paragraph(3),
            // company
            "company" => self.company(),
            "company_suffix" => self.company_suffix().to_string(),
            "catch_phrase" => self.catch_phrase(),
            "bs" => self.bs(),
            // phone / misc
            "phone_number" => self.phone_number(),
            "color_name" => self.color_name().to_string(),
            "hex_color" => self.hex_color(),
            "uuid4" => self.uuid4(),
            _ => return None,
        })
    }
}

impl Default for Faker {
    fn default() -> Self {
        Faker::new()
    }
}
