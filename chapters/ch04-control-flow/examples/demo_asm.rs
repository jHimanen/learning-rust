// What `if` and a loop turn into on your CPU. Generate the ARM64 assembly with:
//   rustc -O --emit=asm chapters/ch04-control-flow/examples/demo_asm.rs -o target/demo_asm.s
// and open target/demo_asm.s. Running it just prints the results:
//   cargo run -p ch04-control-flow --example demo_asm --release

use std::hint::black_box;

// `#[inline(never)]` keeps each function as a separate piece of machine code. Otherwise the
// optimizer would paste (inline) it into `main`, and you couldn't find it.
#[inline(never)]
fn pick_larger(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

#[inline(never)]
fn halvings(mut n: u64) -> u32 {
    let mut steps = 0;
    while n > 1 {
        n /= 2;
        steps += 1;
    }
    steps
}

fn main() {
    // black_box (chapter 3) stops the optimizer from computing the answers at compile time.
    println!(
        "pick_larger(3, 8) = {}",
        pick_larger(black_box(3), black_box(8))
    );
    println!("halvings(100)     = {}", halvings(black_box(100)));
}
