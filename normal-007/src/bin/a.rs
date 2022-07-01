use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    println!("{}", n * 15 + m * 5);
}
