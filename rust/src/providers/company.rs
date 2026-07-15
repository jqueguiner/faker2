//! Company provider — mirrors `faker2.providers.company` (en_US).

use super::data::*;
use crate::faker::Faker;

impl Faker {
    pub fn company_suffix(&self) -> &'static str {
        self.rng.choice_str(COMPANY_SUFFIXES)
    }
    /// One of the `company` formats, e.g. `{{last_name}} {{company_suffix}}`.
    pub fn company(&self) -> String {
        let fmt = self.rng.choice_str(COMPANY_FORMATS);
        self.parse(fmt)
    }
    /// One word from each `catch_phrase_words` list. Mirrors `catch_phrase`.
    pub fn catch_phrase(&self) -> String {
        CATCH_LISTS
            .iter()
            .map(|list| self.rng.choice_str(list))
            .collect::<Vec<_>>()
            .join(" ")
    }
    /// One word from each `bsWords` list. Mirrors `bs`.
    pub fn bs(&self) -> String {
        BS_LISTS
            .iter()
            .map(|list| self.rng.choice_str(list))
            .collect::<Vec<_>>()
            .join(" ")
    }
}
