use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: f64,
    }

    let res_str = format!("{:.3}", n / 500.0);

    let mut res_str = res_str.chars();
    res_str.next();
    let res_str = res_str.as_str();

    println!("{}", res_str);
}
