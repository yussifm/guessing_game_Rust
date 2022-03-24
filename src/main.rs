use std::io;
use rand::Rng

fn main() {
    println!("Guess a number!");
    let secret_num = rand::thread_rng().gen_range(1..101);
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

    println!("you guessed: {}", guess);
}
