use proconio::{input, fastout, marker::*};

use std::time::Instant;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let start = Instant::now();

    for i in 0..(s.len() - 1) {
        if s[i] > s[i + 1] {
            // 一番最初
            
            // 一番後ろ
            // その他
        }
    }

    println!("{:?}", s);
    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
}
