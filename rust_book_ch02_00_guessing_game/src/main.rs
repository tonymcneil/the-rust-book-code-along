use rand::Rng;
use std::{
    cmp::Ordering,
    io::{self, prelude::*},
};

fn main() {
    let start_range: u32 = 1;
    let end_range: u32 = 100;
    let secret_num = rand::thread_rng().gen_range(start_range, end_range + 1);
    println!("Guess a number between {} and {}", start_range, end_range);

    loop {
        let mut guess = String::new();
        print!("Input a number to guess: ");
        io::stdout().flush().expect("ISSUE: flushing stout!");
        io::stdin()
            .read_line(&mut guess)
            .expect("issue reading line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("ISSUE: must type a number!");
                continue;
            }
        };

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("INCORRECT: too small"),
            Ordering::Greater => println!("INCORRECT: too big"),
            Ordering::Equal => {
                println!("CORRECT: perfect match!");
                break;
            }
        }
    }
}
