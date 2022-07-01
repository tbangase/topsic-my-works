use proconio::{input, fastout};

use std::cmp::Ordering;

#[fastout]
fn main() {
    input! {
        (r1, k1): (usize, usize),
        (r2, k2): (usize, usize),
    }

    match (r1 * k1).cmp(&(r2*k2)) {
        Ordering::Greater => println!("A"),
        Ordering::Less => println!("B"),
        Ordering::Equal => println!("E"),
    }
}
