//! Locale-driven generic provider engine (opt-in `locales` feature).
//!
//! Ports the *data-driven* faker2 formatters across all 151 locales by
//! interpreting a declarative recipe map (`data/recipes.json`) against the
//! extracted per-locale data (`data/locales.json`). Covers CHOICE / FORMAT /
//! NUMERIFY / BOTHIFY / CONSTANT recipes — e.g. name, address, city, country,
//! company, phone_number, job, color_name, currency, ssn. ALGORITHMIC
//! formatters (checksums, dates, Luhn, ...) are not handled here yet.
//!
//! Paths overridable via `FAKER2_LOCALES_JSON` / `FAKER2_RECIPES_JSON`.

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::faker::Faker;

type Fields = HashMap<String, Vec<String>>;
type Providers = HashMap<String, Fields>;
type Locales = HashMap<String, Providers>;

struct Recipe {
    provider: String,
    kind: String,
    field: String,
    value: String,
}

struct Engine {
    locales: Locales,
    /// formatter name -> recipe
    recipes: HashMap<String, Recipe>,
}

fn read(path_env: &str, rel: &str) -> String {
    let path = std::env::var(path_env)
        .unwrap_or_else(|_| format!("{}/../data/{}", env!("CARGO_MANIFEST_DIR"), rel));
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("cannot read {path}: {e}"))
}

fn engine() -> &'static Engine {
    static E: OnceLock<Engine> = OnceLock::new();
    E.get_or_init(|| {
        let locales: Locales = serde_json::from_str(&read("FAKER2_LOCALES_JSON", "locales.json"))
            .expect("locales.json");
        let recipes_raw: serde_json::Value =
            serde_json::from_str(&read("FAKER2_RECIPES_JSON", "recipes.json"))
                .expect("recipes.json");
        let mut recipes = HashMap::new();
        if let Some(obj) = recipes_raw.as_object() {
            for (provider, fmts) in obj {
                if let Some(arr) = fmts.as_array() {
                    for f in arr {
                        let name = f.get("name").and_then(|v| v.as_str()).unwrap_or("");
                        let kind = f.get("kind").and_then(|v| v.as_str()).unwrap_or("");
                        if name.is_empty() {
                            continue;
                        }
                        recipes.insert(
                            name.to_string(),
                            Recipe {
                                provider: provider.clone(),
                                kind: kind.to_string(),
                                field: f
                                    .get("field")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("")
                                    .to_string(),
                                value: f
                                    .get("value")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("")
                                    .to_string(),
                            },
                        );
                    }
                }
            }
        }
        Engine { locales, recipes }
    })
}

