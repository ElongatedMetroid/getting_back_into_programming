//! The program will generate a number from 1
//! to 100, it will then prompt the player to
//! enter a guess, lastly it will tell if the
//! guess is to high or low (or correct)

use std::{io, cmp::Ordering};

use rand::Rng;
fn main() {
    let random: usize = rand::thread_rng()
        .gen_range(0..=100);

    let mut guess = String::new();

    loop {
        guess.clear();
        println!("Enter your guess: ");
        io::stdin().read_line(&mut guess).unwrap();

        let guess: usize = guess
            .trim()
            .parse()
            .unwrap();

        match guess.cmp(&random) {
            // Guess is less then the number
            Ordering::Less => println!("Higher"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            },
            Ordering::Greater => println!("Lower"),
        }
    }
}
