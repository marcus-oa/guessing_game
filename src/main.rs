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
mod lifetimes;
mod closures;
mod iterators;
mod smart_pointers;
mod reference_cycles;
mod concurrency;
mod object_oriented;
mod blog;
mod blog_implementation_two;
mod patterns_and_matching;
mod unsafe_rust;
mod advanced_traits;
mod advanced_types;
mod advanced_functions_and_closures;
mod macros;


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
use lifetimes::{lifetime_example1,lifetime_example2,lifetime_example3};
use closures::{simulated_expensive_calculation,generate_workout};
use smart_pointers::
{box_example,
 deref_example,
 drop_example,
 rc_example,rc_and_refcall_example};
use reference_cycles::{ref_cycle_example, node_example};
use concurrency::
{thread_example_one,
 thread_example_two,
 move_thread_example,
 message_passing_example,
 message_passing_example_two,
 multi_message_sending_example,
 mutex_example,
 shared_mutex_thread_example};
use blog::Post;
use blog_implementation_two::Post as newPost;
use patterns_and_matching::
{if_else_else_if_let_example,
 while_loop_example,
 for_loop_example,
 function_parameters_example,
 destructing_example};
use unsafe_rust::{raw_pointers,static_variables};
use std::os::macos::raw::stat;
use advanced_traits::
{test_point_addition,
 calling_same_named_functions,
 print_names,
 test_fmt_and_print,
 print_test_newtype};
use advanced_functions_and_closures::function_pointer_example;
use macros::macros_example;

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
    //traits_example()

    //lifetimes.rs
    //lifetime_example1();
    //lifetime_example2();
    //lifetime_example3();

    //closures.rs
    //let simulated_user_specified_value = 10;
    //let simulated_random_number = 7;
    //generate_workout(simulated_user_specified_value, simulated_random_number);

    //iterators.rs
    //no runnable examples

    //smart_pointers.rs
    //box_example();
    //deref_example();
    //drop_example();
    //rc_example();
    //rc_and_refcall_example();

    //reference_cycles.rs
    //ref_cycle_example();
    //node_example();

    //concurrency.rs
    //thread_example_one();
    //thread_example_two();
    //move_thread_example();
    //message_passing_example();
    //message_passing_example_two();
    //multi_message_sending_example();
    //mutex_example();
    //shared_mutex_thread_example();

    //blog.rs
    //let mut post = Post::new();

    //post.add_text("I ate a salad for lunch today");
    //assert_eq!("", post.content());

    //post.request_review();
    //assert_eq!("", post.content());

    //post.approve();
    //assert_eq!("I ate a salad for lunch today", post.content());

    //blog_implementation_two.rs
    //let mut post = newPost::new();

    //post.add_text("I ate a salad for lunch today");

    //let post = post.request_review();

    //let post = post.approve();

    //assert_eq!("I ate a salad for lunch today", post.content());

    //patterns_and_matching.rs
    //if_else_else_if_let_example();
    //while_loop_example();
    //for_loop_example();
    //function_parameters_example();
    //destructing_example()


    //unsafe_rust.rs
    //raw_pointers();
    //static_variables();

    //advanced_traits.rs
    //test_point_addition();
    //calling_same_named_functions();
    //print_names();
    //test_fmt_and_print();
    //print_test_newtype();

    //advanced_functions_and_closures.rs
    //function_pointer_example();

    //macros.rs
    macros_example();
}
