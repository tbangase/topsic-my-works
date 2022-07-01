use proconio::{input, fastout};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut a_set: HashSet<usize> = HashSet::new();

    for _ in 0..n {
        input! {
            a: usize
        }
        a_set.insert(a);
    }

    match a_set.len() == n {
        true => println!("YES"),
        false => println!("NO")
    }
}
