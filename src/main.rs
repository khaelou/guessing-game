use std::io; // Import input/output module from the standard library
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("] Guess the number!");
    let mut secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess: ");

        let mut guess = String::new(); // mut identifier enables the value of this variable to be changed
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => break,
        };

        println!("You guessed: {}", guess); // {} string interpolation
        println!("The secret number was: {}", secret_number);

        let guess_to_number = guess.trim().parse::<u32>().unwrap(); // trim string, convert valid integer text to i32 number type
        if guess_to_number == secret_number {
            println!("You guessed correctly, well done!");
        } else if guess_to_number > secret_number {
            println!("You guessed too high, you lose!");
        } else {
            println!("You guessed too low, you lose!");
        }

        match guess.trim().cmp(&secret_number.to_string()) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You won!");
                break;
            },
        }

        secret_number = rand::thread_rng().gen_range(1..=100);
    }
}
