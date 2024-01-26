use num::Float;
use std::cmp::PartialOrd;
use std::collections::HashMap;
use std::hash::Hash;

fn median<T>(v: &[T]) -> T
where
    T: Float + Clone,
{
    // Issue: we must return a decimal since if it has a pair number of elements
    // we'll need to divide them.
    // Solution here https://docs.rs/statistical/latest/src/statistical/stats_.rs.html#60-74
    let mut sorted: Vec<&T> = Vec::with_capacity(v.len());
    sorted.extend(v.iter());
    // Float does not implement Ord trait (though it implements PartialOrd...)
    // So we need another way
    //sorted.sort();
    // Docs to the resque https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let middle = (v.len() - 1) / 2;
    if (v.len() - 1) % 2 == 0 {
        dbg!(middle);
        *sorted[middle]
    } else {
        (*sorted[middle] + *sorted[middle + 1]) / num::cast(2).unwrap()
    }
}

fn mode<T: Eq + Hash + Copy>(v: &[T]) -> T {
    // Updated to generic
    let mut counter: HashMap<T, i32> = HashMap::new();
    for value in v {
        let count = counter.entry(*value).or_insert(0);
        *count += 1;
    }

    let mut mode = v[0];
    let mut max_frequency = 0;
    for (num, count) in counter.iter() {
        if *count > max_frequency {
            max_frequency = *count;
            mode = *num;
        }
    }
    mode
}

fn main() {
    let v = vec![100, 32, 57];
    println!("Mode: {}", mode(&v));

    let v = vec![100.3, 3.43, 32.4, 57.4];
    println!("Median: {} ", median(&v))
}
