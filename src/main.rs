mod fahrenheit_celcius;
mod guessing_game;
mod nth_fibonacci;
mod twelve_days_of_christmas;
mod ownership;

use guessing_game::guessing_game;
use fahrenheit_celcius::temp_converter;
use nth_fibonacci::get_fibonacci;
use twelve_days_of_christmas::print_carol;
use ownership::{ownership_tests};
use std::io;

// Main
fn main() {
    /* CHAPTER 3 EXERCISES
    println!("Guessing Game ,Temperature Converter ,Nth Fibonacci or Twelve days of Christmas ? (G/C/F/T)");
    let mut answer = String::new();

    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");

    if answer.trim() == "G" {
        guessing_game()
    }
    else if answer.trim() == "F" {
        get_fibonacci()
    }
    else if answer.trim() == "T"{
        print_carol()
    }
    else {
        temp_converter()
    }*/

    // Ownership
    ownership_tests();
}
