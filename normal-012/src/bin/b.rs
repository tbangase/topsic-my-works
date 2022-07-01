use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        a_s: [i64; n],
    }

    let mut previous = a_s[0];
    let mut fatigue = 0;
    for i in 1..n {
        fatigue += step(previous, a_s[i]);
        previous = a_s[i];
    }

    let mut rev_fatigue = 0;
    previous = a_s[n-1];
    for i in (0..(n-1)).rev() {
        rev_fatigue += step(previous, a_s[i]);
        previous = a_s[i];
    }

    println!("{}", fatigue.min(rev_fatigue));
}

// return fatigue
fn step(prev: i64, next: i64) -> i64 {
    0.max(next - prev)
}
