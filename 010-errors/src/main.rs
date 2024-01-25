use std::fs::File;
use std::io::{Error, ErrorKind, Read};

pub struct Guess {
    // Use type systems to check for validity and panic if not
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

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

    // Use unwrap for direct panic
    let greeting_file = File::open("hello.txt").unwrap();
    //Or expect for custom error message. It's better
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");

    //
    //////////////// PROPAGATING ERRORS ///////////////////////
    // Extrelly convoluted way
    let text = match read_username_from_file() {
        Ok(s) => s,
        Err(e) => panic!("{e}"),
    };
    println!("{}", text);

    let better = read_from_file_better();
    let shorter = read_from_file_shorter();
    use std::fs;
    let shortest = fs::read_to_string("hello.txt");
}

fn read_username_from_file() -> Result<String, Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e), // This works because we break out of the function
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_from_file_better() -> Result<String, Error> {
    // Use ? to handle the results or return the error from the whole function
    let mut file = File::open("hello.txt")?;
    let mut text = String::new();
    file.read_to_string(&mut text)?; // NOTE: if we don't do this, we'll
    Ok(text)
}

fn read_from_file_shorter() -> Result<String, Error> {
    let mut text = String::new();
    File::open("hello.txt")?.read_to_string(&mut text)?;

    Ok(text)
}
