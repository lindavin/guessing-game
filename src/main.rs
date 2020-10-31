extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // Game Title
    println!("Guess the number!");

    // Set secret number;
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // Show secret number for internal check
    println!("This is the secret number {}.", secret_number);
    loop {
        // Ask user for guess.
        println!("Please input your guess.");

        // Create storage for user input
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert String to Number
        let guess: u32 = guess.trim().parse().expect("Enter a number!");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Guess is too low!"),
            Ordering::Greater => println!("Guess to is large!"),
            Ordering::Equal => {
                println!("You got it!");
                break;
            }
        }
    }
}
