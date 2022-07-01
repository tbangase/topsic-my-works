use proconio::{input, fastout, marker::*};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    for alphabet in 'a'..='z' {
        let mut count = 0;
        for c in s.iter() {
            if *c == alphabet {
                count += 1;
            }
        }
        print!("{} ", count);
    }

    println!("");
}
