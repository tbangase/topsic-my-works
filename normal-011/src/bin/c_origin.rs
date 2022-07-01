use proconio::{input, fastout, marker::*};

#[fastout]
fn main() {
    input! {
        _n: usize,
        s: Chars,
    }
    let mut versus: (usize, usize) = (0, 0);

    for i in 0..(s.len() - 1) {
        if s[i] > s[i+1] {
            versus = (i, i + 1);
            break;
        }
    }

    let mut ans = 1;

    if versus != (0, 0) {
        let mut check_new_world = |world: Vec<char>, target: usize| -> Option<()>{
            for i in 0..(world.len() - 1) {
                if world[i] > world[i+1] {
                    ans = -1;
                    return None;
                }
            }
            ans = (target + 1) as i32;
            Some(())
        };

        let new_s = [&s[..versus.0], &s[versus.0 + 1..]].concat();
        if let None = check_new_world(new_s, versus.0) {
            let new_s = [&s[..versus.1], &s[versus.1 + 1..]].concat();
            check_new_world(new_s, versus.1);
        };
    }

    println!("{}", ans);
}
