use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    if n < 3 {
        println!("YES");
        return
    }

    for i in 0..(n-2) {
        let middle = ( a[i] + a[i + 2] ) / 2;
        if middle >= a[i + 1] {
            println!("NO");
            return
        }
    }

    println!("YES");
}
