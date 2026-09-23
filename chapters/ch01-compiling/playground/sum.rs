fn main() {
    let mut total: u64 = 0;
    for i in 0..100000000000 {
        total += i;
    }
    println!("{total}");
}
