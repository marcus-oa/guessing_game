mod fahrenheit_celcius;
mod guessing_game;

use guessing_game::guessing_game;
use fahrenheit_celcius::temp_converter;
use std::io;

fn main() {
    println!("Guessing game or converter? (G/C)");
    let mut answer = String::new();

    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");

    if answer.trim() == "G" {
        guessing_game()
    }
    else {
        temp_converter()
    }
}
