// Expressions, statements, blocks and `if` as a value.
//   cargo run -p ch04-control-flow --example demo_expressions

use std::mem::size_of;

// No `-> type`: the function returns `()`, the unit value.
fn greet(name: &str) {
    println!("hello, {name}");
}

// `return` exits early. The last expression is the normal return value.
// (`&'static str` is the type of a string literal. Chapter 10 explains the `'static`.)
fn describe(n: i32) -> &'static str {
    if n == 0 {
        return "zero"; // early exit
    }
    if n % 2 == 0 { "even" } else { "odd" } // the value of the whole body
}

// Everything in this function is legal Rust that a real program shouldn't contain, and the
// compiler and clippy warn about every line. Delete the `#[allow(...)]` line and run
// `cargo clippy -p ch04-control-flow --example demo_expressions` to see all the warnings.
#[allow(
    unused_must_use,
    unused_assignments,
    clippy::no_effect,
    clippy::let_unit_value
)]
fn where_unit_comes_from() {
    // A semicolon turns an expression into a statement, whose value is `()`.
    let block_value = {
        let x = 5;
        x + 1; // value computed, then thrown away
    };
    println!("a block ending in `;` has the value {block_value:?}");

    // Even assignment is an expression, of type `()`.
    let mut counter = 0;
    let assignment_value = counter = 10;
    println!("counter = {counter}, and `counter = 10` evaluated to {assignment_value:?}");

    // A function without a return type returns `()`.
    let greeted = greet("again");
    println!("greet returned {greeted:?}");
}

fn main() {
    // 1. A block `{ ... }` is an expression: its value is its last expression.
    let area = {
        let width = 3;
        let height = 4;
        width * height // no semicolon: this is the block's value
    };
    println!("area = {area}");
    // `width` and `height` are gone here: they only lived inside the block.

    // 2. `if` is an expression too. Both branches must have the same type.
    let temperature = 23;
    let feel = if temperature >= 25 {
        "hot"
    } else if temperature >= 15 {
        "warm"
    } else {
        "cold"
    };
    println!("{temperature}°C is {feel}");

    // It fits on one line, like Python's `x if cond else y`:
    let n: i32 = -7;
    let magnitude = if n < 0 { -n } else { n };
    println!("|{n}| = {magnitude}");

    // 3. Functions.
    greet("rust");
    for n in [0, 4, 7] {
        println!("{n} is {}", describe(n));
    }

    // 4. Statements, and the `()` they leave behind.
    where_unit_comes_from();

    // 5. Under the hood: `()` carries no information, so it takes no memory.
    println!();
    println!("size_of::<()>()   = {} bytes", size_of::<()>());
    println!("size_of::<bool>() = {} byte", size_of::<bool>());
}
