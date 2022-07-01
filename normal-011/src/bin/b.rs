use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }

    for i in 0..n {
        if a[i] < b[i] {
            println!("{}", i + 1);
            break
        }
    }
}
