//! Lorem provider — mirrors `faker2.providers.lorem` (en_US word list).

use super::data::LOREM_WORDS;
use crate::faker::Faker;

impl Faker {
    pub fn word(&self) -> &'static str {
        self.rng.choice_str(LOREM_WORDS)
    }
    pub fn words(&self, nb: usize) -> Vec<&'static str> {
        (0..nb).map(|_| self.word()).collect()
    }
    /// A capitalised sentence of ~`nb_words` words ending in a period.
    pub fn sentence(&self, nb_words: usize) -> String {
        // Faker varies the count by +/-40%.
        let lo = ((nb_words as f64) * 0.6).round().max(1.0) as usize;
        let hi = ((nb_words as f64) * 1.4).round().max(lo as f64) as usize;
        let n = self.rng.random_int(lo as i64, hi as i64, 1) as usize;
        let words = self.words(n.max(1));
        let joined = words.join(" ");
        let mut s: Vec<char> = joined.chars().collect();
        if let Some(first) = s.first_mut() {
            *first = first.to_ascii_uppercase();
        }
        let mut out: String = s.into_iter().collect();
        out.push('.');
        out
    }
    pub fn sentences(&self, nb: usize) -> Vec<String> {
        (0..nb).map(|_| self.sentence(6)).collect()
    }
    /// A paragraph of ~`nb_sentences` sentences.
    pub fn paragraph(&self, nb_sentences: usize) -> String {
        self.sentences(nb_sentences.max(1)).join(" ")
    }
    pub fn text(&self, nb_paragraphs: usize) -> String {
        (0..nb_paragraphs.max(1))
            .map(|_| self.paragraph(3))
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}
