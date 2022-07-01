use proconio::{input, fastout, marker::*};

use std::collections::{HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        (r, c): (usize, usize),
        spaces: [Chars; r],
    }

    let mut max = 0;

    for row in 0..r {
        for col in 0..c {
            let min_manhattan = search_min_manhattan(r, c,row, col, &spaces);
            if min_manhattan > max {
                max = min_manhattan;
            }
        }
    }

    println!("{}", max);
}

fn search_min_manhattan(r: usize, c: usize, row: usize, col: usize, spaces: &[Vec<char>]) -> usize {
    // 距離が短いところから探していく
    let mut searched = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back((row, col));

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        if spaces[current.0][current.1] == '*' {
            return calc_manhattan((row, col), current);
        }
        searched.insert(current);

        // 右へ
        // searchedに入っているか、右端にいるかを判断する
        if current.1 != c - 1 {
            let target = (current.0, current.1 + 1);
            if let None = searched.get(&target) {
                queue.push_back(target);
            }
        }
        
        // 左へ
        if current.1 != 0 {
            let target = (current.0, current.1 - 1);
            if let None = searched.get(&target) {
                queue.push_back(target);
            }
        }

        // 上へ
        if current.0 != 0 {
            let target = (current.0 - 1, current.1);
            if let None = searched.get(&target) {
                queue.push_back(target);
            }
        }

        // 下へ
        if current.0 != r - 1 {
            let target = (current.0 + 1, current.1);
            if let None = searched.get(&target) {
                queue.push_back(target);
            }
        }
    }
    0
}

fn calc_manhattan(start: (usize, usize), end: (usize, usize)) -> usize {
    ((start.0 as isize - end.0 as isize).abs() + (start.1 as isize - end.1 as isize).abs()) as usize
}

