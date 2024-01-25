use std::fs::File;
use std::io::ErrorKind;

fn main() {
    println!("Hello, world!");

    // Use panic for non recoverable errors
    // panic!("I'm a dead marioooo");

    let file_to_open = File::open("hello.txt");
    let file = match file_to_open {
        Ok(content) => content,
        Err(err) => panic!("ERROR. File not found. {err}"),
    };

    let file_to_open = File::open("hello.txt");
    let greeting_file = match file_to_open {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // Simpler way with the unwrap_or_else
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
