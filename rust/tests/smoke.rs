use faker2::{grammar, Faker, Gender, Locale};

#[test]
fn infer_gender_by_locale() {
    assert_eq!(Faker::infer_gender("Jacques", Locale::FrFR), Gender::Male);
    assert_eq!(Faker::infer_gender("Marie", Locale::FrFR), Gender::Female);
    assert_eq!(Faker::infer_gender("Zzxq", Locale::FrFR), Gender::Unknown);
    // Case-insensitive.
    assert_eq!(Faker::infer_gender("jacques", Locale::FrFR), Gender::Male);
}

#[test]
fn replacement_preserves_gender() {
    let f = Faker::seeded(1);
    for name in ["Jacques", "Patrick", "Alain"] {
        for _ in 0..50 {
            let repl = f.first_name_like(name, Locale::FrFR);
            assert_eq!(
                Faker::infer_gender(&repl, Locale::FrFR),
                Gender::Male,
                "{name} -> {repl} changed gender"
            );
            assert_ne!(repl, name, "should not echo the input");
        }
    }
    // Female preserved too.
    let repl = f.first_name_like("Marie", Locale::FrFR);
    assert_eq!(Faker::infer_gender(&repl, Locale::FrFR), Gender::Female);
}

#[test]
fn grammar_agreement() {
    assert_eq!(grammar::pluralize("dog"), "dogs");
    assert_eq!(grammar::pluralize("baby"), "babies");
    assert_eq!(grammar::pluralize("box"), "boxes");
    assert_eq!(grammar::pluralize("knife"), "knives");
    assert_eq!(grammar::pluralize("person"), "people");
    assert_eq!(grammar::singularize("cities"), "city");
    assert_eq!(grammar::singularize("people"), "person");
    assert_eq!(grammar::singularize("knives"), "knife");
    assert_eq!(grammar::agree(1, "apple"), "an apple");
    assert_eq!(grammar::agree(1, "dog"), "a dog");
    assert_eq!(grammar::agree(3, "dog"), "3 dogs");
    assert_eq!(grammar::is_are(1), "is");
    assert_eq!(grammar::is_are(2), "are");
}

#[test]
fn deterministic_with_seed() {
    let a = Faker::seeded(42);
    let b = Faker::seeded(42);
    assert_eq!(a.name(), b.name());
    assert_eq!(a.email(), b.email());
    assert_eq!(a.address(), b.address());
}

#[test]
fn non_empty_output() {
    let f = Faker::seeded(1);
    assert!(!f.name().is_empty());
    assert!(f.email().contains('@'));
    assert!(f.address().contains('\n'));
    assert!(f.uuid4().len() == 36);
    assert!(f.hex_color().starts_with('#'));
}

#[test]
fn no_unresolved_tokens() {
    let f = Faker::seeded(3);
    for _ in 0..200 {
        let a = f.address();
        assert!(!a.contains("{{"), "leaked token in {a:?}");
        let c = f.company();
        assert!(!c.contains("{{"), "leaked token in {c:?}");
    }
}

#[test]
fn primitives() {
    let f = Faker::seeded(9);
    let n = f.pyint(10, 20);
    assert!((10..=20).contains(&n));
    assert_eq!(f.rng.numerify("###").len(), 3);
    let lx = f.rng.lexify("???", b"ab");
    assert_eq!(lx.len(), 3);
    assert!(lx.chars().all(|c| c == 'a' || c == 'b'));
}
