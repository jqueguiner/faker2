use faker2::Faker;
use std::time::Instant;
fn main() {
    let f = Faker::seeded(42);
    let n = 1_000_000u64;
    for fmt in ["name", "email", "address", "credit_card_number", "iban"] {
        let t = Instant::now();
        let mut sink = 0usize;
        for _ in 0..n {
            sink += f.gen("en_US", fmt).map(|s| s.len()).unwrap_or(0);
        }
        let d = t.elapsed();
        println!(
            "RUST {:20} {:.3} M ops/s  (sink {})",
            fmt,
            n as f64 / d.as_secs_f64() / 1e6,
            sink
        );
    }
    println!(
        "locales: {}  formatters(data-driven): {}",
        Faker::locales().len(),
        Faker::locale_formatters().len()
    );
}
