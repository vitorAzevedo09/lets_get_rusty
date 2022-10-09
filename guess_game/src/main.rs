use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number: i32 = rand::thread_rng().gen_range(1, 101);

    println!("Your secret number is {}", secret_number);

    println!("Please input your guess");

    let mut guess: String = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
