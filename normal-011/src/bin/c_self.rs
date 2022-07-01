use proconio::{input, fastout, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }


    // pattern 
    // Ok : 123
    // Ok : 132 => Count 1 UP => 323 => Want to through this case => 232 => This is out!!
    // Ok : 213 => Count 1 UP
    // NG : 321 
    // Ok : ``5
    // Ok : 531 
    // => 連続する3つを見て降順になってたら即OUT
    // Case 13123 => 132 ==> OK: Count Up => 123 => Ok
    // Case 131234 => 132 ==> OK: Count Up => 123 => Ok => 34?


    let mut invalid_count = 0;
    let mut prev_char = '`';
    let mut prev_prev_char = '`';
    let mut result = 1;

    // 文字を走査していく
    for (i, c) in s.iter().enumerate() {
        if invalid_count > 1 {
            result = -1;
            break
        }

        // println!("{} -> {} -> {}", prev_prev_char, prev_char, c);
        if *c < prev_char {
            result = i as i32;
            if *c < prev_prev_char {
                if invalid_count > 0 /*|| (prev_prev_char > prev_char && prev_char > *c)*/ {
                    // println!("Error: {} -> {}", prev_prev_char, c);
                    result = -1;
                    break
                }
                invalid_count += 1;
                result = -1;
                continue;
            }
            invalid_count += 1;
            // prev_char = prev_prev_char;
            prev_char = *c;
            continue
        }

        prev_prev_char = prev_char;
        prev_char = *c;
    }

    println!("{}", result);
}
