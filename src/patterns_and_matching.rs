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

fn deconstructing_enums() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    // illustrating the different ways we can match on different enum
    // variants
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure")
        }
        Message::Move { x ,y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x, y
            )
        }
        Message::Write(text) => {
            println!("Text message: {}", text)
        }
        Message::ChangeColor(r, g, b) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r, g, b
            )
        }
    }
}

fn destructing_nested_structs() {
    enum Colour {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        // now a nested struct that uses
        // the Colour enum struct
        ChangeColor(Colour),
    }

    let msg = Message::ChangeColor(Colour::Hsv(0, 160, 255));

    // See how we match on two different Colour variants here
    // Example of matching on a nested struct
    match msg {
        Message::ChangeColor(Colour::Rgb(r, g, b)) => println!(
            "Change the colour to red {}, green {}, blue {}",
            r, g, b
        ),
        Message::ChangeColor(Colour::Hsv(h, s, v)) => println!(
            "Change the colour to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }
}

fn mix_and_match_destruct() {
    struct Point {
        x: i32,
        y: i32,
    }

    // showing a complex mix and match to destruct the right-hand side expression
    // into pattern on the left
    let ((feet, inches), Point {x, y}) =
        ((3, 10), Point { x: 3, y: -10});

}

fn ignoring_entire_values() {
    // compile doesn't moan about unused params as it
    // would for named parameters
    fn foo(_: i32, y: i32) {
        println!("This code only uses y parameter: {}", y)
    }

    // 3 is unused and ignored here
    foo(3,4)
}

fn ignoring_part_values() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    // This match takes a while to understand but is fairly simple
    // Business logic is to not overwrite an existing set of customization
    // so if either case is None then set with new_setting_value
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value")
        }
        _ => {
            setting_value = new_setting_value
        }
    }

    println!("setting is {:?}", setting_value);

    // extended example
    let number = (2, 4, 8, 16, 32);

    // by matching against all numbers we can conveniently pick
    // and choose which to name and match against
    match number {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }
}

fn ignoring_unused_variables() {
    // the underscore removes the compiler warning
    // usually used when prototyping ot starting
    // out a project
    let _x = 5;
    let y = 10;

    let s = Some(String::from("Hello!"));

    // using an Some(_s) takes ownership so better to just use '_' here
    // as shown below
    if let Some(_) = s {
        println!("Found a string")
    }

    println!("{:?}", s);
}

fn ignoring_remaining_parts() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0};

    // .. notation means we'll match x and ignore the rest of Point values
    // without explicitly stating _ or named values to match
    match origin {
        Point {x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);

    // .. can be used to match everything in-between as well
    match numbers {
        (first, .., last) =>  {
            println!("Some numbers: {}, {}", first, last)
        }
    }

    // the below is invalid as it's ambiguous: how many before and after do we match
    // for example?
    /*
    match numbers {
        (.., second, ..) => {
            println!("Some numbers: {}", second)
        }
    }
     */
}

fn extra_conditions_with_match() {
    let num = Some(4);

    match num {
        // conditional as first match within clause
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(10);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // with this clause we can test if the outer variable y
        // matches against the inner bound variable n which is x
        // essentially comparing out x and y in the inner match
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);

    // use of | (or) pattern match below
    let x = 4;
    let y = false;

    match x {
        // matches if x is 4, 5 or 6 AND and y is true
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

// @ binding example
// allows us to bind a variable AND test it matches some condition
fn pattern_bindings() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            // id stored in a variable id_variables for later use
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        // no storage of id variable so can't be used
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        },
        // id is useable because of shorthand syntax
        // but not usable above as we used some condition to test id
        // and no @ binding to store said variable
        Message::Hello { id } => println!("Found some other id: {}", id),
    }

}