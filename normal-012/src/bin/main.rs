use proconio::{input, fastout};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [[i64; n]; n],
    }

    let mut row_max = -100000;
    for perms in (0..n).permutations(n) {
        let mut sum = 0;
        for i in 0..n {
            for j in 1..n {
                sum += a[i][perms[j - 1]] * a[i][perms[j]];
            }
        }

        if row_max < sum {
            row_max = sum;
        }
    }

    let mut col_max = -100000;
    for perms in (0..n).permutations(n) {
        let mut sum = 0;
        for i in 1..n {
            for j in 0..n {
                sum += a[perms[i-1]][j] * a[perms[i]][j];
            }
        }

        if col_max < sum {
            col_max = sum;
        }
    }

    println!("{}", row_max + col_max);
}
