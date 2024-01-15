

fn main() {
    println!(r"Rust has two different memory ownership behaviours
    One for simple and usually compile-time known variables (stored in the stack)
    And the other one more complex classes like Vec and String, that usually vary in lenght.

    The fists one goes to the stack and it's usually copied, while the latter goes to the heap.
    For those variables we usually hold a *UNIQUE* reference in the stack. 
    This uniquenes is where the *ownership* concept comes from. Due to how memory is managed,
    if two variables could have the same reference, we could end up cleaning it twice or something
    worse, causing fragility and potential security threads.
    ");
    ownership_in_functions();

    let mut canbe_muted = String::from("\n\nWe can pass mutable references to a function");
    mutable_references(&mut canbe_muted);
    println!("{}", canbe_muted);
    mutable_references_nomut(&canbe_muted);

    println!("\n\nAlso, only one mutable reference can be active at a time to avoid race conditions. Useful!");
    println!("\tAnd similarly, it does not allow for any other active immutable reference to the same variable");


    mutability_and_race_conditions();
}


fn ownership_in_functions() {
    let number = 3;
    let my_string = String::from("I am a heap string");

    giving_heap_ownership(number, my_string);
    println!("Outside the function I still can use {number} but not the string");
    //println!("This fails due to ownership transfer {my_string}");

    let my_string = String::from("I am a heap string");
    deep_cloning(number, my_string.clone());
    println!("I still can use my_string outside the function '{my_string}'");

    let my_string = String::from("I am a heap string");
    just_referencing(number, &my_string);
    println!("I still can use my_string outside the function '{my_string}'");
}

fn giving_heap_ownership(stack: i32, heap: String) {
    println!("\n\nTypes like i32 are copied but for String and Vec we pass the ownership.");
    println!("  So they cannot be used outside this function anymore unless we return them");
    println!("     I can print the stack '{stack}' and '{heap}' variables");
}

fn deep_cloning(stack: i32, heap: String) {
    println!("\n\nWe can deep clone/copy but we'll duplicate data 😞"); 
    println!("     I can print the stack '{stack}' and '{heap}' variables");
}

fn just_referencing(stack: i32, heap: &String) {
    println!("\n\nThis time we just pass the reference and will remain usable outside"); 
    println!("     I can print the stack '{stack}' and '{heap}' variables");
}


fn mutable_references(some_text: &mut String) {
    some_text.push_str("\n\tI can add some text if I pass the variable as a mutable reference parameter");
}

fn mutable_references_nomut(some_text: &String) {
    println!("\tOr send them without allowing the mutation. Just to read as '{some_text}'");
}

fn mutability_and_race_conditions() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &v[2];
    // Trying to update a heap vaiable that will be later read!
    // v.push(4);
    println!("Third element is {}", *num);
    
    // We can modify it later safely
    v.push(4);
}
