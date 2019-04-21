extern crate rand;

use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("secret number is {}", secret_number);

    let mut max_number = 100;
    let mut min_number = 1;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => { num }
            Err(_) => { continue; }
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                min_number = guess;
                println!("now, The secret number is between {} and {}", guess, max_number);
            }
            Ordering::Greater => {
                max_number = guess;
                println!("now, The secret number is between {} and {}", min_number, guess);
            }
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
