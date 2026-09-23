// cargo run -p ch03-scalars --example demo_casts
//
// `as` converts between numeric types. It never fails, which means it sometimes
// silently changes the value. Know the rules.

use std::hint::black_box;

fn main() {
    println!("== Widening: always safe ==");
    let small: u8 = 200;
    println!("200u8 as u32   = {}", small as u32);
    println!("-5i8 as i64    = {}", -5i8 as i64); // sign is preserved (sign extension)

    println!();
    println!("== Narrowing: keeps only the low bits ==");
    let big: i32 = black_box(300);
    println!(
        "300i32 as u8   = {}   (300 = 0b1_0010_1100; keep the low 8 bits)",
        big as u8
    );
    let neg: i32 = black_box(-1);
    println!("-1i32 as u32   = {}", neg as u32);
    println!("-1i32 as u8    = {}", neg as u8);

    println!();
    println!("== Signed <-> unsigned of the same size: same bits, new meaning ==");
    println!("200u8 as i8    = {}", small as i8);
    println!("-56i8 as u8    = {}", -56i8 as u8);

    println!();
    println!("== Float -> int: truncates toward zero, and saturates ==");
    let f: f64 = black_box(3.99);
    println!("3.99 as i32    = {}", f as i32);
    let f: f64 = black_box(-3.99);
    println!("-3.99 as i32   = {}", f as i32);
    let f: f64 = black_box(-1.5);
    println!("-1.5 as u8     = {}   (clamped to u8's range)", f as u8);
    let f: f64 = black_box(1e20);
    println!("1e20 as i32    = {}", f as i32);
    let f: f64 = black_box(f64::NAN);
    println!("NaN as i32     = {}", f as i32);

    println!();
    println!("== Int -> float: may lose precision for huge values ==");
    let huge: u64 = black_box(9_007_199_254_740_993); // 2^53 + 1
    println!("(2^53 + 1) as f64 = {}", huge as f64);
}
