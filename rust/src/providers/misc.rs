//! Phone, color, python/misc providers.

use super::data::*;
use crate::faker::Faker;

impl Faker {
    // ---- phone_number ------------------------------------------------------
    pub fn phone_number(&self) -> String {
        let fmt = self.rng.choice_str(PHONE_FORMATS);
        self.rng.numerify(fmt)
    }

    // ---- color -------------------------------------------------------------
    pub fn color_name(&self) -> &'static str {
        self.rng.choice_str(SAFE_COLOR_NAMES)
    }
    pub fn hex_color(&self) -> String {
        format!("#{}", self.rng.hexify("^^^^^^", false))
    }
    pub fn rgb_color(&self) -> String {
        format!(
            "{},{},{}",
            self.rng.random_int(0, 255, 1),
            self.rng.random_int(0, 255, 1),
            self.rng.random_int(0, 255, 1)
        )
    }

    // ---- python / misc -----------------------------------------------------
    pub fn boolean(&self, chance_true: u8) -> bool {
        (self.rng.random_int(1, 100, 1) as u8) <= chance_true.min(100)
    }
    pub fn pyint(&self, min: i64, max: i64) -> i64 {
        self.rng.random_int(min, max, 1)
    }
    pub fn pyfloat(&self) -> f64 {
        self.rng.unit()
    }
    /// RFC-4122-ish v4 UUID (uses this crate's PRNG, not crypto-strong).
    pub fn uuid4(&self) -> String {
        let h = |n: usize, f: &Faker| (0..n).map(|_| f.rng.hexify("^", false)).collect::<String>();
        format!(
            "{}-{}-4{}-{}{}-{}",
            h(8, self),
            h(4, self),
            h(3, self),
            self.rng.choice_str(&["8", "9", "a", "b"]),
            h(3, self),
            h(12, self)
        )
    }
}
