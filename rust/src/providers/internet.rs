//! Internet provider — mirrors `faker2.providers.internet`.

use crate::faker::Faker;
use super::data::*;

impl Faker {
    fn slugify(s: &str) -> String {
        s.chars()
            .filter_map(|c| {
                if c.is_ascii_alphanumeric() {
                    Some(c.to_ascii_lowercase())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn free_email_domain(&self) -> &'static str {
        self.rng.choice_str(FREE_EMAIL_DOMAINS)
    }
    pub fn tld(&self) -> &'static str {
        self.rng.choice_str(TLDS)
    }
    pub fn domain_word(&self) -> String {
        Self::slugify(self.last_name())
    }
    pub fn domain_name(&self) -> String {
        format!("{}.{}", self.domain_word(), self.tld())
    }
    /// One of Faker's `user_name_formats`, slugified.
    pub fn user_name(&self) -> String {
        let first = Self::slugify(&self.first_name());
        let last = Self::slugify(self.last_name());
        match self.rng.below(4) {
            0 => format!("{}.{}", first, last),
            1 => format!("{}.{}", last, first),
            2 => format!("{}{}", first, self.rng.random_int(1900, 2020, 1)),
            _ => format!("{}{}", first, last),
        }
    }
    pub fn email(&self) -> String {
        format!("{}@{}", self.user_name(), self.domain_name())
    }
    pub fn free_email(&self) -> String {
        format!("{}@{}", self.user_name(), self.free_email_domain())
    }
    pub fn url(&self) -> String {
        let scheme = if self.rng.below(2) == 0 { "http" } else { "https" };
        format!("{}://www.{}/", scheme, self.domain_name())
    }
    pub fn ipv4(&self) -> String {
        format!(
            "{}.{}.{}.{}",
            self.rng.random_int(1, 255, 1),
            self.rng.random_int(0, 255, 1),
            self.rng.random_int(0, 255, 1),
            self.rng.random_int(1, 254, 1)
        )
    }
    pub fn mac_address(&self) -> String {
        let mut parts = Vec::with_capacity(6);
        for _ in 0..6 {
            parts.push(self.rng.hexify("^^", false));
        }
        parts.join(":")
    }
}
