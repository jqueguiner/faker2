//! Real, data-backed names — the "strong ground" (opt-in `real-names` feature).
//!
//! Loads `data/first_names.parquet` (1.43M names / 139 countries / gender +
//! real frequency) and provides frequency-weighted, gender-preserving name
//! generation across every country in the dataset. Mirrors the Python
//! `faker2.realnames` module.
//!
//! Enable with `--features real-names`. The dataset path can be overridden with
//! the `FAKER2_NAMES_PARQUET` env var.

use std::collections::HashMap;
use std::sync::OnceLock;

use arrow::array::{Array, DictionaryArray, Float32Array, StringArray};
use arrow::datatypes::Int32Type;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

use crate::faker::Faker;
use crate::gender::Gender;

/// Read one column as owned strings, handling plain Utf8 and dictionary-encoded.
fn col_strings(col: &dyn Array) -> Vec<Option<String>> {
    if let Some(a) = col.as_any().downcast_ref::<StringArray>() {
        return (0..a.len())
            .map(|i| if a.is_null(i) { None } else { Some(a.value(i).to_string()) })
            .collect();
    }
    if let Some(d) = col.as_any().downcast_ref::<DictionaryArray<Int32Type>>() {
        let values = d.values().as_any().downcast_ref::<StringArray>().unwrap();
        let keys = d.keys();
        return (0..d.len())
            .map(|i| {
                if keys.is_null(i) {
                    None
                } else {
                    Some(values.value(keys.value(i) as usize).to_string())
                }
            })
            .collect();
    }
    vec![None; col.len()]
}

fn col_f32(col: &dyn Array) -> Vec<Option<f32>> {
    let a = col.as_any().downcast_ref::<Float32Array>().unwrap();
    (0..a.len())
        .map(|i| if a.is_null(i) { None } else { Some(a.value(i)) })
        .collect()
}

type PoolKey = (Option<String>, char);

/// In-memory indexes built once from the parquet dataset.
pub struct NameBank {
    /// (country?, gender) -> (names, cumulative *relative* weights).
    /// Weights are frequency shares, never raw population counts.
    pools: HashMap<PoolKey, (Vec<String>, Vec<f64>)>,
    /// (name_lower, country?) -> (male_weight, female_weight).
    gender_by: HashMap<(String, Option<String>), (f64, f64)>,
}

impl NameBank {
    fn load() -> Self {
        let path = std::env::var("FAKER2_NAMES_PARQUET").unwrap_or_else(|_| {
            format!("{}/../data/first_names.parquet", env!("CARGO_MANIFEST_DIR"))
        });
        let file = std::fs::File::open(&path)
            .unwrap_or_else(|e| panic!("cannot open {path}: {e}"));
        let reader = ParquetRecordBatchReaderBuilder::try_new(file)
            .expect("parquet open")
            .build()
            .expect("parquet reader");

        let mut acc: HashMap<PoolKey, (Vec<String>, Vec<f64>)> = HashMap::new();
        let mut gender_by: HashMap<(String, Option<String>), (f64, f64)> = HashMap::new();

        for batch in reader {
            let batch = batch.expect("record batch");
            let schema = batch.schema();
            let idx = |n: &str| schema.index_of(n).unwrap();
            let names = col_strings(batch.column(idx("name")).as_ref());
            let asciis = col_strings(batch.column(idx("name_ascii")).as_ref());
            let ccs = col_strings(batch.column(idx("country_code")).as_ref());
            let genders = col_strings(batch.column(idx("gender")).as_ref());
            let freqs = col_f32(batch.column(idx("frequency")).as_ref());

            for i in 0..names.len() {
                let g = match &genders[i] {
                    Some(s) if s == "m" || s == "f" => s.chars().next().unwrap(),
                    _ => continue,
                };
                let name = match &names[i] {
                    Some(n) => n.clone(),
                    None => continue,
                };
                // relative frequency share; tiny default when missing
                let w = freqs[i].filter(|v| *v > 0.0).map(|v| v as f64).unwrap_or(1e-6);
                let cc = ccs[i].clone();
                // per-country and global pools
                for scope in [cc.clone(), None] {
                    let e = acc.entry((scope, g)).or_default();
                    e.0.push(name.clone());
                    let last = e.1.last().copied().unwrap_or(0.0);
                    e.1.push(last + w);
                }
                // index under both accented and ascii forms so lookups round-trip
                let mut keys = vec![name.to_lowercase()];
                let ascii_key = asciis[i].as_ref().unwrap_or(&name).to_lowercase();
                if ascii_key != keys[0] {
                    keys.push(ascii_key);
                }
                for key in keys {
                    for scope in [cc.clone(), None] {
                        let e = gender_by.entry((key.clone(), scope)).or_insert((0.0, 0.0));
                        if g == 'm' {
                            e.0 += w;
                        } else {
                            e.1 += w;
                        }
                    }
                }
            }
        }

        NameBank { pools: acc, gender_by }
    }

