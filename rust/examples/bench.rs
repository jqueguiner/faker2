//! Rough perf harness. Run: `cargo run --release --features real-names --example bench`
use faker2::{Faker, Gender};
use std::time::Instant;

fn main() {
    let f = Faker::seeded(42);

    // 1) basic name generation (no dataset)
    let n = 1_000_000;
    let t = Instant::now();
    let mut sink = 0usize;
    for _ in 0..n {
        sink += f.name().len();
    }
    let d = t.elapsed();
    println!(
        "basic name(): {n} in {:?}  = {:.2} M ops/s  (sink {})",
        d,
        n as f64 / d.as_secs_f64() / 1e6,
        sink
    );

    // 2) real-names: bank load (one-time) + gender-preserving replacement
    let t = Instant::now();
    let _ = Faker::infer_gender_real("Jacques", Some("FR")); // triggers load
    println!("real bank load: {:?}", t.elapsed());

    let m = 1_000_000;
    let t = Instant::now();
    let mut s2 = 0usize;
    for _ in 0..m {
        s2 += f.first_name_like_real("Jacques", Some("FR")).len();
    }
    let d = t.elapsed();
    println!(
        "first_name_like_real: {m} in {:?}  = {:.2} M ops/s  (sink {})",
        d,
        m as f64 / d.as_secs_f64() / 1e6,
        s2
    );

    // 3) infer only
    let t = Instant::now();
    let mut s3 = 0usize;
    for _ in 0..m {
        if Faker::infer_gender_real("Jacques", Some("FR")) == Gender::Male {
            s3 += 1;
        }
    }
    let d = t.elapsed();
    println!(
        "infer_gender_real: {m} in {:?}  = {:.2} M ops/s  (sink {})",
        d,
        m as f64 / d.as_secs_f64() / 1e6,
        s3
    );
}
