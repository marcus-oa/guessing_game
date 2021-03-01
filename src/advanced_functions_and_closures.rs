//-------------------------------------------------------------------------------------------
//---------------- ch19-04-advanced-functions-and-closures ---------------------
//-------------------------------------------------------------------------------------------

// function pointers
fn add_one(arg: i32) -> i32 {
    arg + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

pub fn function_pointer_example() {
    // we pass in the fn pointer add_one as a param
    // for a function which accepts it as an argument
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer)
}

// example of map accepting a closure or fn pointer
fn function_closure_or_pointer() {
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter().map(|i| i.to_string()).collect();

    let list_of_strings_fn: Vec<String> = list_of_strings
        .iter().map(ToString::to_string).collect();
}

// example showing enum struct being accepted as closures
// to create a list of said struct tuple
fn enums_accepting_closures_fn() {
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}

// returning closures example
// we can't return closures on their type alone
// but we can with a trait object
fn return_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}