    fn infer(&self, name: &str, country: Option<&str>) -> Gender {
        let key = name.trim().to_lowercase();
        let counts = country
            .and_then(|c| self.gender_by.get(&(key.clone(), Some(c.to_string()))))
            .or_else(|| self.gender_by.get(&(key.clone(), None)));
        match counts {
            None => Gender::Unknown,
            Some(&(m, f)) => {
                if m > 0.0 && f > 0.0 {
                    let minority = m.min(f);
                    if minority >= 0.20 * (m + f) {
                        Gender::Unisex
                    } else if m > f {
                        Gender::Male
                    } else {
                        Gender::Female
                    }
                } else if m > 0.0 {
                    Gender::Male
                } else {
                    Gender::Female
                }
            }
        }
    }

    fn draw(&self, rng: &crate::rng::Rng, country: Option<&str>, g: char, avoid: Option<&str>) -> Option<String> {
        let pool = country
            .and_then(|c| self.pools.get(&(Some(c.to_string()), g)))
            .or_else(|| self.pools.get(&(None, g)))?;
        let (names, cum) = pool;
        let total = *cum.last()?;
        let avoid_l = avoid.map(|s| s.trim().to_lowercase());
        let mut pick = &names[0];
        for _ in 0..8 {
            let r = rng.unit() * total;
            // first index whose cumulative weight exceeds r
            let idx = cum.partition_point(|&c| c <= r).min(names.len() - 1);
            pick = &names[idx];
            if avoid_l.as_deref() != Some(pick.to_lowercase().as_str()) {
                return Some(pick.clone());
            }
        }
        Some(pick.clone())
    }
}

fn bank() -> &'static NameBank {
    static BANK: OnceLock<NameBank> = OnceLock::new();
    BANK.get_or_init(NameBank::load)
}

impl Faker {
    /// Infer a name's gender from the real 139-country dataset.
    /// `country` is an ISO-3166 alpha-2 code (e.g. `"FR"`); `None` = global.
    pub fn infer_gender_real(name: &str, country: Option<&str>) -> Gender {
        bank().infer(name, country)
    }

    /// Frequency-weighted first name for a country + gender.
    pub fn first_name_real(&self, country: Option<&str>, gender: Gender) -> Option<String> {
        let g = match gender {
            Gender::Female => 'f',
            Gender::Male => 'm',
            _ => {
                if self.rng.below(2) == 0 {
                    'm'
                } else {
                    'f'
                }
            }
        };
        bank().draw(&self.rng, country, g, None)
    }

    /// Gender-preserving, frequency-weighted replacement using real data.
    /// `first_name_like_real("Jacques", Some("FR"))` -> another male FR name.
    pub fn first_name_like_real(&self, name: &str, country: Option<&str>) -> String {
        let g = match bank().infer(name, country) {
            Gender::Male => 'm',
            Gender::Female => 'f',
            _ => {
                if self.rng.below(2) == 0 {
                    'm'
                } else {
                    'f'
                }
            }
        };
        bank()
            .draw(&self.rng, country, g, Some(name))
            .unwrap_or_else(|| name.to_string())
    }
}
