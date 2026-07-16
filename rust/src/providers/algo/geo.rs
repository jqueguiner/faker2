//! Geo formatters: coordinates. Mirrors faker2.providers.geo.
#![allow(unused_variables, clippy::all)]
use crate::faker::Faker;

fn coord(f: &Faker, lo: i64, hi: i64) -> String {
    let micro = f.rng.random_int(lo * 1_000_000, hi * 1_000_000, 1);
    let deg = micro / 1_000_000;
    let frac = (micro % 1_000_000).abs();
    format!("{}.{:06}", deg, frac)
}
fn latitude(f: &Faker, _l: &str) -> String {
    coord(f, -90, 90)
}
fn longitude(f: &Faker, _l: &str) -> String {
    coord(f, -180, 180)
}
fn latlng(f: &Faker, l: &str) -> String {
    format!("{},{}", latitude(f, l), longitude(f, l))
}

pub fn dispatch(f: &Faker, locale: &str, name: &str) -> Option<String> {
    Some(match name {
        "latitude" => latitude(f, locale),
        "longitude" => longitude(f, locale),
        "coordinate" => latitude(f, locale),
        "latlng" | "local_latlng" | "location_on_land" => latlng(f, locale),
        _ => return None,
    })
}
