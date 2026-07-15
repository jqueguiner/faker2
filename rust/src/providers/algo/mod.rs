//! Hand-written algorithmic formatters, dispatched by name.
//! Each submodule ports one provider's ALGORITHMIC formatters to Rust.
use crate::faker::Faker;

pub mod address;
pub mod automotive;
pub mod bank;
pub mod barcode;
pub mod color;
pub mod company;
pub mod credit_card;
pub mod currency;
pub mod date_time;
pub mod file;
pub mod internet;
pub mod isbn;
pub mod misc;
pub mod phone_number;
pub mod python;
pub mod sbn;

/// Try each provider module; first Some wins.
pub fn dispatch(f: &Faker, locale: &str, name: &str) -> Option<String> {
    None.or_else(|| barcode::dispatch(f, locale, name))
        .or_else(|| credit_card::dispatch(f, locale, name))
        .or_else(|| bank::dispatch(f, locale, name))
        .or_else(|| color::dispatch(f, locale, name))
        .or_else(|| currency::dispatch(f, locale, name))
        .or_else(|| company::dispatch(f, locale, name))
        .or_else(|| automotive::dispatch(f, locale, name))
        .or_else(|| internet::dispatch(f, locale, name))
        .or_else(|| python::dispatch(f, locale, name))
        .or_else(|| misc::dispatch(f, locale, name))
        .or_else(|| file::dispatch(f, locale, name))
        .or_else(|| isbn::dispatch(f, locale, name))
        .or_else(|| sbn::dispatch(f, locale, name))
        .or_else(|| phone_number::dispatch(f, locale, name))
        .or_else(|| address::dispatch(f, locale, name))
        .or_else(|| date_time::dispatch(f, locale, name))
}
