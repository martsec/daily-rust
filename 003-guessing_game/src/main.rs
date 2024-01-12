use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");

    let lower_bound = 1;
    let higher_bound = 100;
    let secret_number = rand::thread_rng().gen_range(lower_bound..=higher_bound);

    loop {
        println!("Please input your guess of a number uin the range [{lower_bound}, {higher_bound}]");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        // We can override variables?!
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\tYou did not type a number.");
                continue;
            }
        };
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("\tToo small 😞"),
            Ordering::Greater => println!("\tToo big 😞"),
            Ordering::Equal => {
               println!("\t🎉Yaaaay!!!! Correct one!🎉");
               break;
            }
        }
    }


}
