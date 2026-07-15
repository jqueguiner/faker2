//! Algorithmic formatters for the `company` provider.
//!
//! Implements the two ALGORITHMIC formatters from the Python base class:
//!   - `catch_phrase`: one random word from each of the three catch_phrase_words sub-lists, joined with a space.
//!   - `bs`:          one random word from each of the three bsWords sub-lists, joined with a space.
//!
//! The word lists are identical to the Python base-class tuples and are also
//! available as `CATCH_LISTS` / `BS_LISTS` in `providers::data`, but we
//! reference them directly here to keep the algo module self-contained.
#![allow(unused_variables, clippy::all)]
use crate::faker::Faker;
use crate::providers::data::{BS_LISTS, CATCH_LISTS};

// Pick one random element from each sub-list and join with a space.
fn join_lists(f: &Faker, lists: &[&[&str]]) -> String {
    lists
        .iter()
        .map(|list| {
            let idx = f.rng.below(list.len());
            list[idx]
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn catch_phrase(f: &Faker, _locale: &str) -> String {
    join_lists(f, CATCH_LISTS)
}

fn bs(f: &Faker, _locale: &str) -> String {
    join_lists(f, BS_LISTS)
}

/// Return `Some(value)` for a formatter this module implements, else `None`.
pub fn dispatch(f: &Faker, locale: &str, name: &str) -> Option<String> {
    Some(match name {
        "catch_phrase" => catch_phrase(f, locale),
        "bs" => bs(f, locale),
        _ => return None,
    })
}
