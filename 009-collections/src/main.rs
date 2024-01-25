use crate::hashmaps::*;
use crate::strings::*;
use std::collections::HashMap;

mod hashmaps;
mod strings;

fn main() {
    strings();
    str_indexing();

    hash_maps();

    let v = vec![1, 2, 3];
    let med = median(&v);
    println!("Median {}", med);

    let v = vec![1, 2, 3, 4];
    let med = median(&v);
    println!("Median {}", med);

    let v = vec![1, 2, 3, 4, 1, 2, 1, 1, 1, 2, 3, 4, 1];
    let mode_v = mode(&v);
    println!("Mode {}", mode_v);
}

fn median(v: &Vec<i32>) -> i32 {
    let mut sorted = v.clone();
    sorted.sort();
    dbg!(&sorted);
    if (v.len() - 1) % 2 == 0 {
        let dirty_middle = (v.len() - 1) / 2;
        dbg!(dirty_middle);
        sorted[dirty_middle]
    } else {
        println!("   NOTE: This is not correct since we should return decimals");
        sorted[v.len() / 2]
    }
}

fn mode(v: &Vec<i32>) -> i32 {
    let mut counter: HashMap<i32, i32> = HashMap::new();
    for value in v {
        let count = counter.entry(*value).or_insert(0);
        *count += 1;
    }
    dbg!(&counter);

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
