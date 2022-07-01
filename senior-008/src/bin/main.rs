use proconio::{input, fastout};

use std::{collections::HashSet, iter::FromIterator};

#[fastout]
fn main() {
    input! {
        (n, k): (usize, usize),
    }

    let mut meet_records = vec![];

    let mut days: usize = 0;

    while k * 2usize.pow(days as u32) < 2 * n {
        days += 1;
    }

    for day in 0..days {
        // Let's simulate!
        let knowing_people = k * 2usize.pow(day as u32);
        let people = (0..(2*n)).collect::<Vec<_>>();
        let mut not_met_people: HashSet<usize> = HashSet::from_iter(people.iter().cloned());

        let mut meet_people = vec![0; 2 * n];
        let mut i = 0;
        while i + knowing_people < 2 * n && i < knowing_people {
            // println!("Productive Met: {}", i);
            meet_people[i + knowing_people] = i;
            meet_people[i] = i + knowing_people;
            
            not_met_people.remove(&i);
            not_met_people.remove(&(i + knowing_people));
            i += 1;
        }

        while i < n {
            // println!("Fun met: {}", i);
            let p1 = not_met_people.iter().cloned().next().unwrap_or(0);
            not_met_people.remove(&p1);
            let p2 = not_met_people.iter().cloned().next().unwrap_or(0);
            not_met_people.remove(&p2);

            meet_people[p1] = p2;
            meet_people[p2] = p1;
            i += 1;
        }

        meet_records.push(meet_people);
    }

    println!("{}", days);

    for meet_record in meet_records {
        for ones_record in meet_record {
            print!("{} ", ones_record + 1);
        }
        println!("");
    }
}
