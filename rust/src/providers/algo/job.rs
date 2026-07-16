//! Algorithmic/data formatters for the `job` provider.
#![allow(unused_variables, clippy::all)]
use crate::faker::Faker;

/// Pick from `jobs_female` if the locale has it, otherwise fall back to `jobs`.
fn job_female(f: &Faker, locale: &str) -> String {
    f.lpick(locale, "job", "jobs_female")
        .or_else(|| f.lpick(locale, "job", "jobs"))
        .unwrap_or_default()
}

/// Pick from `jobs_male` if the locale has it, otherwise fall back to `jobs`.
fn job_male(f: &Faker, locale: &str) -> String {
    f.lpick(locale, "job", "jobs_male")
        .or_else(|| f.lpick(locale, "job", "jobs"))
        .unwrap_or_default()
}

pub fn dispatch(f: &Faker, locale: &str, name: &str) -> Option<String> {
    Some(match name {
        "job_female" => job_female(f, locale),
        "job_male" => job_male(f, locale),
        _ => return None,
    })
}
