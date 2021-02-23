//-------------------------------------------------------------------------------------------
//---------------- ch18-01-all-the-places-for-patterns ---------------------
//-------------------------------------------------------------------------------------------

// if let, else if, else if lets flexibility
// By using a combination of the above we can express matches
// more advanced (and possibly unrelated) than those expressed
// with match due to match statements only comparing the one
// statement at each arm

pub fn if_else_else_if_let_example() {
    let favourite_colour: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(colour) = favourite_colour {
        println!("Using your favourite colour, {}, as the background", colour);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background colour");
        } else {
            println!("Using orange as the background colour");
        }
    } else {
        println!("Using blue as the background colour");
    }
}

// while loop example
pub fn while_loop_example() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    // this loop continues until we no longer see
    // a Some (The eventual None when all items are 'popped')
    while let Some(top) = stack.pop() {
        println!("{}", top)
    }
}

// for loop pattern example
pub fn for_loop_example() {
    let v = vec!['a','b','c'];

    // for [pattern] in [expression]
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index)
    }
}

// function parameters are patterns too!
pub fn function_parameters_example() {
    fn point_coordinates(&(x, y): &(i32, i32)) {
        println!("Current Location: ({}, {})", x, y);
    }

    let point = (3,5);
    point_coordinates(&point);
}