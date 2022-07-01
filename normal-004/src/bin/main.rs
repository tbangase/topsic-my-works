use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        (h, w, q): (usize, usize, usize),
    }

    let mut map = vec![
        vec!['.'; w]; h
    ];

    for _i in 0..q {
        input! {
            (r, c, s): (usize, usize, char)
        }
        map[r-1][c-1] = s;
    }

    for row in map {
        for col in row {
            print!("{}", col);
        }
        println!("");
    }
}
