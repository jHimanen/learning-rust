// FIXME 2: Rust never converts between number types implicitly
//   cargo run -p ch03-scalars --example fixme_02 --features fixme
//
// Fix both errors using `as` casts. Don't change any type annotations.
// Careful: after your first fix, the compiler may report a *new* error. Think about which
// type each `+` or `/` is computed in, not just what the result is assigned to.
// Fixed means it prints:
//   total bytes: 5000000000
//   average: 2.5

fn main() {
    // A file-size counter: u32 is too small for the total, so it's accumulated in a u64.
    let file_a: u32 = 3_000_000_000;
    let file_b: u32 = 2_000_000_000;
    let total: u64 = file_a as u64 + file_b as u64;
    println!("total bytes: {total}");

    let sum: i32 = 10;
    let count: i32 = 4;
    let average: f64 = sum as f64 / count as f64;
    println!("average: {average}");
}
