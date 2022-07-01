use proconio::{input, fastout};

#[derive(Debug, Clone)]
struct BoyList {
    boys: Vec<Boy>
}

impl BoyList {
    pub fn give_effective(&mut self) {
        let (mut res, mut max_prob) = (0, 0.0);

        for (i, _boy) in self.boys.iter().enumerate() {
            // 各Boyについて
            let mut next_boys = self.clone();
            next_boys.boys.get_mut(i).unwrap().give_chocolate();
            let next_prob = next_boys.prob();
            if max_prob < next_prob {
                max_prob = next_prob;
                res = i;
            };

        }

        self.boys.get_mut(res).unwrap().give_chocolate();
    }

    pub fn prob(&self) -> f64 {
        let mut prob = 1.0;
        for boy in self.boys.iter() {
            prob *= boy.un_prob();
            println!("Prob: {prob}");
        }
        1.0 - prob
    }
}

#[derive(Debug, Clone)]
struct Boy {
    a: f64,
    b: f64,
    j: usize,
}

impl Boy {
    pub fn un_prob(&self) -> f64 {
        let j = self.j as i32;
        1.0 - self.a * (1.0  - self.b.powi(j))
    }

    pub fn give_chocolate(&mut self) {
        self.j += 1;
    }
}

#[fastout]
fn main() {
    input! {
        (n,k): (usize, usize),
        a_b: [(f64, f64); n],
    }

    let mut boys = BoyList {
        boys: a_b.iter().map(|(a, b)| {
            Boy {
                a: *a, b: *b, j:0
            }
        }).collect()
    };

    // チョコを配る
    for _ in 0..k {
        boys.give_effective();
    }

    let prob = boys.prob();

    println!("{:?}", prob);
}

