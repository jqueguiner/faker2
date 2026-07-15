//! PyO3 bindings: expose the Rust `faker2` engine to Python as `faker2._native`.
//!
//! Stateless helpers (gender inference, country detection, homophones) are
//! module functions; generation lives on the `Faker` class (seedable).

use faker2::{Faker as RsFaker, Gender, Locale};
use pyo3::prelude::*;

fn gender_code(g: Gender) -> &'static str {
    match g {
        Gender::Male => "m",
        Gender::Female => "f",
        Gender::Unisex => "u",
        Gender::Unknown => "",
    }
}

fn gender_from(code: Option<&str>) -> Gender {
    match code.map(|c| c.to_ascii_lowercase()) {
        Some(ref c) if c == "m" => Gender::Male,
        Some(ref c) if c == "f" => Gender::Female,
        _ => Gender::Unknown,
    }
}

// ---- stateless module functions -------------------------------------------

/// Infer a name's gender from the real dataset: "m" / "f" / "u" / "" (unknown).
#[pyfunction]
#[pyo3(signature = (name, country=None))]
fn infer_gender_real(name: &str, country: Option<&str>) -> String {
    gender_code(RsFaker::infer_gender_real(name, country)).to_string()
}

/// Rank the countries a first name is most characteristic of (scores sum to 1).
#[pyfunction]
#[pyo3(signature = (name, top=5))]
fn detect_country(name: &str, top: usize) -> Vec<(String, f64)> {
    RsFaker::detect_country(name, top)
}

/// Same-sounding names in a country, with probabilities.
/// method: "metaphone" | "ipa" | "levenshtein" | "balanced".
#[pyfunction]
#[pyo3(signature = (name, country, method="metaphone", top=10, include_self=true, max_distance=None))]
fn homophones(
    name: &str,
    country: &str,
    method: &str,
    top: usize,
    include_self: bool,
    max_distance: Option<usize>,
) -> Vec<(String, f64)> {
    RsFaker::homophones(name, country, method, top, include_self, max_distance)
}

/// ISO codes with name data (139).
#[pyfunction]
fn available_countries() -> Vec<String> {
    RsFaker::available_countries()
}

/// All locales with data (151).
#[pyfunction]
fn locales() -> Vec<String> {
    RsFaker::locales()
}

/// Data-driven formatter names usable via `Faker.gen(locale, name)`.
#[pyfunction]
fn locale_formatters() -> Vec<String> {
    RsFaker::locale_formatters()
}

// ---- seedable generator ----------------------------------------------------

#[pyclass(unsendable)]
struct Faker {
    inner: RsFaker,
}

#[pymethods]
impl Faker {
    #[new]
    #[pyo3(signature = (seed=None))]
    fn new(seed: Option<u64>) -> Self {
        Faker {
            inner: match seed {
                Some(s) => RsFaker::seeded(s),
                None => RsFaker::new(),
            },
        }
    }

    fn seed(&self, seed: u64) {
        self.inner.seed(seed);
    }

    // core providers
    fn name(&self) -> String {
        self.inner.name()
    }
    fn first_name(&self) -> String {
        self.inner.first_name()
    }
    fn last_name(&self) -> String {
        self.inner.last_name().to_string()
    }
    fn email(&self) -> String {
        self.inner.email()
    }
    fn address(&self) -> String {
        self.inner.address()
    }
    fn company(&self) -> String {
        self.inner.company()
    }
    fn phone_number(&self) -> String {
        self.inner.phone_number()
    }
    fn sentence(&self, nb_words: usize) -> String {
        self.inner.sentence(nb_words)
    }

    // data-backed name intelligence
    #[pyo3(signature = (country=None, gender=None))]
    fn first_name_real(&self, country: Option<&str>, gender: Option<&str>) -> Option<String> {
        self.inner.first_name_real(country, gender_from(gender))
    }

    #[pyo3(signature = (name, country=None))]
    fn first_name_like_real(&self, name: &str, country: Option<&str>) -> String {
        self.inner.first_name_like_real(name, country)
    }

    /// Generate a data-driven formatter for any of the 151 locales, e.g.
    /// `f.gen("ja_JP", "name")`, `f.gen("fr_FR", "address")`. Returns None for
    /// unknown or not-yet-ported (algorithmic) formatters.
    fn gen(&self, locale: &str, formatter: &str) -> Option<String> {
        self.inner.gen(locale, formatter)
    }

    /// Gender-pure full name for a locale ("en_US" / "fr_FR").
    #[pyo3(signature = (locale="en_US", gender=None))]
    fn name_of(&self, locale: &str, gender: Option<&str>) -> String {
        let loc = Locale::from_code(locale).unwrap_or(Locale::EnUS);
        let g = match gender_from(gender) {
            Gender::Unknown => Gender::Unknown, // name_of picks randomly
            g => g,
        };
        self.inner.name_of(g, loc)
    }
}

#[pymodule]
fn _native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(infer_gender_real, m)?)?;
    m.add_function(wrap_pyfunction!(detect_country, m)?)?;
    m.add_function(wrap_pyfunction!(homophones, m)?)?;
    m.add_function(wrap_pyfunction!(available_countries, m)?)?;
    m.add_function(wrap_pyfunction!(locales, m)?)?;
    m.add_function(wrap_pyfunction!(locale_formatters, m)?)?;
    m.add_class::<Faker>()?;
    m.add("__doc__", "Rust-backed faker2 engine (PyO3).")?;
    Ok(())
}
