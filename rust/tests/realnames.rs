#![cfg(feature = "real-names")]
use faker2::{Faker, Gender};

#[test]
fn real_gender_inference() {
    assert_eq!(Faker::infer_gender_real("Jacques", Some("FR")), Gender::Male);
    assert_eq!(Faker::infer_gender_real("Marie", Some("FR")), Gender::Female);
    assert_eq!(Faker::infer_gender_real("Mohammed", Some("EG")), Gender::Male);
    assert_eq!(Faker::infer_gender_real("Zzxq", Some("FR")), Gender::Unknown);
}

#[test]
fn real_replacement_preserves_gender() {
    let f = Faker::seeded(42);
    for name in ["Jacques", "Julien", "Pierre"] {
        for _ in 0..30 {
            let repl = f.first_name_like_real(name, Some("FR"));
            let g = Faker::infer_gender_real(&repl, Some("FR"));
            assert!(
                matches!(g, Gender::Male | Gender::Unisex),
                "{name} -> {repl} became {g:?}"
            );
        }
    }
    let repl = f.first_name_like_real("Marie", Some("FR"));
    assert!(matches!(
        Faker::infer_gender_real(&repl, Some("FR")),
        Gender::Female | Gender::Unisex
    ));
}

#[test]
fn weighted_draw_nonempty() {
    let f = Faker::seeded(1);
    assert!(f.first_name_real(Some("JP"), Gender::Female).is_some());
    assert!(f.first_name_real(Some("BR"), Gender::Male).is_some());
    assert!(f.first_name_real(None, Gender::Male).is_some());
}

#[test]
fn detect_country() {
    let yuki = Faker::detect_country("Yuki", 3);
    assert_eq!(yuki[0].0, "JP");
    assert!(yuki.iter().all(|(_, s)| (0.0..=1.0).contains(s)));
    let bjorn: Vec<String> = Faker::detect_country("Bjorn", 4).into_iter().map(|(c, _)| c).collect();
    assert!(bjorn.contains(&"SE".to_string()) || bjorn.contains(&"NO".to_string()));
    assert!(Faker::detect_country("Zzxqwv", 3).is_empty());
}

fn names(v: Vec<(String, f64)>) -> Vec<String> {
    v.into_iter().map(|(n, _)| n).collect()
}

#[test]
fn homophones_methods() {
    for m in ["metaphone", "ipa", "levenshtein", "balanced"] {
        let h = Faker::homophones("Dominique", "FR", m, 999, true, None);
        assert_eq!(h[0].0, "Dominique", "method {m}");
        let total: f64 = h.iter().map(|(_, p)| p).sum();
        assert!((total - 1.0).abs() < 1e-6, "method {m} sums to {total}");
    }
    // metaphone is coarse (Xavier collides); ipa + balanced drop it
    let meta = names(Faker::homophones("Sophie", "FR", "metaphone", 20, true, None));
    let ipa = names(Faker::homophones("Sophie", "FR", "ipa", 20, true, None));
    let bal = names(Faker::homophones("Sophie", "FR", "balanced", 20, true, None));
    assert!(meta.contains(&"Xavier".to_string()));
    assert!(!ipa.contains(&"Xavier".to_string()));
    assert!(!bal.contains(&"Xavier".to_string()));
    // unknown + exclude-self
    assert!(Faker::homophones("Zzxqwv", "FR", "ipa", 5, true, None).is_empty());
    assert!(!names(Faker::homophones("Dominique", "FR", "ipa", 20, false, None))
        .contains(&"Dominique".to_string()));
}
