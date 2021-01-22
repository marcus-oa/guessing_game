mod fahrenheit_celcius;
mod guessing_game;
mod nth_fibonacci;
mod twelve_days_of_christmas;
mod ownership;
mod structs;
mod enums;
mod pattern_matching;
mod strings;
mod hash_maps;
mod errors;
mod generics;
mod traits;

use guessing_game::guessing_game;
use fahrenheit_celcius::temp_converter;
use nth_fibonacci::get_fibonacci;
use twelve_days_of_christmas::print_carol;
use ownership::{ownership_tests};
use structs::{print_rectangle_area,print_rectangle_area_rfct};
use enums::main_c;
use pattern_matching::match_examples;
use std::io;
use strings::string_examples;
use hash_maps::hashmap_example;
use errors::{errors_examples,errors_examples2};
use generics::{largest_generic,impl_generic_example,different_types_example};
use traits::traits_example;

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
    // ownership_tests();

    /*
    print_rectangle_area();
    print_rectangle_area_rfct();
    main_c();
    match_examples();
    */

    // Strings.rs
    //string_examples();

    //hash_map.rs
    //hashmap_example()

    //errors.rs
    //errors_examples()
    //errors_examples()

    //generics.rs
    //largest_generic();
    //impl_generic_example();
    //different_types_example()

    //traits.rs
    traits_example()
}
