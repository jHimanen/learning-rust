// Run it in both profiles and compare the endings:
//   cargo run -p ch03-scalars --example demo_overflow
//   cargo run -p ch03-scalars --example demo_overflow --release

use std::hint::black_box;

fn main() {
    // black_box hides the value from the optimizer. Without it, the compiler would see
    // `250 + 10` at compile time, and refuse to compile a guaranteed overflow.
    let x: u8 = black_box(250);

    println!("x = {x}; what is x + 10 for a u8 (max 255)?");
    println!("wrapping_add(10)    = {}", x.wrapping_add(10));
    println!("saturating_add(10)  = {}", x.saturating_add(10));
    println!("checked_add(10)     = {:?}", x.checked_add(10)); // None = "didn't fit"
    println!("checked_add(5)      = {:?}", x.checked_add(5)); // Some(255) = "fit, here it is"
    println!("overflowing_add(10) = {:?}", x.overflowing_add(10)); // (result, did it overflow?)

    println!();
    println!("And now plain `x + 10`...");
    let y = x + 10; // debug: panic. release: silently wraps.
    println!("x + 10 = {y}");
}
