// FIXME 2: where a loop's values live, and which loops can produce a value
//   cargo run -p ch04-control-flow --example fixme_02 --features fixme
//
// The compiler reports three errors, but there are only two mistakes: one error is a
// consequence of another. Which one? Keep the first loop a `for` loop over `1..=10`, and keep
// the second one searching upwards from 101 (you'll need a different kind of loop for it).
// Fixed means it prints:
//   sum of 1..=10 = 55
//   first multiple of 7 above 100 = 105

fn main() {
    for i in 1..=10 {
        let mut total = 0;
        total += i;
    }
    println!("sum of 1..=10 = {total}");

    let first = for n in 101.. {
        if n % 7 == 0 {
            break n;
        }
    };
    println!("first multiple of 7 above 100 = {first}");
}
