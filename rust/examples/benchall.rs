use faker2::Faker;
use std::time::{Duration, Instant};
fn main() {
    let list = std::fs::read_to_string("/tmp/fmts.txt").unwrap();
    let f = Faker::seeded(1);
    let mut out = String::from("{");
    let mut first = true;
    for name in list.split_whitespace() {
        if f.gen("en_US", name).is_none() {
            if !first {
                out.push(',');
            }
            first = false;
            out.push_str(&format!("\"{}\":null", name));
            continue;
        }
        let _ = f.gen("en_US", name); // warmup
        let mut n = 0u64;
        let t0 = Instant::now();
        while t0.elapsed() < Duration::from_millis(250) {
            let _ = f.gen("en_US", name);
            n += 1;
        }
        let ops = n as f64 / t0.elapsed().as_secs_f64();
        if !first {
            out.push(',');
        }
        first = false;
        out.push_str(&format!("\"{}\":{:.1}", name, ops));
    }
    out.push('}');
    std::fs::write("/tmp/rs_ops.json", out).unwrap();
    println!("rust done");
}
