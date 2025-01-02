use std::io;
fn main() {
    println!("Hello, world!");

    println!("Guess the number!");
    guessing_game();
}

fn guessing_game() {
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
