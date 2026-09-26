// The three loops, ranges, and labels.
//   cargo run -p ch04-control-flow --example demo_loops

fn main() {
    // --- `for` over a range --------------------------------------------------
    // a..b excludes b (like Python's range), a..=b includes it.
    print!("0..5     :");
    for i in 0..5 {
        print!(" {i}");
    }
    println!();

    print!("1..=5    :");
    for i in 1..=5 {
        print!(" {i}");
    }
    println!();

    // Ranges have methods (chapter 20 explains how this works).
    print!("rev      :");
    for i in (1..=5).rev() {
        print!(" {i}");
    }
    println!();

    print!("step_by 3:");
    for i in (0..10).step_by(3) {
        print!(" {i}");
    }
    println!();

    let n = 0;
    print!("1..=n, n=0:");
    for i in 1..=n {
        print!(" {i}"); // never runs: the range is empty
    }
    println!(" (nothing)");

    // --- `while`: loop as long as a condition holds --------------------------
    let mut n = 100;
    let mut halvings = 0;
    while n > 1 {
        n /= 2;
        halvings += 1;
    }
    println!("100 halves {halvings} times before reaching 1");

    // --- `loop`: forever, until `break`. `break value` makes it an expression --
    let mut x: u64 = 1;
    let first_big_power_of_3 = loop {
        x *= 3;
        if x > 1_000 {
            break x;
        }
    };
    println!("first power of 3 above 1000: {first_big_power_of_3}");

    // --- `continue`: skip the rest of this iteration ---------------------------
    print!("odd numbers below 10:");
    for i in 0..10 {
        if i % 2 == 0 {
            continue;
        }
        print!(" {i}");
    }
    println!();

    // --- Labels: say *which* loop to break out of or continue ----------------
    print!("pairs with i * j == 12:");
    'rows: for i in 1..=12 {
        for j in 1..=12 {
            if i * j > 12 {
                continue 'rows; // no point trying bigger j: go to the next i
            }
            if i * j == 12 {
                print!(" ({i},{j})");
            }
        }
    }
    println!();

    // A labeled block is an expression you can `break` out of with a value: the
    // Rust version of Python's `for ... else`.
    let target = 91;
    let smallest_factor = 'search: {
        for d in 2..target {
            if target % d == 0 {
                break 'search d; // found one: this is the block's value
            }
        }
        target // no break happened: this is the block's value
    };
    println!("smallest factor of {target} (other than 1): {smallest_factor}");
}
