use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        mut s: String,
        n: usize
    }

    for _ in 0..n {
        s = format!("a{}", s);
    }

    println!("{}", s);
}
