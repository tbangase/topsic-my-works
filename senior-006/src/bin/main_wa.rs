use proconio::{input, fastout};
#[fastout]
fn main() {
    input! {
        n: usize,
        (x_p, y_p): (i64, i64),
        (x_q, y_q): (i64, i64),
        points: [(i64, i64); n],
    }
    let mut q_max = 0;
    let mut p_max = 0;
    for point in points.iter() {
        let p_diff_square = diff_square(*point, (x_p, y_p));
        let q_diff_square = diff_square(*point, (x_q, y_q));
        if q_diff_square < p_diff_square {
            if q_max < q_diff_square {
                q_max = q_diff_square;
            }
        } else {
            if p_max < p_diff_square {
                p_max = p_diff_square;
            }
        }
    }
    println!("{}", q_max + p_max);
}
fn diff_square(point_1: (i64, i64), point_2: (i64, i64)) -> i64 {
    let diff_x = (point_1.0 - point_2.0).abs();
    let diff_y = (point_1.1 - point_2.1).abs();
    diff_x * diff_x + diff_y * diff_y
}
