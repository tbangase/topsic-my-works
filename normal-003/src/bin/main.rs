use proconio::{input, fastout, marker::*};

use std::time::Instant;

#[fastout]
fn main() {
    input! {
        (r1, k1): (usize, usize),
        (r2, k2): (usize, usize),
    }

    let start = Instant::now();

    println!("{:?}", s);
    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
}
