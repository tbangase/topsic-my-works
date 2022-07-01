use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        d: [isize; n],
    }

    let mut current = (0, 0);

    for (index, d_i) in d.iter().enumerate() {
        match index % 4 {
            0 => current.0 += d_i,
            1 => current.1 -= d_i,
            2 => current.0 -= d_i,
            3 => current.1 += d_i,
            _ => ()
        }
        
    }

    println!("{} {}", current.0, current.1);
}
