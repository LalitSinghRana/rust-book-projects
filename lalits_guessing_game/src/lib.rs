//! # Lalit's guessing game
//!
//! A fun game where you guess what number the computer has chosen.

use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub fn start_game() {
    println!("Guess the number!!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("please input your guess.");
        
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering:: Less => println!("Too small!"),
            Ordering:: Greater => println!("Too big!"),
            Ordering:: Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
