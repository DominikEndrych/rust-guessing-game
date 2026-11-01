use std::io;
use rand::prelude::*;

fn main(){
    // Generate random number
    let random_number = rand::rng().random_range(1..=100);
    println!("Generated number is {random_number}");

    // Let user input number
    println!("Guess the number: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Input is not a number!");

    // Print user guess
    println!("Guess: {guess}");
}