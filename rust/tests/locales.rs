#![cfg(feature = "locales")]
use faker2::Faker;

#[test]
fn all_locales_generate_names() {
    let f = Faker::seeded(3);
    let locs = Faker::locales();
    assert!(locs.len() >= 150);
    // name resolves (no leftover {{tokens}}) for nearly every locale
    let clean = locs
        .iter()
        .filter(|l| f.gen(l, "name").map_or(false, |s| !s.contains("{{")))
        .count();
    assert!(clean >= 145, "only {clean} locales produced a clean name");
}

#[test]
fn known_locales_samples() {
    let f = Faker::seeded(1);
    for loc in ["en_US", "fr_FR", "ja_JP", "de_DE", "ru_RU", "zh_CN"] {
        for fmt in ["name", "city", "company", "job"] {
            let v = f.gen(loc, fmt);
            assert!(v.is_some(), "{loc}/{fmt} is None");
            assert!(
                !v.unwrap().contains("{{"),
                "{loc}/{fmt} has unresolved token"
            );
        }
    }
    assert!(f.gen("fr_FR", "nonexistent_formatter").is_none());
}
