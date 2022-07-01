use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    println!("{:?}", n + (n - 1) / 100 * 30);
}
