use std::io;

fn main() {
    // Game Title
    println!("Guess the number!");
    // Ask user for guess.
    println!("Please input your guess.");

    // Create storage for user input
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
