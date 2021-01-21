// Simple function to find the largest number of type i32
// To be refactored into a generic function
pub(crate) fn generics_example() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    // If we wanted to find the largest number across
    // two different number sets we can duplicate the
    // above code (inefficient but to show a point)
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest)
}

// So lets create a function to reduce code duplication
// of the same operation
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Beginning to see where generics can be applied
// here as the only difference between
// this function on the function for
// i32 is the function signatures
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}


// The generic type function
// signature of function reads as:
// - function largest is generic over some type T
// - list is a slice of type T
// - returns a reference to some type T (Same type throughout)
//
// <T: std::cmp::PartialOrd> = Bounds generic type T to
// only work on types that implement PartialOrd
// Errors otherwise
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}


pub(crate) fn largest_i32_using_func() {
    let number_list = vec![34, 50, 25, 100, 65];

    // providing the list to the function reducing
    // code duplication on the way to generics
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
}

pub(crate) fn largest_char_using_func() {
    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result)
}

// The generic implementation of the above function calls
pub(crate) fn largest_generic() {
    let number_list = vec![34, 50, 25, 100, 65];

    // Function can be provided type i32
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    // Function can be provided type char
    let result = largest(&char_list);
    println!("The largest char is {}", result)
}

// ----------------------------------------------------------------
// Structs
// ----------------------------------------------------------------

// Structs can also use generics
struct Point<T> {
    x: T,
    y: T,
}

fn structs_example() {
    // Using generic struct to implement i32 Point
    let integer = Point { x: 5, y: 10 };
    // Using generic struct to implement float Point
    let float = Point { x: 1.0, y: 4.0 };

    // Won't work as struct generic only implements ONE generic type
    // for its two values
    // let wont_work = Point { x: 5, y: 4.0};
}

// lets implement a Struct that will allow
// different types for the coordinate values
struct Point2<T,U> {
    x: T,
    y: U
}

// generic methods can be implemented on structs and enums
// as show below
// Returns a reference to the value x of the struct Point
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn structs_example_with_mixed_coordinate_types() {
    let both_integer = Point2 { x: 5, y: 10 };
    let both_float = Point2 { x: 1.0, y: 4.0 };
    let integer_and_float = Point2 { x: 5, y: 4.0 };
}

pub(crate) fn impl_generic_example() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
