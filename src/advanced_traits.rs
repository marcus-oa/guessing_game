//-------------------------------------------------------------------------------------------
//---------------- ch19-03-advanced-traits ---------------------
//-------------------------------------------------------------------------------------------

use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// this example displays the fact that the RHS of the Add
// (in this instance Point + Point) has an assumed type of Self
// which is derived from the LHS
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub fn test_point_addition() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 },
    )
}

struct Millimeters(u32);
struct Meters(u32);

// Below shows that we can change the default type of RHS to
// whatever we want and implement for it, so in this instance we've created
// a RHS where Meters is added to Millimeters
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// same named functions from
// different traits

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking!.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*")
    }
}

pub fn calling_same_named_functions () {
    let person = Human;
    Wizard::fly(&person);
    Pilot::fly(&person);
    person.fly();
}

// Fully qualified name for disambiguation

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("Puppy")
    }
}

pub fn print_names() {
    // prints dogs implementation
    println!("A baby dog is called a {}", Dog::baby_name());

    // Doesn't work
    //println!("A baby dog is called a {}", Animal::baby_name());

    // correct syntax to get beyond default baby_name set impl
    // this tells rust we want the method from Animal implemented on Dog
    // i.e. treat the Dog as a type of Animal
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

// traits, supertraits and dependant traits

use std::fmt;

// by defining the dependence on Display
// we can use the to_string method on self
// which is implemented for any type that
// implements displat
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*"," ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*"," ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));

    }
}

// implement Display for point so that we can impl
// the trait defined above which requires the Display
// to be implemented by any type that uses the trait
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// this wouldn't work without the above
impl OutlinePrint for Point {}

pub fn test_fmt_and_print() {
    let p = Point {x: 1, y: 3};

    p.outline_print()
}

// newtype pattern
// to implement external traits on external types
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

pub fn print_test_newtype() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);

    println!("w = {}", w);
}