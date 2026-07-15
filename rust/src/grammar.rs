//! English grammatical-number agreement: pluralize / singularize, indefinite
//! article, and count agreement ("1 dog" / "3 dogs", "a apple" -> "an apple").
//!
//! Rule-based with a small irregular table — enough for generated text to read
//! correctly, not a full linguistic engine.

const IRREGULAR: &[(&str, &str)] = &[
    ("man", "men"),
    ("woman", "women"),
    ("child", "children"),
    ("person", "people"),
    ("foot", "feet"),
    ("tooth", "teeth"),
    ("goose", "geese"),
    ("mouse", "mice"),
    ("ox", "oxen"),
];

const UNCOUNTABLE: &[&str] = &[
    "sheep",
    "fish",
    "series",
    "species",
    "money",
    "information",
    "equipment",
    "rice",
];

/// -ves plurals whose singular ends in -fe (not -f).
const VES_TO_FE: &[(&str, &str)] = &[("knives", "knife"), ("wives", "wife"), ("lives", "life")];

fn ends_with(s: &str, suf: &str) -> bool {
    s.len() >= suf.len() && s[s.len() - suf.len()..].eq_ignore_ascii_case(suf)
}

/// Return the plural form of an English noun.
pub fn pluralize(word: &str) -> String {
    let lower = word.to_lowercase();
    if UNCOUNTABLE.contains(&lower.as_str()) {
        return word.to_string();
    }
    for (s, p) in IRREGULAR {
        if lower == *s {
            return p.to_string();
        }
    }
    let n = word.len();
    if ends_with(word, "y") && n >= 2 && !"aeiou".contains(word.as_bytes()[n - 2] as char) {
        return format!("{}ies", &word[..n - 1]);
    }
    if ends_with(word, "s")
        || ends_with(word, "x")
        || ends_with(word, "z")
        || ends_with(word, "ch")
        || ends_with(word, "sh")
    {
        return format!("{word}es");
    }
    if ends_with(word, "f") {
        return format!("{}ves", &word[..n - 1]);
    }
    if ends_with(word, "fe") {
        return format!("{}ves", &word[..n - 2]);
    }
    format!("{word}s")
}

/// Return the singular form of an English noun (inverse of [`pluralize`]).
pub fn singularize(word: &str) -> String {
    let lower = word.to_lowercase();
    if UNCOUNTABLE.contains(&lower.as_str()) {
        return word.to_string();
    }
    for (s, p) in IRREGULAR {
        if lower == *p {
            return s.to_string();
        }
    }
    let n = word.len();
    for (plural, singular) in VES_TO_FE {
        if lower == *plural {
            return singular.to_string();
        }
    }
    if ends_with(word, "ies") && n > 3 {
        return format!("{}y", &word[..n - 3]);
    }
    if ends_with(word, "ves") {
        return format!("{}f", &word[..n - 3]);
    }
    if ends_with(word, "es")
        && (ends_with(&word[..n - 2], "s")
            || ends_with(&word[..n - 2], "x")
            || ends_with(&word[..n - 2], "z")
            || ends_with(&word[..n - 2], "ch")
            || ends_with(&word[..n - 2], "sh"))
    {
        return word[..n - 2].to_string();
    }
    if ends_with(word, "s") && !ends_with(word, "ss") {
        return word[..n - 1].to_string();
    }
    word.to_string()
}

/// "a" or "an" for `word` based on its leading sound (vowel-letter heuristic).
pub fn indefinite_article(word: &str) -> &'static str {
    match word.chars().next().map(|c| c.to_ascii_lowercase()) {
        Some('a') | Some('e') | Some('i') | Some('o') | Some('u') => "an",
        _ => "a",
    }
}

/// Agree a noun with a count: `1 -> "a dog" / "an apple"`, `n -> "3 dogs"`.
pub fn agree(count: i64, singular_noun: &str) -> String {
    if count == 1 {
        format!("{} {}", indefinite_article(singular_noun), singular_noun)
    } else {
        format!("{} {}", count, pluralize(singular_noun))
    }
}

/// Subject-verb agreement for "to be".
pub fn is_are(count: i64) -> &'static str {
    if count == 1 {
        "is"
    } else {
        "are"
    }
}
