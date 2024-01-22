// Defining a function to take a string slice instead of a reference to a String
// makes our API more general and useful without losing any functionality:
//
//
// Called "Impliicit *deref coercions*

fn main() {
    let mut s = String::from("hello world");
    
    let first = first_word(&s);

    //s.clear();
    println!("First word is: {}", first);


    let my_string = String::from("hello world");
     // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    // Other slices also exist. Similar to the python ones
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &str) -> &str {
    // str and String are equivalent in this case
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[..i];  // &s[0..i]
        }
    }
    &s[..]  // &s[0..s.len()]
}
