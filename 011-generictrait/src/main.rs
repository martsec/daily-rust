use num::Float;
use std::cmp::PartialOrd;
use std::collections::HashMap;
use std::hash::Hash;

fn median<T>(v: &[T]) -> T
where
    T: Float,
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

fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    // Compiler can't infer which str we will return, so we must say
    // that both need to have the same lifetime (or validity)
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let v = vec![100, 32, 57];
    println!("Mode: {}", mode(&v));

    let v = vec![100.3, 3.43, 32.4, 57.4];
    println!("Median: {} ", median(&v));

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest_str(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let result: &str;
    {
        let string3 = String::from("mnijk");
        result = longest_str(string1.as_str(), string3.as_str());
        println!("The longest string is {}", result);
    }
    // This will fail
    //println!("The longest string is {}", result);
    let result: &str;
    {
        let string3 = String::from("mnijk");
        //result = longest_str(string1.clone().as_str(), string3.as_str());
        //println!("The longest string is {}", result);
    }

    let novel = String::from("Call me Atlantis. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{}", i.announce_and_return_part("This is important"));
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
