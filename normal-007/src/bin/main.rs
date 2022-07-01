use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        points: [(f64, f64); 6],
    }

    // get length ratio of 3 edges
    let length = [
        get_length(points[0], points[1]),
        get_length(points[1], points[2]),
        get_length(points[2], points[0]),
    ];

    // get target length_ratio
    let target_length = [
        get_length(points[3], points[4]),
        get_length(points[4], points[5]),
        get_length(points[5], points[3]),
    ];

    match check_ratio(length, target_length) {
        true => println!("YES"),
        false => println!("NO"),
    }
}

fn get_length(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    ((p1.0 - p2.0).powi(2) + (p1.1 - p2.1).powi(2)).sqrt()
}

fn check_ratio(tri1: [f64; 3], tri2: [f64; 3]) -> bool {

    // 3種類の始点の選び方
    for i in 0..3 {
        let ratio = tri1[0] / tri2[i];
        
        // 時計周り
        // 長さの比を比較
        let mut res = true;
        for j in 0..3 {
            if tri1[offset(0, j, true)] / tri2[offset(i, j, true)] != ratio {
                res = false;
                break;
            }
        }

        if res {
            return true;
        }

        let mut res = true;
        // 反時計回り
        for j in 0..3 {
            if tri1[offset(0, j, false)] / tri2[offset(i, j, true)] != ratio {
                res = false;
                break;
            }
        }

        if res {
            return true;
        }
    }
    false
}

fn offset(current: usize, offset: usize, is_minus: bool) -> usize {
    let current = current as isize;
    let offset  = offset  as isize;
    let mut res = if is_minus { (current - offset) % 3 } else { (current + offset) % 3 };
    if res < 0 { res += 3 };
    res as usize
}
