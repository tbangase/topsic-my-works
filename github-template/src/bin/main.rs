use proconio::{input, fastout, marker::*};

use std::time::Instant;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [(u64, u64, u64); n],
    }

    let start = Instant::now();

    println!("{:?}", s);
    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
}
