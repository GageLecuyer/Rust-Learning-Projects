use rand::{Rng, RngExt};
use std::io;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!");
    let secret_number: u32 = rand::rng().random_range(1..=100);
    print!("The secret number is: {}", secret_number);
    let mut guess_count = 0;
    loop{
        println!("\nPlease input your guess.");
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        guess_count += 1;
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("You guessed {} many times", guess_count);
            }
        }

    }

}
