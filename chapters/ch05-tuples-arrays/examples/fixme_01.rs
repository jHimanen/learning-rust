// FIXME 1: an array's length is part of its type, and so are a tuple's positions
//   cargo run -p ch05-tuples-arrays --example fixme_01 --features fixme
//
// Three errors. Fix each one where the mistake is: keep all six primes and all three
// readings, and remember what the first position of a tuple is called. Fixed means it prints:
//   the first 6 primes: [2, 3, 5, 7, 11, 13]
//   readings: [20.0, 21.5, 19.0]
//   3 readings, mean 20.2

fn main() {
    let primes: [u32; 5] = [2, 3, 5, 7, 11, 13];
    println!("the first {} primes: {primes:?}", primes.len());

    let readings = [20, 21.5, 19.0];
    println!("readings: {readings:?}");

    let summary = (3, 20.2);
    println!("{} readings, mean {}", summary.1, summary.2);
}
