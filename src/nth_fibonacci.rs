use std::io;

fn nth_fibonacci(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => nth_fibonacci(n - 1) + nth_fibonacci(n - 2)
    }
}

pub(crate) fn get_fibonacci() {
    println!("What number of the fibonacci sequence would you like to get?");

    let mut answer = String::new();

    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");

    let answer: i32 = answer.trim().parse().expect("Please enter a number");

    println!("Nth fibonacci number for {} is {}", answer, nth_fibonacci(answer));
}