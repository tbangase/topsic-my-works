use proconio::{input, fastout, marker::*};

#[fastout]
fn main() {
    input! {
        (r, c): (usize, usize),
        map: [Chars; r]
    }

    let mut s_coordinate = (0, 0);
    let mut t_coordinate = (0, 0);

    for (r, row) in map.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if *col == 'S' {
                s_coordinate = (r, c);
            }
            if *col == 'T' {
                t_coordinate = (r, c);
            }
        }
    }

    // let res = 
    //     (t_coordinate.0 as isize - s_coordinate.0 as isize).abs() + 
    //     (t_coordinate.1 as isize - s_coordinate.1 as isize).abs();
    let calc_map = vec![vec![false; c]; r];
    let res = walk(&map, calc_map, s_coordinate.0, s_coordinate.1, t_coordinate, r, c, 0);

    println!("{}", res);
}

fn walk(map: &Vec<Vec<char>>, 
        calc_map: &mut Vec<Vec<bool>>,
        r: usize, c: usize, 
        dest: (usize, usize), 
        r_max: usize, c_max: usize,
        count: usize) 
    -> usize {
    if map[r][c] == '.' {
        return 0;
    } else if map[r][c] == 'T' {
        return count;
    } else if calc_map[r][c] == true {
        return count 
    }

    let mut values = vec![];
    if r < r_max - 1 {
        let up = walk(map, r + 1, c, dest, r_max, c_max, count + 1);
        if up != 0 {
            values.push(up);
        }
    }

    if r > 0 {
        let down = walk(map, r - 1, c, dest, r_max, c_max, count + 1);
        if down != 0 {
            values.push(down);
        }
    }

    if c > 0 {
        let left = walk(map, r, c - 1, dest, r_max, c_max, count + 1);
        if left != 0 {
            values.push(left);
        }
    }

    if c < c_max - 1 {
        let right = walk(map, r, c + 1, dest, r_max, c_max, count + 1);
        if right != 0 {
            values.push(right);
        }
    }

    match values.iter().min() {
        Some(val) => return *val,
        None => return 0,
    }
    
}
