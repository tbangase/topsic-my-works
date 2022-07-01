use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        (a, t): (usize, usize),
    }

    println!("{}", a * t);
}
