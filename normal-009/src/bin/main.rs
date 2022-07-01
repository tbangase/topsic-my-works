use proconio::{input, fastout};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        (_n, m): (u64, u64),
    }
    let mut map: HashMap<u64, Vec<u64>> = HashMap::new();
    for _ in 0..m {
        input!{
            a: u64,
            b: u64,
        } 

        set_connection(&mut map, a, b);
        set_connection(&mut map, b, a);
    }

    let mut count = 0;
    for (_, connections) in &map {
        if connections.len() >= 6 {
            let length = connections.len();
            // omg: calc nP6
            count += length *
                (length - 1) *
                (length - 2) *
                (length - 3) *
                (length - 4) *
                (length - 5);
        }
    }

    println!("{:?}", count);
}

fn set_connection(connection: &mut HashMap<u64, Vec<u64>>, a: u64, b: u64) {
    match connection.get_mut(&a){
        Some(val) => {
            val.push(b);
        },
        None => {
            connection.insert(a, vec![b]);
        }
    }
}
