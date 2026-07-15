//! Algorithmic formatters for the `currency` provider (to be ported).
#![allow(unused_variables, clippy::all)]
use crate::faker::Faker;

/// Return `Some(value)` for a formatter this module implements, else `None`.
pub fn dispatch(f: &Faker, locale: &str, name: &str) -> Option<String> {
    let _ = (f, locale);
    None
}
