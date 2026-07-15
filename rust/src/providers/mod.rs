//! Providers — each module adds an `impl Faker` block, mirroring the
//! `faker2.providers.*` packages (en_US locale).

pub mod data;

mod address;
mod company;
mod internet;
mod lorem;
mod misc;
mod person;
