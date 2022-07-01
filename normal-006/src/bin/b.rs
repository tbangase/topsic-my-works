use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        (xl, yl, xr, yt, a, b): (isize, isize, isize, isize, isize, isize),
    }

    let is_start_inside = 0 > xl && 0 < xr && 0 > yl && 0 < yt;
    let is_goal_inside  = a > xl && a < xr && b > yl && b < yt;

    match is_goal_inside ^ is_start_inside {
        true  => println!("NO"),
        false => println!("YES"),
    }
}
