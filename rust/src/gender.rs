//! Gender inference + gender-preserving name replacement.
//!
//! Reverse-indexes each locale's `first_names_male` / `first_names_female`
//! lists (from the Python `faker2` data) into a name -> gender lookup, then
//! generates a replacement name of the *same* inferred gender and locale:
//!
//! ```
//! use faker2::{Faker, Gender, Locale};
//! let f = Faker::seeded(1);
//! assert_eq!(Faker::infer_gender("Jacques", Locale::FrFR), Gender::Male);
//! let repl = f.first_name_like("Jacques", Locale::FrFR); // another male FR name
//! assert_eq!(Faker::infer_gender(&repl, Locale::FrFR), Gender::Male);
//! ```

use crate::faker::Faker;
use crate::providers::data::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Gender {
    Male,
    Female,
    /// Name appears in both male and female pools (unisex).
    Unisex,
    /// Name not found in the locale's pools.
    Unknown,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Locale {
    EnUS,
    FrFR,
}

impl Locale {
    /// `(male_names, female_names)` for the locale.
    pub fn first_names(self) -> (&'static [&'static str], &'static [&'static str]) {
        match self {
            Locale::EnUS => (FIRST_NAMES_MALE, FIRST_NAMES_FEMALE),
            Locale::FrFR => (FR_FIRST_NAMES_MALE, FR_FIRST_NAMES_FEMALE),
        }
    }

    pub fn last_names(self) -> &'static [&'static str] {
        match self {
            Locale::EnUS => LAST_NAMES,
            Locale::FrFR => FR_LAST_NAMES,
        }
    }

    /// Parse a locale code like `"fr"`, `"fr_FR"`, `"en-US"`.
    pub fn from_code(code: &str) -> Option<Locale> {
        match code.to_lowercase().replace('-', "_").as_str() {
            "fr" | "fr_fr" => Some(Locale::FrFR),
            "en" | "en_us" => Some(Locale::EnUS),
            _ => None,
        }
    }
}

fn contains_ci(list: &[&str], name: &str) -> bool {
    list.iter().any(|n| n.eq_ignore_ascii_case(name))
}

impl Faker {
    /// Infer the gender of `name` within `locale` by dictionary lookup.
    /// Case-insensitive. Returns [`Gender::Unknown`] if not found.
    pub fn infer_gender(name: &str, locale: Locale) -> Gender {
        let name = name.trim();
        let (male, female) = locale.first_names();
        let m = contains_ci(male, name);
        let f = contains_ci(female, name);
        match (m, f) {
            (true, false) => Gender::Male,
            (false, true) => Gender::Female,
            (true, true) => Gender::Unisex,
            (false, false) => Gender::Unknown,
        }
    }

    /// Pick a random first name of the given gender/locale.
    /// [`Gender::Unisex`]/[`Gender::Unknown`] draw from either pool.
    pub fn first_name_of(&self, gender: Gender, locale: Locale) -> &'static str {
        let (male, female) = locale.first_names();
        match gender {
            Gender::Male => self.rng.choice_str(male),
            Gender::Female => self.rng.choice_str(female),
            _ => {
                if self.rng.below(2) == 0 {
                    self.rng.choice_str(male)
                } else {
                    self.rng.choice_str(female)
                }
            }
        }
    }

    /// Replace `name` with a different name of the **same inferred gender**
    /// and locale — the headline feature (`first_name_like("Jacques", fr)`).
    pub fn first_name_like(&self, name: &str, locale: Locale) -> String {
        let g = Self::infer_gender(name, locale);
        // Avoid returning the exact same name when the pool allows.
        for _ in 0..8 {
            let candidate = self.first_name_of(g, locale);
            if !candidate.eq_ignore_ascii_case(name.trim()) {
                return candidate.to_string();
            }
        }
        self.first_name_of(g, locale).to_string()
    }

    /// Full name of a fixed gender, with prefix/suffix agreeing in gender
    /// (en_US). Mirrors `formats_male` / `formats_female`.
    pub fn name_of(&self, gender: Gender, locale: Locale) -> String {
        let g = match gender {
            Gender::Male | Gender::Female => gender,
            _ => {
                if self.rng.below(2) == 0 {
                    Gender::Male
                } else {
                    Gender::Female
                }
            }
        };
        let first = self.first_name_of(g, locale);
        let last = self.rng.choice_str(locale.last_names());
        // ~2% prefix, ~2.5% suffix, matching the Python format weights.
        let roll = self.rng.random_int(1, 1000, 1);
        let (pre_m, pre_f): (&[&str], &[&str]) = (PREFIXES_MALE, PREFIXES_FEMALE);
        let (suf_m, suf_f): (&[&str], &[&str]) = (SUFFIXES_MALE, SUFFIXES_FEMALE);
        let prefix = if roll <= 20 {
            let p = if g == Gender::Male { pre_m } else { pre_f };
            if p.is_empty() { "" } else { self.rng.choice_str(p) }
        } else {
            ""
        };
        let suffix = if roll > 20 && roll <= 45 {
            let s = if g == Gender::Male { suf_m } else { suf_f };
            if s.is_empty() { "" } else { self.rng.choice_str(s) }
        } else {
            ""
        };
        let mut out = String::new();
        if !prefix.is_empty() {
            out.push_str(prefix);
            out.push(' ');
        }
        out.push_str(first);
        out.push(' ');
        out.push_str(last);
        if !suffix.is_empty() {
            out.push(' ');
            out.push_str(suffix);
        }
        out
    }
}
