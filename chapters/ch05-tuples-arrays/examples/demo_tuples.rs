// Tuples: fixed-size groups of values, each position with its own type.
//   cargo run -p ch05-tuples-arrays --example demo_tuples

// Returning several values: return one tuple.
fn min_max(a: i32, b: i32) -> (i32, i32) {
    if a <= b { (a, b) } else { (b, a) }
}

fn main() {
    // 1. A tuple, and its type. Each position has its own type, fixed at compile time.
    let reading: (u32, f64, bool) = (7, 21.5, true);
    println!("reading = {reading:?}");
    println!(
        "fields: .0 = {}, .1 = {}, .2 = {}",
        reading.0, reading.1, reading.2
    );

    // 2. Destructuring: unpack into variables. `_` ignores a position.
    let (id, celsius, _) = reading;
    println!("sensor {id} says {celsius}°C");

    // 3. Several return values.
    let (low, high) = min_max(9, 4);
    println!("min_max(9, 4) = ({low}, {high})");

    // 4. Destructuring assignment: no temporary variable needed for a swap.
    let mut x = 1;
    let mut y = 2;
    (x, y) = (y, x);
    println!("after the swap: x = {x}, y = {y}");

    // 5. Tuples compare position by position, like in Python.
    println!("(1, 9) < (2, 0) is {}", (1, 9) < (2, 0));
    println!("(1, 'b') < (1, 'c') is {}", (1, 'b') < (1, 'c'));

    // 6. One-element tuples need a comma. Without it, the parentheses are just grouping.
    let one = (5,);
    #[allow(unused_parens)] // the compiler would (rightly) warn: these parentheses do nothing
    let not_a_tuple = (5);
    println!("(5,) is {one:?}, but (5) is {not_a_tuple:?}");

    // 7. Zero elements: `()`, the unit type from chapter 4, is the empty tuple.
    let empty = ();
    println!("the empty tuple is {empty:?}");

    // 8. {:#?} pretty-prints nested values over several lines.
    let nested = ((1, 2), [3.5, 4.5], ('a', "text"));
    println!("{nested:#?}");

    // 9. dbg! prints the file, the line, the expression itself and its value (to stderr),
    //    and then hands the value back, so you can wrap it around any expression.
    let area = dbg!(x * 10) * y;
    dbg!(area, min_max(area, 15));
}
