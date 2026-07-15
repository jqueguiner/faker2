//! # faker2 (Rust)
//!
//! A Rust port of the [`faker2`](../..) Python fake-data generator.
//!
//! This port implements the core engine (a seedable PRNG + the `BaseProvider`
//! primitives: `numerify`, `lexify`, `bothify`, `hexify`, `random_element`,
//! `random_int`) and the most-used providers for the `en_US` locale: person,
//! address, internet, lorem, company, phone_number, color, and misc/python.
//!
//! Data lists in [`providers::data`] are extracted verbatim from the Python
//! source. Seeding is deterministic within this implementation but is *not*
//! byte-identical to Python's Mersenne-Twister sequence.
//!
//! ```
//! use faker2::Faker;
//! let fake = Faker::seeded(42);
//! let name = fake.name();
//! assert!(!name.is_empty());
//! ```

mod faker;
mod gender;
pub mod grammar;
mod rng;
pub mod providers;

#[cfg(feature = "real-names")]
mod realnames;

pub use faker::Faker;
pub use gender::{Gender, Locale};
pub use rng::Rng;
