use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let five_hundreds = n / 500;
    let remains = n % 500;

    if remains % 100 != 0 {
        println!("-1");
        return
    }

    let hundreds = remains / 100;

    println!("{}", five_hundreds + hundreds);
}
