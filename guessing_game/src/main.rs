use std::cmp::Ordering;

use std::io;
use rand::Rng;

fn main() {

    println!("welcome to guessing game");

    let secret_number= rand::thread_rng().gen_range(1..=100);

    println!("The secret_number is: {secret_number}");

    println!("Enter your guess: ");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess:i32 = guess.trim().parse().expect("Please Type a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too Small"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win"),
    }

}