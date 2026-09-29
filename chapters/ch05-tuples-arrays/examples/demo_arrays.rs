// Arrays: a fixed number of values of one type, stored side by side.
//   cargo run -p ch05-tuples-arrays --example demo_arrays

// This function gets its own copy of the array: all five numbers are copied in.
fn zero_the_first(mut values: [i32; 5]) -> [i32; 5] {
    values[0] = 0;
    values
}

fn main() {
    // 1. Literals. The type is [element type; length], and the length is part of the type.
    let primes: [u32; 5] = [2, 3, 5, 7, 11];
    let zeros = [0.0; 4]; // [value; count]: four 0.0s, type [f64; 4]
    println!("primes = {primes:?}, len {}", primes.len());
    println!("zeros  = {zeros:?}");

    // 2. Indexing. Indices are usize and start at 0. `mut` makes the elements writable.
    let mut counts = [0u32; 3];
    counts[1] += 10;
    counts[2] = counts[1] * 2;
    println!("counts = {counts:?}, first prime = {}", primes[0]);

    // 3. Looping. `for x in array` hands out each element by value.
    let mut total = 0;
    for p in primes {
        total += p;
    }
    println!("sum of primes = {total}");

    // `.into_iter().enumerate()` hands out (index, element) tuples. Destructure them.
    for (i, p) in primes.into_iter().enumerate() {
        if p > 4 {
            println!("the first prime above 4 is primes[{i}] = {p}");
            break;
        }
    }

    // 4. Destructuring works for arrays too.
    let [red, green, blue] = [255u8, 128, 0];
    println!("red {red}, green {green}, blue {blue}");

    // 5. Arrays of arrays: a grid. grid[row][column].
    let mut grid = [[0; 3]; 2]; // 2 rows of 3
    grid[1][2] = 7;
    println!("grid = {grid:?}");
    for row in grid {
        println!("  {row:?}");
    }

    // 6. Assignment COPIES the whole array. There's no sharing, unlike a Python list.
    let a = [1, 2, 3, 4, 5];
    let mut b = a;
    b[0] = 99;
    println!("a = {a:?}, b = {b:?}");

    // So does passing it to a function.
    let c = zero_the_first(a);
    println!("a = {a:?} is untouched, c = {c:?}");

    // 7. Arrays compare element by element.
    println!("a == [1, 2, 3, 4, 5] is {}", a == [1, 2, 3, 4, 5]);
}
