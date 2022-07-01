use rand::prelude::*;
use std::time::Instant;

fn main() {
    let mut vector = vec![];

    const VEC_LENGTH: usize = 100000;

    for _ in 0..1000 {
        let start = Instant::now();
        vector.clear();
        for _ in 0..VEC_LENGTH {
            let mut rng = thread_rng();
            let random_value = rng.gen_range(0, VEC_LENGTH);
            vector.push(random_value);
        }
        let duration = start.elapsed();

        println!("Initialized Time: {:?}", duration);

        let length = vector.len();
        let start = Instant::now();
        quick_sort(& mut vector, 0, length - 1);
        let duration = start.elapsed();

        println!("Sorted      Time: {:?}", duration);
    }
}

fn quick_sort<T>(vector: &mut Vec<T>, left: usize, right: usize)
where
    T: PartialOrd + Clone,
{
    if left >= right {
        return;
    }
    // ソート対象のベクトルから3つ取ってきて中央値をPivotとする
    let pivot;
    {
        let pivot_vec = vec![
            vector[left].clone(),
            vector[(left + (right - 1)) / 2].clone(),
            vector[left].clone(),
        ];

        pivot = medium(pivot_vec);
    }
    let mut i = left;
    let mut j = right;

    loop {
        // Pivotより小さい値のインデックスを探す
        while vector[i] > pivot { i += 1 }
        // Pivotより大きい値のインデックスを探す
        while vector[j] < pivot { j -= 1 }

        if i >= j { break; }

        vector.swap(i, j);

        i += 1;
        j -= 1;
    }

    // i is the index of last searched under pivot data
    // In other words, left to i - 1 is over the pivot.
    if i != 0 {
        quick_sort(vector, left, i - 1);
    }
    // same logic as obove, j + 1 to right is under the pivot.
    quick_sort(vector, j + 1, right);
}

fn medium<T: PartialOrd + Clone>(list: Vec<T>) -> T {
    if list[0] > list[1] {
        if list[1] > list[2] {
            let val = list[1].clone();
            return val;
        } else {
            if list[0] > list[2] {
                return list[2].clone();
            } else {
                return list[0].clone();
            }
        }
    } else {
        if list[0] > list[2] {
            return list[0].clone();
        } else {
            if list[1] > list[2] {
                return list[2].clone();
            } else {
                return list[1].clone();
            }
        }
    }
}
