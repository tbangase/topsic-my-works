use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        c1: String,
        c2: String,
    }

    if c1 < c2 {
        println!("FIRST");
    } else {
        println!("SECOND");
    }
}
