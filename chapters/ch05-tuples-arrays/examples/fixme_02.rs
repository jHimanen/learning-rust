// FIXME 2: indexing an array
//   cargo run -p ch05-tuples-arrays --example fixme_02 --features fixme
//
// The compiler reports one error. Fix it, and a second error appears that wasn't reported
// before: the compiler checks your program in phases, and the second check only runs once the
// types are right. Keep `count` an `i32`. Fixed means it prints:
//   score 1: 72
//   score 2: 95
//   score 3: 88
//   score 4: 61
//   last score: 61

fn main() {
    let scores = [72, 95, 88, 61];
    let count: i32 = 4;

    // Print every score with its position, counting from 1 like a human would.
    for position in 1..=count {
        println!("score {position}: {}", scores[position - 1]);
    }

    let last = scores[4];
    println!("last score: {last}");
}
