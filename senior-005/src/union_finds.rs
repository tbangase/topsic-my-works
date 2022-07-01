use proconio::{input, fastout};
use petgraph::unionfind::UnionFind;

use std::time::Instant;

#[fastout]
fn main() {
    input! {
        (n, q): (usize, usize),
    }

    let start = Instant::now();

    let mut par = UnionFind::new(n);

    for _ in 0..q {
        input! {
            (p, a, b): (usize, usize, usize)
        }

        if p == 0 {
            // Connect Nodes
            par.union(a, b);
        } else {
            // Judge Linking
            match par.equiv(a, b) {
                true => println!("Yes"),
                false => println!("No"),
            }
        }
    }

    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
}

// pub trait UnionFind {
//     fn root(&self, x: usize) -> usize;
//     fn same(&self, x: usize, y: usize) -> bool;
//     fn unite(&mut self, x: usize, y: usize); 
// }
// 
// impl UnionFind for Vec<usize> {
//     fn root(&self, x: usize) -> usize {
//         if self[x] == x {
//             x
//         } else {
//             self.root(self[x])
//         }
//     }
// 
//     fn same(&self, x: usize, y: usize) -> bool {
//         self.root(x) == self.root(y)
//     }
// 
//     
//     fn unite(&mut self, x: usize, y: usize) { 
//         let x = self.root(x);
//         let y = self.root(y);
//         if x == y {
//             return
//         }
// 
//         self[x] = y;
//     }
// }
