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
