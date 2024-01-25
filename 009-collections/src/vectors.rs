fn fun_with_vectors() {
    let mut v: Vec<i32> = Vec::new();
    v.push(2);
    v.push(4);
    // Vec doubles allocated space every time it reaches the limit, so we have 2, 4, 8, 16, 32...
    println!("My vector is {}", v);
    // Can also manually allocate more, but be careful since it can be suboptimal
    v.allocate(3);

    let v = vec![1, 2, 3];

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("My vector is {}", v);

}

fn can_eums_are_same_type() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];



}