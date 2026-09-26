// FIXME 1: mutability
//   cargo run -p ch03-scalars --example fixme_01 --features fixme
//
// This doesn't compile. Read the error, then fix it with the smallest possible change.
// Fixed means: it compiles and prints "counted to 3".

fn main() {
    let mut count = 0;
    count += 1;
    count += 1;
    count += 1;
    println!("counted to {count}");
}
