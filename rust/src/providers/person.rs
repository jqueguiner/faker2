//! Person provider — mirrors `faker2.providers.person` (en_US).

use crate::faker::Faker;
use super::data::*;

impl Faker {
    pub fn first_name_male(&self) -> &'static str {
        self.rng.choice_str(FIRST_NAMES_MALE)
    }
    pub fn first_name_female(&self) -> &'static str {
        self.rng.choice_str(FIRST_NAMES_FEMALE)
    }
    pub fn first_name(&self) -> String {
        if self.rng.below(2) == 0 {
            self.first_name_male().to_string()
        } else {
            self.first_name_female().to_string()
        }
    }
    pub fn last_name(&self) -> &'static str {
        self.rng.choice_str(LAST_NAMES)
    }
    pub fn prefix(&self) -> &'static str {
        if self.rng.below(2) == 0 {
            self.rng.choice_str(PREFIXES_MALE)
        } else {
            self.rng.choice_str(PREFIXES_FEMALE)
        }
    }
    pub fn suffix(&self) -> &'static str {
        if SUFFIXES.is_empty() {
            ""
        } else {
            self.rng.choice_str(SUFFIXES)
        }
    }
    /// `{{first_name}} {{last_name}}`.
    pub fn name(&self) -> String {
        format!("{} {}", self.first_name(), self.last_name())
    }
}
