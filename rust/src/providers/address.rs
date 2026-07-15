//! Address provider — mirrors `faker2.providers.address` (en_US).

use super::data::*;
use crate::faker::Faker;

impl Faker {
    pub fn building_number(&self) -> String {
        let fmt = self.rng.choice_str(BUILDING_NUMBER_FORMATS);
        self.rng.numerify(fmt)
    }
    pub fn street_suffix(&self) -> &'static str {
        self.rng.choice_str(STREET_SUFFIXES)
    }
    pub fn city_prefix(&self) -> &'static str {
        self.rng.choice_str(CITY_PREFIXES)
    }
    pub fn city_suffix(&self) -> &'static str {
        self.rng.choice_str(CITY_SUFFIXES)
    }
    pub fn state(&self) -> &'static str {
        self.rng.choice_str(STATES)
    }
    pub fn state_abbr(&self) -> &'static str {
        self.rng.choice_str(STATE_ABBR)
    }
    pub fn postcode(&self) -> String {
        self.rng.numerify("#####")
    }
    pub fn country(&self) -> &'static str {
        "United States"
    }
    pub fn secondary_address(&self) -> String {
        let fmt = self.rng.choice_str(&["Apt. ###", "Suite ###"]);
        self.rng.numerify(fmt)
    }
    /// `{{first_name}} {{street_suffix}}` style street name.
    pub fn street_name(&self) -> String {
        let fmt = self.rng.choice_str(STREET_NAME_FORMATS);
        self.parse(fmt)
    }
    /// `{{building_number}} {{street_name}}`.
    pub fn street_address(&self) -> String {
        let fmt = self.rng.choice_str(STREET_ADDRESS_FORMATS);
        self.parse(fmt)
    }
    pub fn city(&self) -> String {
        let fmt = self.rng.choice_str(CITY_FORMATS);
        self.parse(fmt)
    }
    /// Full multi-line US address.
    pub fn address(&self) -> String {
        format!(
            "{}\n{}, {} {}",
            self.street_address(),
            self.city(),
            self.state_abbr(),
            self.postcode()
        )
    }
}
