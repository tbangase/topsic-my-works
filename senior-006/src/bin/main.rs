use proconio::{input, fastout};

use std::{time::Instant, cmp::Reverse, collections::HashSet};

#[fastout]
fn main() {
    input! {
        n: usize,
        point_p: (i64, i64),
        point_q: (i64, i64),
        points: [(i64, i64); n],
    }

    let start = Instant::now();

    let mut dist_from_p_list = vec![(0, 0)];
    let mut dist_from_q_list = vec![(0, 0)];

    let mut i = 1;
    for &point in points.iter() {
        dist_from_p_list.push((diff_square(point_p, point), i));
        dist_from_q_list.push((diff_square(point_q, point), i));
        i += 1;
    }

    dist_from_p_list.sort();
    dist_from_q_list.sort_by_key(|w| Reverse(*w));

    let mut ans = std::i64::MAX;
    let mut in_p_list = HashSet::new();
    let mut st = 0;

    in_p_list.insert(0);

    // pの円を少しずつ大きくしていく
    for dist_from_p in dist_from_p_list.iter() {
        in_p_list.insert(dist_from_p.1);
        for i in st..=n {
            if in_p_list.contains(&dist_from_q_list[i].1) {
                st += 1;
            } else {
                ans = ans.min(dist_from_p.0 + dist_from_q_list[i].0);
                break;
            }
        }
    }

    ans = ans.min(dist_from_p_list[n].0);

    println!("{:?}", ans);

    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
}

fn diff_square(point_1: (i64, i64), point_2: (i64, i64)) -> i64 {
    let diff_x = (point_1.0 - point_2.0).abs();
    let diff_y = (point_1.1 - point_2.1).abs();
    diff_x * diff_x + diff_y * diff_y
}
