use proconio::{input, fastout};

use std::{time::Instant, collections::{HashMap, HashSet}};

// #[fastout]
fn main() {
    input! {
        n: usize,
        ws: [u64; n],
        edges: [(usize, usize); n-1],
    }

    let start = Instant::now();

    let mut directions = HashMap::new();

    for (node1, node2) in edges.iter() {
        directions.entry(*node1).or_insert(vec![]).push(*node2);
        directions.entry(*node2).or_insert(vec![]).push(*node1);
    }

    let mut weights = HashMap::new(); 

    for (i, w) in ws.iter().enumerate() {
        weights.insert(i+1, *w);
    }

    println!("");
    println!("Directions: {:?}", directions);

    for current in 1..=n {
        let mut searched = HashSet::new();
        searched.insert(current);
        if calc_weight_recursively(current, &mut searched, &directions, &weights) != None {
            print!("{} ", current);
        }
    }
    println!("");

    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
}

fn calc_weight_recursively(
    current: usize, 
    searched: &mut HashSet<usize>, 
    directions: &HashMap<usize, Vec<usize>>,
    weights: &HashMap<usize, u64>,
) -> Option<u64> {
    // Get list of Directions
    if let Some(list) = directions.get(&current) {
        let mut fc = None;
        let current_weight = weights.get(&current).unwrap();
        println!("Searching Node: {}", current);
        for node in list {
            if searched.get(&node) == None {
                // If not searched
                searched.insert(*node);
                if let Some(f) = calc_weight_recursively(*node, searched, directions, weights) {
                    if let Some(target) = fc {
                        // If 2nd child
                        if target != f {
                            println!("Not the same as fc.");
                            return None;
                        }
                    } else {
                        // If 1st child
                        println!("Searching first child.");
                        fc = Some(current_weight + f);
                    }
                } else {
                    return None
                }
            }
            println!("fc: {:?}", fc);
            // If searched, skip.
        }
        if fc == None {
            // If no search target, it means this is the end of the tree.
            println!("f(x) of {current}: {:?}", current_weight);
            return Some(*current_weight);
        } else {
            // If has search target, return fc.
            println!("f(x) of {current}: {:?}", fc);
            return fc;
        }
    }
    None
}


