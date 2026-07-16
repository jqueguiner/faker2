//! Algorithmic/data formatters for the `doi` provider.
#![allow(unused_variables, clippy::all)]
use crate::faker::Faker;

fn doi(f: &Faker, locale: &str) -> String {
    let registrant = f.rng.random_int(1000, 99_999_999, 1);
    let suffix = f.rng.bothify("?#?#?##").to_lowercase();
    format!("10.{}/{}", registrant, suffix)
}

pub fn dispatch(f: &Faker, locale: &str, name: &str) -> Option<String> {
    Some(match name {
        "doi" => doi(f, locale),
        _ => return None,
    })
}
