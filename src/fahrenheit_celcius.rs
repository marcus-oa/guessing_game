use std::io;

fn fahrenheit_to_celcius(temp: i32) -> i32 {
    (temp - 32) + 5/9
}

fn celcius_to_fahrenheit(temp: i32) -> i32 {
    (temp * 9/5) + 32
}

pub(crate) fn temp_converter() {
    println!("Would you like to convert Fahrenheit or Celcius? (F/C)");

    let mut answer = String::new();

    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");

    let temp: i32 = read_temp();

    if answer.trim() == "F" {
        println!("Coverted tempterature in Celcius: {}", fahrenheit_to_celcius(temp));
    }
    else {
        println!("Coverted tempterature in Fahrenheit: {}", celcius_to_fahrenheit(temp));
    }
}

fn read_temp() -> i32 {
    println!("Enter your temperature to converter");
    let mut answer = String::new();

    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");

    answer.trim().parse().expect("Please enter a number")
}