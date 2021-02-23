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

//-------------------------------------------------------------------------------------------
//---------------- ch18-02-refutability ---------------------
//-------------------------------------------------------------------------------------------

fn irrefutable_example() {
    let some_option_value: Option<i32> = None;

    // doesn't compile as the None case isn't handled (and can't be)
    // let Some(x) = some_option_value;

    // compiles!
    if let Some(x) = some_option_value {
        println!("{}", x);
    }

    // equally, this compile but highlights that it isn't necessary
    // as it doesn't make sense to use if let for an irrefutable pattern bind
    if let x = 5 {
        println!("{}", x)
    }

}

//-------------------------------------------------------------------------------------------
//---------------- ch18-03-pattern-syntax ---------------------
//-------------------------------------------------------------------------------------------

// useful if you want your code to take action
// if it gets a particular concrete value
fn matching_literals() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything")
    }
}

fn matching_named_variables() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // binds to any Some so this line print
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    // y binding in match statement gone at this point
    // so prints outer x and y
    println!("at the end: x = {:?}, y = {:?}", x, y)
}

// prints 'one or two' as
// 1 matches 1 | 2
fn matching_multiple_patterns() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything")
    }
}

fn matching_ranges() {
    let x = 5;

    match x {
        // matches the range inclusive of 1 .. 5
        // essentially the same as 1 | 2 | 3 | 4 | 5
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        // inclusion and ranges work with characters too!
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("later ASCII letter"),
        _ => println!("something else"),
    }
}

pub fn destructing_example() {
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    // her we use the let PATTERN = EXPRESSION syntax
    // to have a and b bind to p's x and y values
    // Note: Shorthand example used with long form commented out
    // let Point { x: a, y: b } = p;
    let Point {x, y} = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // matching destructed patterns below
    match p {
        Point {x, y: 0} => println!("On the x axis at {}", x),
        Point {x: 0, y} => println!("On the y axis at {}", y),
        Point {x, y} => println!("On neither axis: ({}, {})", x, y),
    }
}