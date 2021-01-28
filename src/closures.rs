use std::thread;
use std::time::Duration;

pub fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

pub fn generate_workout(intensity: u32, random_number: u32) {
    // A closure
    // Contains the definition of a function stored to be
    // used later
    // Note: Closures don't require type annotations
    let expensive_result = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    if intensity < 25 {
        println!("Today, do {} pushups", expensive_result(intensity));
        println!("Next, do {} situps!", expensive_result(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!")
        } else {
            println!("Today, run for {} minutes!", expensive_result(intensity));
        }
    }
}

pub fn closure_usage_example() {
    let example_closure = |x| x;

    // Compiles and the inference is that the closure
    // above accepts string
    let s = example_closure(String::from("hello"));

    // Doesn't compile as closure is already inferred to string
    // let n = example_closure(5);
}