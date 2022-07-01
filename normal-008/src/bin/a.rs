use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        (x,y,z): (usize, usize, usize),
    }

    println!("{}", x * 60 * 60 + y * 60 + z);
}
