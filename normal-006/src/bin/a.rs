use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    if n % 100 == 0 {
        println!("{}", n / 100);
    } else {
        println!("{}", n / 100 + 1);
    }
}
