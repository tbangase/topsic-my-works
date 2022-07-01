use proconio::{input, fastout, marker::*};

use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        s: Chars
    }

    let mut qwerty: HashMap<char, isize> = HashMap::new();
    qwerty.insert('a', 0);
    qwerty.insert('s', 1);
    qwerty.insert('d', 2);
    qwerty.insert('f', 3);
    qwerty.insert('g', 4);
    qwerty.insert('h', 5);
    qwerty.insert('j', 6);
    qwerty.insert('k', 7);
    qwerty.insert('l', 8);
    let qwerty = qwerty;

    let mut total = 0;
    let mut current = 0;
    for c in s {
        total += (current - qwerty[&c]).abs() + 1;
        current = qwerty[&c];
    }

    println!("{}", total);
}
