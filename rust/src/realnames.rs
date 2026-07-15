//! Real, data-backed names — the "strong ground" (opt-in `real-names` feature).
//!
//! Loads `data/first_names.parquet` (1.43M names / 139 countries / gender +
//! relative frequency + phonetics) and provides frequency-weighted,
//! gender-preserving generation, country detection, and homophone lookup.
//! Mirrors the Python `faker2.naming.realnames` module.
//!
//! Enable with `--features real-names`. Dataset path overridable via
//! `FAKER2_NAMES_PARQUET`.

use std::collections::HashMap;
use std::sync::OnceLock;

use arrow::array::{Array, DictionaryArray, Float32Array, StringArray};
use arrow::datatypes::Int32Type;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

use crate::faker::Faker;
use crate::gender::Gender;

// ---- homophone method names + balanced tuning -----------------------------
pub const METAPHONE: &str = "metaphone";
pub const IPA: &str = "ipa";
pub const LEVENSHTEIN: &str = "levenshtein";
pub const BALANCED: &str = "balanced";

const BAL_META_BONUS: f64 = 0.05;

/// Per-country balanced config (from data/balanced_params.json, swept offline).
#[derive(Clone, Copy)]
struct BalCfg {
    w_ipa: f64,
    min: f64,
    cap: usize,
}

const BAL_DEFAULT: BalCfg = BalCfg {
    w_ipa: 0.3,
    min: 0.55,
    cap: 2,
};

fn bal_params() -> &'static (HashMap<String, BalCfg>, BalCfg) {
    static P: OnceLock<(HashMap<String, BalCfg>, BalCfg)> = OnceLock::new();
    P.get_or_init(|| {
        let path = format!(
            "{}/../data/balanced_params.json",
            env!("CARGO_MANIFEST_DIR")
        );
        let parse = |v: &serde_json::Value| BalCfg {
            w_ipa: v
                .get("w_ipa")
                .and_then(|x| x.as_f64())
                .unwrap_or(BAL_DEFAULT.w_ipa),
            min: v
                .get("min")
                .and_then(|x| x.as_f64())
                .unwrap_or(BAL_DEFAULT.min),
            cap: v.get("cap").and_then(|x| x.as_u64()).unwrap_or(2) as usize,
        };
        let mut map = HashMap::new();
        let mut default = BAL_DEFAULT;
        if let Ok(txt) = std::fs::read_to_string(&path) {
            if let Ok(j) = serde_json::from_str::<serde_json::Value>(&txt) {
                if let Some(d) = j.get("_default") {
                    default = parse(d);
                }
                if let Some(c) = j.get("countries").and_then(|c| c.as_object()) {
                    for (cc, v) in c {
                        map.insert(cc.clone(), parse(v));
                    }
                }
            }
        }
        (map, default)
    })
}

fn bal_cfg(country: &str) -> BalCfg {
    let (map, default) = bal_params();
    *map.get(country).unwrap_or(default)
}

fn default_dist(method: &str) -> usize {
    match method {
        IPA => 1,
        BALANCED => 1,
        _ => 2, // levenshtein
    }
}

/// Strip IPA stress/length marks and separators before comparing phonemes.
fn norm_ipa(s: &str) -> String {
    s.chars()
        .filter(|c| !matches!(c, 'ˈ' | 'ˌ' | 'ː' | 'ˑ' | ' ' | '.' | '/'))
        .collect()
}

/// Edit distance over chars, short-circuiting once it provably exceeds `cap`.
fn levenshtein(a: &[char], b: &[char], cap: usize) -> usize {
    if a.len().abs_diff(b.len()) > cap {
        return cap + 1;
    }
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    for (i, &ca) in a.iter().enumerate() {
        let mut cur = vec![i + 1];
        let mut best = i + 1;
        for (j, &cb) in b.iter().enumerate() {
            let cost = if ca == cb { 0 } else { 1 };
            let v = (prev[j + 1] + 1).min(cur[j] + 1).min(prev[j] + cost);
            cur.push(v);
            best = best.min(v);
        }
        if best > cap {
            return cap + 1;
        }
        prev = cur;
    }
    prev[b.len()]
}

/// Normalized similarity in [0,1] from edit distance; 0 if either empty.
fn sim(a: &[char], b: &[char]) -> f64 {
    if a.is_empty() || b.is_empty() {
        return 0.0;
    }
    let m = a.len().max(b.len());
    1.0 - levenshtein(a, b, m) as f64 / m as f64
}

