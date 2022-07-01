use proconio::{input, fastout, marker::*};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        s_list: [Chars; n],
    }

    let mut anagram_list = HashMap::new();
    
    let mut anagram_count: i64 = 0;

    for s in s_list.iter() {
        let mut s = s.clone();
        s.sort();
        if let Some(val) = anagram_list.get_mut(&s) {
            *val += 1;
            anagram_count += *val;
        } else {
            anagram_list.insert(s.clone(), 0);
        }
    }

    println!("{}", anagram_count);
}
