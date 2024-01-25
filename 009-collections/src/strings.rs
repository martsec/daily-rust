pub fn strings() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used



    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    let s = "tic-tac-toe".to_string();
    println!("NTSH {}", s.replace("tic", "nothingtoseehere"));
}

pub fn str_indexing() {
    let s1 = String::from("hello");
    //let h = s1[0];  // Not supported
    // A String is a wrapper over a Vec<u8>
    // in UTF-8, because each Unicode scalar value in that string takes 2 bytes of storage.
    // Therefore, an index into the string’s bytes will not always correlate
    // to a valid Unicode scalar value

    // Similarly, better to not slice strings since we might get half characters
    // Rather, iterate through those

    for c in "Зд".chars() {
        println!("{c}");
    }

}