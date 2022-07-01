use proconio::{input, fastout};
use std::cmp::{min, max};

#[fastout]
fn main() {
    input! {
        (n, g, a): (isize, isize, isize),
    }

    let mut count = 0;
    let mut people = a;
    while people > 0 {
        let majority = n / 2 + 1;
        if people >= majority {
            people -= majority;
            count += 1;
        } else {
            people = 0;
        }
    }
    print!("{}", min(count, g));

    count = 0;
    people = a;
    while people > 0 {
        let minority = n / 2;
        // 残りグループ数と残り賛成人数と照らし合わせる
        let left_space = (g - count) * n - minority;
        let has_space = left_space > people;
        if !has_space {
            break
        }
        if people >= minority {
            people -= minority;
            count += 1;
        } else {
            count = g;
            break
        } 
    }
    print!(" {}\n", max(g - count, 0));
}
