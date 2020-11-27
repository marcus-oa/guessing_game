// Struct type (Chapter 5)
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Tuple structs (Structs with no named fields) for
// creating different types
struct color(i32, i32, i32);
struct point(i32, i32, i32);

// Used to derive the debug output
// of this struct.
// Allows the struct to be formatted
// for a println
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Generic function
pub(crate) fn fun() {
    
    // Struct instance
    let mut user1 = User {
        username: String::from("someone@example.com"),
        email: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername456"),
        // Shorthand for ..
        //sign_in_count: user1.sign_in_count,
        //active: user1.active,
        ..user1
    };

    let black = (0,0,0);
    let origin = (0,0,0);


}

// Builder function for the User struct
fn build_user(email: String, username: String) -> User {
    User {
        // username: username shorthand
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

// Struct example function
// Print the area of a predefined rectangle
pub(crate) fn print_rectangle_area() {
    let width1 = 30;
    let height1 = 50;

    println! (
        "The area of the rectangle is {} square pixels.",
        area(width1,height1)
    );
}

// Calculate area given width and height
fn area(width: u32, height: u32) -> u32 {
    width * height
}

// Calculate area given width and height (Different params)
fn area_rfct(rectangle: &Rectangle) -> u32 {
    rectangle.width* rectangle.height
}

// Refactor of ->  print_rectangle_area
// Utilizing tuples
pub(crate) fn print_rectangle_area_rfct() {
    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };

    println! (
        "The area of the rectangle is {} square pixels.",
        area_rfct(&rect1)
    );

    // rect1 println derived from the struct
    println! (
        "rect1 is {:#?}",
        rect1
    );
}


