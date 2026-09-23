// cargo run -p ch03-scalars --example demo_floats

use std::hint::black_box;

// Clippy (rightly) flags expressions like `x == x` and `x / x` as probable bugs. Here we
// write them on purpose to show how NaN behaves, so we silence that one lint for this function.
#[allow(clippy::eq_op)]
fn main() {
    println!("== 0.1 + 0.2 ==");
    let sum = 0.1 + 0.2;
    println!("0.1 + 0.2        = {sum}");
    println!("0.1 + 0.2 == 0.3 ? {}", sum == 0.3);
    println!("0.1 to 20 places = {:.20}", 0.1);

    println!();
    println!("== What an f64 looks like in memory (IEEE 754) ==");
    for x in [1.0f64, -1.0, 0.1, 2.0] {
        let bits = x.to_bits(); // the raw 64 bits as a u64
        let sign = bits >> 63;
        let exponent = (bits >> 52) & 0x7FF; // 11 bits
        let mantissa = bits & 0xF_FFFF_FFFF_FFFF; // 52 bits
        println!("{x:>5}: sign={sign} exponent={exponent:011b} mantissa={mantissa:052b}");
    }

    println!();
    println!("== Special values ==");
    let zero: f64 = black_box(0.0);
    let inf = 1.0 / zero;
    let nan = zero / zero;
    println!("1.0 / 0.0  = {inf}"); // no panic: floats have infinity
    println!("0.0 / 0.0  = {nan}");
    println!("NaN == NaN ? {}", nan == nan);
    println!("NaN != NaN ? {}", nan != nan);
    println!("NaN < 1.0  ? {}   NaN > 1.0 ? {}", nan < 1.0, nan > 1.0);
    println!("-0.0 == 0.0 ? {}", -0.0f64 == 0.0);

    println!();
    println!("== f32 has ~7 significant digits, f64 ~16 ==");
    let a: f32 = 16_777_216.0; // 2^24
    println!("16777216f32 + 1.0 = {}", a + 1.0);
    let b: f64 = 16_777_216.0;
    println!("16777216f64 + 1.0 = {}", b + 1.0);
    println!("0.1f32 as f64     = {}", 0.1f32 as f64);

    println!();
    println!("== Scientific notation ==");
    println!("{:e}", 6.022e23);
    println!("{:e}", 0.000_123);
}
