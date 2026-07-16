//! Algorithmic/locale-aware formatters for the `lorem` provider.
#![allow(unused_variables, clippy::all)]
use crate::faker::Faker;

/// Pick a random word from the locale's word_list (falls back to "la").
fn pick_word(f: &Faker, locale: &str) -> String {
    // Try the requested locale, then fall back to the lorem default locale "la".
    f.lpick(locale, "lorem", "word_list")
        .or_else(|| f.lpick("la", "lorem", "word_list"))
        .unwrap_or_else(|| "lorem".to_string())
}

/// Pick `n` random words from the locale word list.
fn pick_words(f: &Faker, locale: &str, n: usize) -> Vec<String> {
    (0..n).map(|_| pick_word(f, locale)).collect()
}

/// Randomize a count by +/- 40%, minimum of `min_val`.
/// Mirrors Python's `randomize_nb_elements(nb, min=1)`.
fn randomize_nb(f: &Faker, nb: usize, min_val: usize) -> usize {
    let lo = ((nb as f64) * 0.6).round() as usize;
    let hi = ((nb as f64) * 1.4).round() as usize;
    let lo = lo.max(min_val);
    let hi = hi.max(lo);
    f.rng.random_int(lo as i64, hi as i64, 1) as usize
}

/// Generate a single capitalised sentence of ~`nb_words` words ending in ".".
fn make_sentence(f: &Faker, locale: &str, nb_words: usize, variable: bool) -> String {
    if nb_words == 0 {
        return String::new();
    }
    let n = if variable {
        randomize_nb(f, nb_words, 1)
    } else {
        nb_words.max(1)
    };
    let mut words = pick_words(f, locale, n);
    if let Some(first) = words.first_mut() {
        let mut chars = first.chars();
        if let Some(c) = chars.next() {
            *first = c.to_uppercase().collect::<String>() + chars.as_str();
        }
    }
    let mut out = words.join(" ");
    out.push('.');
    out
}

/// Generate a single paragraph of ~`nb_sentences` sentences joined by a space.
fn make_paragraph(f: &Faker, locale: &str, nb_sentences: usize, variable: bool) -> String {
    if nb_sentences == 0 {
        return String::new();
    }
    let n = if variable {
        randomize_nb(f, nb_sentences, 1)
    } else {
        nb_sentences.max(1)
    };
    (0..n)
        .map(|_| make_sentence(f, locale, 6, true))
        .collect::<Vec<_>>()
        .join(" ")
}

// ── Public formatter functions ────────────────────────────────────────────────

fn word(f: &Faker, locale: &str) -> String {
    pick_word(f, locale)
}

fn words(f: &Faker, locale: &str) -> String {
    // Default nb=3, return space-joined
    let ws = pick_words(f, locale, 3);
    ws.join(" ")
}

fn sentence(f: &Faker, locale: &str) -> String {
    make_sentence(f, locale, 6, true)
}

fn sentences(f: &Faker, locale: &str) -> String {
    // Default nb=3 sentences, newline-separated
    (0..3)
        .map(|_| make_sentence(f, locale, 6, true))
        .collect::<Vec<_>>()
        .join("\n")
}

fn paragraph(f: &Faker, locale: &str) -> String {
    make_paragraph(f, locale, 3, true)
}

fn paragraphs(f: &Faker, locale: &str) -> String {
    // Default nb=3 paragraphs, double-newline-separated
    (0..3)
        .map(|_| make_paragraph(f, locale, 3, true))
        .collect::<Vec<_>>()
        .join("\n\n")
}

/// `text` with default max_nb_chars=200 — mirrors the Python implementation.
fn text(f: &Faker, locale: &str) -> String {
    // max_nb_chars = 200 => use paragraphs path (>= 100)
    let mut parts: Vec<String> = Vec::new();
    let mut size = 0usize;
    loop {
        let para = {
            let prefix = if size > 0 { "\n" } else { "" };
            format!("{}{}", prefix, make_paragraph(f, locale, 3, true))
        };
        if size > 0 && size + para.len() > 200 {
            break;
        }
        size += para.len();
        parts.push(para);
        if size >= 200 {
            break;
        }
    }
    // If we got nothing (unlikely), emit one paragraph
    if parts.is_empty() {
        parts.push(make_paragraph(f, locale, 3, true));
    }
    parts.concat()
}

fn texts(f: &Faker, locale: &str) -> String {
    // Default nb_texts=3, double-newline-separated
    (0..3)
        .map(|_| text(f, locale))
        .collect::<Vec<_>>()
        .join("\n\n")
}

// ── Dispatch ──────────────────────────────────────────────────────────────────

pub fn dispatch(f: &Faker, locale: &str, name: &str) -> Option<String> {
    Some(match name {
        "word" => word(f, locale),
        "words" => words(f, locale),
        "sentence" => sentence(f, locale),
        "sentences" => sentences(f, locale),
        "paragraph" => paragraph(f, locale),
        "paragraphs" => paragraphs(f, locale),
        "text" => text(f, locale),
        "texts" => texts(f, locale),
        _ => return None,
    })
}
