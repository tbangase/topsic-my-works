use proconio::{input, fastout};

use std::time::Instant;
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize),
        a_s: [usize; n],
        fine_pairs: [(usize, usize); m]
    }

    let start = Instant::now();

    let mut connection: Connection = Connection::new();

    for fine_pair in fine_pairs.iter() {
        connection.connect(fine_pair.0, fine_pair.1);
    }

    println!("{}", connection.count_four_connect_combs());
    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
}

struct Connection{
    connection: HashMap<usize, Vec<usize>>
}

impl Connection {
    pub fn new() -> Self {
        Self{
            connection: HashMap::new()
        }
    }

    pub fn connect(&mut self, a: usize, b: usize) {
        if let Some(vals) = self.connection.get_mut(&a) {
            for val in vals.iter() {
                if *val == b {
                    return 
                }
            }
            vals.push(b);
        } else {
            self.connection.insert(a, vec![b]);
        }
        
        if let Some(vals) = self.connection.get_mut(&b) {
            for val in vals.iter() {
                if *val == a {
                    return 
                }
            }
            vals.push(a);
        } else {
            self.connection.insert(b, vec![a]);
        }
    }

    pub fn count_four_connect_combs(&self) -> usize {
        0
    }
}