fn col_strings(col: &dyn Array) -> Vec<Option<String>> {
    if let Some(a) = col.as_any().downcast_ref::<StringArray>() {
        return (0..a.len())
            .map(|i| {
                if a.is_null(i) {
                    None
                } else {
                    Some(a.value(i).to_string())
                }
            })
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

/// Per-country, per-name row used for ipa/levenshtein/balanced scans.
struct NameRow {
    ascii: Vec<char>,
    ipa: Vec<char>,
    share: f64,
    phon: String,
}

/// In-memory indexes built once from the parquet dataset.
pub struct NameBank {
    /// (country?, gender) -> (names, cumulative *relative* weights).
    pools: HashMap<PoolKey, (Vec<String>, Vec<f64>)>,
    /// (name_lower, country?) -> (male_weight, female_weight).
    gender_by: HashMap<(String, Option<String>), (f64, f64)>,
    /// name_lower -> {country: within-country share} for origin detection.
    country_by: HashMap<String, HashMap<String, f64>>,
    /// (name_lower, country) -> metaphone key.
    phon_key: HashMap<(String, String), String>,
    /// (name_lower, country) -> normalized IPA.
    ipa_key: HashMap<(String, String), String>,
    /// (country, metaphone) -> {name: share}.
    homo: HashMap<(String, String), HashMap<String, f64>>,
    /// country -> {name: NameRow}.
    by_country: HashMap<String, HashMap<String, NameRow>>,
}

impl NameBank {
    fn load() -> Self {
        let path = std::env::var("FAKER2_NAMES_PARQUET").unwrap_or_else(|_| {
            format!("{}/../data/first_names.parquet", env!("CARGO_MANIFEST_DIR"))
        });
        let file = std::fs::File::open(&path).unwrap_or_else(|e| panic!("cannot open {path}: {e}"));
        let reader = ParquetRecordBatchReaderBuilder::try_new(file)
            .expect("parquet open")
            .build()
            .expect("parquet reader");

        let mut pools_acc: HashMap<PoolKey, (Vec<String>, Vec<f64>)> = HashMap::new();
        let mut gender_by: HashMap<(String, Option<String>), (f64, f64)> = HashMap::new();
        let mut country_by: HashMap<String, HashMap<String, f64>> = HashMap::new();
        let mut phon_key: HashMap<(String, String), String> = HashMap::new();
        let mut ipa_key: HashMap<(String, String), String> = HashMap::new();
        let mut homo: HashMap<(String, String), HashMap<String, f64>> = HashMap::new();
        let mut by_country: HashMap<String, HashMap<String, NameRow>> = HashMap::new();

        for batch in reader {
            let batch = batch.expect("record batch");
            let schema = batch.schema();
            let idx = |n: &str| schema.index_of(n).unwrap();
            let names = col_strings(batch.column(idx("name")).as_ref());
            let asciis = col_strings(batch.column(idx("name_ascii")).as_ref());
            let ccs = col_strings(batch.column(idx("country_code")).as_ref());
            let genders = col_strings(batch.column(idx("gender")).as_ref());
            let freqs = col_f32(batch.column(idx("frequency")).as_ref());
            let shares = col_f32(batch.column(idx("country_share")).as_ref());
            let phonetics = col_strings(batch.column(idx("phonetic")).as_ref());
            let ipas = col_strings(batch.column(idx("ipa")).as_ref());

            for i in 0..names.len() {
                let g = match &genders[i] {
                    Some(s) if s == "m" || s == "f" => s.chars().next().unwrap(),
                    _ => continue,
                };
                let name = match &names[i] {
                    Some(n) => n.clone(),
                    None => continue,
                };
                let w = freqs[i]
                    .filter(|v| *v > 0.0)
                    .map(|v| v as f64)
                    .unwrap_or(1e-6);
                let csh = shares[i]
                    .filter(|v| *v > 0.0)
                    .map(|v| v as f64)
                    .unwrap_or(1e-9);
                let cc = ccs[i].clone();
                let phon = phonetics[i].clone().unwrap_or_default();
                let ipa_n = ipas[i].as_deref().map(norm_ipa).unwrap_or_default();

                // weighted-draw pools (per-country + global)
                for scope in [cc.clone(), None] {
                    let e = pools_acc.entry((scope, g)).or_default();
                    e.0.push(name.clone());
                    let last = e.1.last().copied().unwrap_or(0.0);
                    e.1.push(last + w);
                }

                // homophone group + per-country row
                if let Some(ref c) = cc {
                    if !phon.is_empty() {
                        *homo
                            .entry((c.clone(), phon.clone()))
                            .or_default()
                            .entry(name.clone())
                            .or_insert(0.0) += csh;
                    }
                    let table = by_country.entry(c.clone()).or_default();
                    table
                        .entry(name.clone())
                        .and_modify(|r| r.share += csh)
                        .or_insert_with(|| NameRow {
                            ascii: asciis[i]
                                .as_ref()
                                .unwrap_or(&name)
                                .to_lowercase()
                                .chars()
                                .collect(),
                            ipa: ipa_n.chars().collect(),
                            share: csh,
                            phon: phon.clone(),
                        });
                }

                // name-keyed indexes under accented + ascii forms
                let mut keys = vec![name.to_lowercase()];
                let ak = asciis[i].as_ref().unwrap_or(&name).to_lowercase();
                if ak != keys[0] {
                    keys.push(ak);
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
                    if let Some(ref c) = cc {
                        *country_by
                            .entry(key.clone())
                            .or_default()
                            .entry(c.clone())
                            .or_insert(0.0) += w;
                        if !phon.is_empty() {
                            phon_key.insert((key.clone(), c.clone()), phon.clone());
                        }
                        if !ipa_n.is_empty() {
                            ipa_key.insert((key.clone(), c.clone()), ipa_n.clone());
                        }
                    }
                }
            }
        }

        NameBank {
            pools: pools_acc,
            gender_by,
            country_by,
            phon_key,
            ipa_key,
            homo,
            by_country,
        }
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
                    if m.min(f) >= 0.20 * (m + f) {
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

    fn draw(
        &self,
        rng: &crate::rng::Rng,
        country: Option<&str>,
        g: char,
        avoid: Option<&str>,
    ) -> Option<String> {
        let pool = country
            .and_then(|c| self.pools.get(&(Some(c.to_string()), g)))
            .or_else(|| self.pools.get(&(None, g)))?;
        let (names, cum) = pool;
        let total = *cum.last()?;
        let avoid_l = avoid.map(|s| s.trim().to_lowercase());
        let mut pick = &names[0];
        for _ in 0..8 {
            let r = rng.unit() * total;
            let idx = cum.partition_point(|&c| c <= r).min(names.len() - 1);
            pick = &names[idx];
            if avoid_l.as_deref() != Some(pick.to_lowercase().as_str()) {
                return Some(pick.clone());
            }
        }
        Some(pick.clone())
    }

    fn detect_country(&self, name: &str, top: usize) -> Vec<(String, f64)> {
        let counts = match self.country_by.get(&name.trim().to_lowercase()) {
            Some(c) => c,
            None => return vec![],
        };
        let total: f64 = counts.values().sum::<f64>().max(1e-12);
        let mut ranked: Vec<(String, f64)> =
            counts.iter().map(|(c, w)| (c.clone(), w / total)).collect();
        ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        ranked.truncate(top);
        ranked
    }

    fn homophones(
        &self,
        name: &str,
        country: &str,
        method: &str,
        top: usize,
        include_self: bool,
        max_distance: Option<usize>,
    ) -> Vec<(String, f64)> {
        let key = name.trim().to_lowercase();
        let mut items: Vec<(String, f64)> = match method {
            METAPHONE => self
                .phon_key
                .get(&(key.clone(), country.to_string()))
                .and_then(|p| self.homo.get(&(country.to_string(), p.clone())))
                .map(|g| g.iter().map(|(n, w)| (n.clone(), *w)).collect())
                .unwrap_or_default(),
            BALANCED => self.balanced(&key, country, max_distance),
            IPA | LEVENSHTEIN => self.fuzzy(&key, country, method, max_distance),
            _ => panic!("unknown method {method:?}"),
        };
        if !include_self {
            items.retain(|(n, _)| n.to_lowercase() != key);
        }
        if items.is_empty() {
            return vec![];
        }
        let total: f64 = items.iter().map(|(_, w)| *w).sum::<f64>().max(1e-12);
        items.iter_mut().for_each(|(_, w)| *w /= total);
        items.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        items.truncate(top);
        items
    }

    fn fuzzy(
        &self,
        key: &str,
        country: &str,
        method: &str,
        max_distance: Option<usize>,
    ) -> Vec<(String, f64)> {
        let table = match self.by_country.get(country) {
            Some(t) => t,
            None => return vec![],
        };
        let cap = max_distance.unwrap_or_else(|| default_dist(method));
        let query: Vec<char> = if method == IPA {
            match self.ipa_key.get(&(key.to_string(), country.to_string())) {
                Some(q) => q.chars().collect(),
                None => return vec![],
            }
        } else {
            key.chars().collect()
        };
        if query.is_empty() {
            return vec![];
        }
        let mut out = vec![];
        for (nm, row) in table {
            let target = if method == IPA { &row.ipa } else { &row.ascii };
            if !target.is_empty() && levenshtein(&query, target, cap) <= cap {
                out.push((nm.clone(), row.share));
            }
        }
        out
    }

    fn balanced(
        &self,
        key: &str,
        country: &str,
        max_distance: Option<usize>,
    ) -> Vec<(String, f64)> {
        let table = match self.by_country.get(country) {
            Some(t) => t,
            None => return vec![],
        };
        let q_ipa: Vec<char> = self
            .ipa_key
            .get(&(key.to_string(), country.to_string()))
            .map(|s| s.chars().collect())
            .unwrap_or_default();
        let q_phon = self
            .phon_key
            .get(&(key.to_string(), country.to_string()))
            .cloned()
            .unwrap_or_default();
        let q_ascii: Vec<char> = key.chars().collect();
        let cfg = bal_cfg(country);
        let (w_ipa, w_spell, bal_min) = (cfg.w_ipa, 1.0 - cfg.w_ipa, cfg.min);
        let cap = max_distance.unwrap_or(cfg.cap);

        // candidate pool: metaphone group ∪ IPA-close names
        let mut pool: std::collections::HashSet<String> = std::collections::HashSet::new();
        if !q_phon.is_empty() {
            if let Some(g) = self.homo.get(&(country.to_string(), q_phon.clone())) {
                pool.extend(g.keys().cloned());
            }
        }
        if !q_ipa.is_empty() {
            for (nm, row) in table {
                if !row.ipa.is_empty() && levenshtein(&q_ipa, &row.ipa, cap) <= cap {
                    pool.insert(nm.clone());
                }
            }
        }

        let mut out = vec![];
        for nm in pool {
            let row = match table.get(&nm) {
                Some(r) => r,
                None => continue,
            };
            let ipa_sim = sim(&q_ipa, &row.ipa);
            let spell_sim = sim(&q_ascii, &row.ascii);
            let meta = if !q_phon.is_empty() && row.phon == q_phon {
                1.0
            } else {
                0.0
            };
            let base = if !q_ipa.is_empty() && !row.ipa.is_empty() {
                w_ipa * ipa_sim + w_spell * spell_sim
            } else {
                0.5 * spell_sim + 0.5 * meta
            };
            let score = (base + BAL_META_BONUS * meta).min(1.0);
            if score >= bal_min || nm.to_lowercase() == key {
                out.push((nm, row.share * score));
            }
        }
        out
    }

    fn countries(&self) -> Vec<String> {
        let mut c: Vec<String> = self
            .pools
            .keys()
            .filter_map(|(cc, _g)| cc.clone())
            .collect();
        c.sort();
        c.dedup();
        c
    }
}

fn bank() -> &'static NameBank {
    static BANK: OnceLock<NameBank> = OnceLock::new();
    BANK.get_or_init(NameBank::load)
}

impl Faker {
    /// Infer a name's gender from the real 139-country dataset.
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

    /// Rank the countries a first name is most characteristic of (scores sum to 1).
    pub fn detect_country(name: &str, top: usize) -> Vec<(String, f64)> {
        bank().detect_country(name, top)
    }

    /// ISO codes with name data (139). Mirrors Python `available_countries()`.
    pub fn available_countries() -> Vec<String> {
        bank().countries()
    }

    /// Same-sounding names in a country with probabilities.
    ///
    /// `method`: `"metaphone"` | `"ipa"` | `"levenshtein"` | `"balanced"`.
    pub fn homophones(
        name: &str,
        country: &str,
        method: &str,
        top: usize,
        include_self: bool,
        max_distance: Option<usize>,
    ) -> Vec<(String, f64)> {
        bank().homophones(name, country, method, top, include_self, max_distance)
    }
}
