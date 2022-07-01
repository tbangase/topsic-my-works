use proconio::{input, fastout, marker::*};

use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut appearance: HashSet<char> = HashSet::new();

    for c in s {
        appearance.insert(c);
    }

    println!("{}", appearance.len());
}
