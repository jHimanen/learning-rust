// FIXME 1: `if` is an expression, and every branch must produce a value of the same type
//   cargo run -p ch04-control-flow --example fixme_01 --features fixme
//
// There are two errors. Read each one fully (the `-->` location, the labels under the code,
// and the `help:` lines) before changing anything. Don't change the `println!` lines.
// Fixed means it prints:
//   23°C is warm
//   7 is odd

fn main() {
    let celsius = 23;
    let feel = if celsius >= 25 {
        "hot"
    } else if celsius >= 15 {
        "warm";
    } else {
        "cold"
    };
    println!("{celsius}°C is {feel}");

    let n = 7;
    let parity = if n % 2 == 0 {
        "even"
    };
    println!("{n} is {parity}");
}