impl Faker {
    /// Data value list for a (locale, provider, field), falling back to en_US then en.
    fn field_values<'a>(
        e: &'a Engine,
        locale: &str,
        provider: &str,
        field: &str,
    ) -> Option<&'a Vec<String>> {
        for loc in [locale, "en_US", "en"] {
            if let Some(v) = e
                .locales
                .get(loc)
                .and_then(|p| p.get(provider))
                .and_then(|fields| fields.get(field))
            {
                if !v.is_empty() {
                    return Some(v);
                }
            }
        }
        None
    }

    /// Expand `{{token}}` placeholders by resolving each formatter for the locale.
    fn parse_locale(&self, locale: &str, template: &str, depth: u8) -> String {
        if depth > 8 {
            return template.to_string();
        }
        let mut out = String::with_capacity(template.len());
        let b = template.as_bytes();
        let mut i = 0;
        while i < b.len() {
            if i + 1 < b.len() && b[i] == b'{' && b[i + 1] == b'{' {
                if let Some(end) = template[i + 2..].find("}}") {
                    let tok = template[i + 2..i + 2 + end].trim();
                    match self.gen_depth(locale, tok, depth + 1) {
                        Some(v) => out.push_str(&v),
                        None => {
                            out.push_str("{{");
                            out.push_str(tok);
                            out.push_str("}}");
                        }
                    }
                    i = i + 2 + end + 2;
                    continue;
                }
            }
            out.push(b[i] as char);
            i += 1;
        }
        out
    }

    /// Candidate data-field names for a bare `{{token}}` (plural conventions).
    fn field_candidates(token: &str) -> Vec<String> {
        let mut c = vec![token.to_string(), format!("{token}s"), format!("{token}es")];
        for (a, b) in [
            ("_name", "_names"),
            ("prefix", "prefixes"),
            ("suffix", "suffixes"),
        ] {
            if token.contains(a) {
                c.push(token.replacen(a, b, 1));
            }
        }
        if let Some(stem) = token.strip_suffix("name") {
            c.push(format!("{stem}names"));
        }
        if let Some(stem) = token.strip_suffix("company") {
            c.push(format!("{stem}companies"));
        }
        if let Some(stem) = token.strip_suffix('y') {
            c.push(format!("{stem}ies"));
        }
        // irregular / cross-provider aliases
        for (tok, field) in [
            ("city_name", "cities"),
            ("state_name", "states"),
            ("state_abbr", "states_abbr"),
            ("administrative_unit", "states"),
            ("company_category", "company_categories"),
            ("last_name_female", "last_names_female"),
            ("last_name_male", "last_names_male"),
            ("region", "regions"),
        ] {
            if token == tok {
                c.push(field.to_string());
            }
        }
        c
    }

    /// Resolve an unregistered `{{token}}` as a CHOICE over any matching field
    /// in the locale (then en_US fallback). Covers gendered names, city_name,
    /// company_prefix, etc. that live only in locale subclasses.
    fn choice_fallback(&self, locale: &str, token: &str) -> Option<String> {
        let e = engine();
        let cands = Self::field_candidates(token);
        for loc in [locale, "en_US", "en"] {
            if let Some(provs) = e.locales.get(loc) {
                for fields in provs.values() {
                    for cand in &cands {
                        if let Some(v) = fields.get(cand) {
                            if !v.is_empty() {
                                return Some(v[self.rng.below(v.len())].clone());
                            }
                        }
                    }
                }
            }
        }
        None
    }

    fn gen_depth(&self, locale: &str, formatter: &str, depth: u8) -> Option<String> {
        let e = engine();
        let r = match e.recipes.get(formatter) {
            Some(r) => r,
            None => return self.choice_fallback(locale, formatter),
        };
        let pick = |field: &str| -> Option<String> {
            let vals = Self::field_values(e, locale, &r.provider, field)?;
            Some(vals[self.rng.below(vals.len())].clone())
        };
        Some(match r.kind.as_str() {
            "CHOICE" => pick(&r.field)?,
            "CONSTANT" => r.value.clone(),
            "NUMERIFY" => self.rng.numerify(&pick(&r.field)?),
            "BOTHIFY" => self.rng.bothify(&pick(&r.field)?),
            "FORMAT" => {
                let tmpl = pick(&r.field)?;
                self.parse_locale(locale, &tmpl, depth)
            }
            // ALGORITHMIC: handle a few simple transforms, else try a field
            // CHOICE (e.g. first_name_female -> first_names_female); real logic
            // (dates, Luhn, ...) falls through to None.
            _ => {
                if formatter == "postcode" {
                    if let Some(v) = Self::field_values(e, locale, "address", "postcode_formats") {
                        let t = v[self.rng.below(v.len())].clone();
                        return Some(self.rng.bothify(&t).to_uppercase());
                    }
                }
                return self.choice_fallback(locale, formatter);
            }
        })
    }

    /// Generate a data-driven formatter for a locale, e.g.
    /// `f.gen("fr_FR", "name")`, `f.gen("ja_JP", "address")`. Returns `None`
    /// for unknown or ALGORITHMIC formatters.
    pub fn gen(&self, locale: &str, formatter: &str) -> Option<String> {
        self.gen_depth(locale, formatter, 0)
            .or_else(|| crate::providers::algo::dispatch(self, locale, formatter))
    }

    // ---- helpers for hand-written algorithmic providers --------------------

    /// Cloned value list for `(locale, provider, field)`, en_US/en fallback.
    /// Empty vec if the field is absent.
    pub fn lfield(&self, locale: &str, provider: &str, field: &str) -> Vec<String> {
        Self::field_values(engine(), locale, provider, field)
            .cloned()
            .unwrap_or_default()
    }

    /// Random element of `(locale, provider, field)`, or `None` if empty.
    pub fn lpick(&self, locale: &str, provider: &str, field: &str) -> Option<String> {
        let v = self.lfield(locale, provider, field);
        if v.is_empty() {
            None
        } else {
            Some(v[self.rng.below(v.len())].clone())
        }
    }

    /// Expand `{{token}}` placeholders in `template` for `locale`.
    pub fn lparse(&self, locale: &str, template: &str) -> String {
        self.parse_locale(locale, template, 0)
    }

    /// Locales that have data (151).
    pub fn locales() -> Vec<String> {
        let mut v: Vec<String> = engine().locales.keys().cloned().collect();
        v.sort();
        v
    }

    /// Data-driven formatter names available via `gen`.
    pub fn locale_formatters() -> Vec<String> {
        let mut v: Vec<String> = engine()
            .recipes
            .iter()
            .filter(|(_, r)| r.kind != "ALGORITHMIC")
            .map(|(k, _)| k.clone())
            .collect();
        v.sort();
        v
    }
}
