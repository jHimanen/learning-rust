use std::hint::black_box;

fn main() {
    #[inline(never)]
    fn pick_larger(a: u64, b: u64) -> u64 {
        if a > b { a } else { b }
    }
    println!("{}", pick_larger(black_box(10), black_box(11)));
}
