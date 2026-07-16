//! Algorithmic/data formatters for the `person` provider.
//!
//! Implements gendered/nonbinary name formatters that mirror the Python base class:
//!   name_male, name_female, name_nonbinary — pick a format template and lparse it.
//!   last_name_male, last_name_female, last_name_nonbinary — pick from gendered last-name list.
//!   suffix — pick from suffixes field.
#![allow(unused_variables, clippy::all)]
use crate::faker::Faker;

/// Pick a template from `formats_<gender>` (falling back to `formats`) and lparse it.
fn name_gendered(f: &Faker, locale: &str, gender: &str) -> String {
    let field = format!("formats_{}", gender);
    let formats = f.lfield(locale, "person", &field);
    let tmpl = if formats.is_empty() {
        match f.lpick(locale, "person", "formats") {
            Some(t) => t,
            None => {
                return format!(
                    "{} {}",
                    f.lparse(locale, "{{first_name}}"),
                    f.lparse(locale, "{{last_name}}")
                )
            }
        }
    } else {
        formats[f.rng.below(formats.len())].clone()
    };
    f.lparse(locale, &tmpl)
}

fn name_male(f: &Faker, locale: &str) -> String {
    name_gendered(f, locale, "male")
}

fn name_female(f: &Faker, locale: &str) -> String {
    name_gendered(f, locale, "female")
}

fn name_nonbinary(f: &Faker, locale: &str) -> String {
    name_gendered(f, locale, "nonbinary")
}

/// Pick from `last_names_<gender>`, falling back to `last_names`.
fn last_name_gendered(f: &Faker, locale: &str, gender: &str) -> String {
    let field = format!("last_names_{}", gender);
    if let Some(v) = f.lpick(locale, "person", &field) {
        return v;
    }
    f.lpick(locale, "person", "last_names")
        .unwrap_or_else(|| "Doe".to_string())
}

fn last_name_male(f: &Faker, locale: &str) -> String {
    last_name_gendered(f, locale, "male")
}

fn last_name_female(f: &Faker, locale: &str) -> String {
    last_name_gendered(f, locale, "female")
}

fn last_name_nonbinary(f: &Faker, locale: &str) -> String {
    last_name_gendered(f, locale, "nonbinary")
}

/// Pick from `suffixes`, falling back to empty string.
fn suffix(f: &Faker, locale: &str) -> String {
    f.lpick(locale, "person", "suffixes")
        .unwrap_or_else(|| "".to_string())
}

/// Return `Some(value)` for a formatter this module implements, else `None`.
pub fn dispatch(f: &Faker, locale: &str, name: &str) -> Option<String> {
    Some(match name {
        "name_male" => name_male(f, locale),
        "name_female" => name_female(f, locale),
        "name_nonbinary" => name_nonbinary(f, locale),
        "last_name_male" => last_name_male(f, locale),
        "last_name_female" => last_name_female(f, locale),
        "last_name_nonbinary" => last_name_nonbinary(f, locale),
        "suffix" => suffix(f, locale),
        _ => return None,
    })
}
