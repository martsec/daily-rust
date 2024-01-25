use std::collections::HashMap;

pub fn hash_maps() {

    let mut scores = HashMap::new();

    // Strings get MOVED!!! So we can't reuse them after it
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // You can overwrite a value
    scores.insert(String::from("Blue"), 1);
    println!("{:?}", scores);



    // Insert if empty
    scores.entry(String::from("Orange")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // Updating a Value Based on the Old Value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }


    println!("Words counted {:?}", map);
}