use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        (a, b, c): (usize, usize, usize),
    }

    let max = a / b;
    if max > c {
        println!("{}", c);
    } else {
        println!("{}", max);
    }
}
