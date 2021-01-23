// Lifetime annotation syntax
// &i32        - no lifetime, just a reference
// &'a i32     - a reference with an explicit lifetime
// &'a mut i32 - a mutable reference with an explicit lifetime

pub(crate) fn lifetime_example1() {
    // Not null as won't compile unless bound to a value
    // Keeps to Rust principles
    let r;
    {
        let x = 5;
        r = &x;
        // scope of x ends and therefore leaves r referencing a dereferences location
    }

    // Errors on 'x does nor live long enough'
    //println!("r: {}", r);
}

// Similar to generics, we express the constraint of the generic
// lifetime across all values we want it applied
// The below expresses that all references in the parameter
// and return value have the same lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// This works after the lifetime annotation has been
// added to the 'longest' function
pub(crate) fn lifetime_example2() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// example showing string1 valid within inner scope
// and result valid at point where it is called
// within the println statement
pub(crate) fn lifetime_example3() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result)
    }
}

// example where the println fails as string2 does not live
// long enough
// value is borrowed in result but result has a lifetime
// of the shortest lived lifetime of values passed into the 'longest'
// function
// Note: lifetimes default to the shortest lifetime of the values
// passed into a function
/*
pub(crate) fn lifetime_example4() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result)
}
 */

// example of 'longest' which won't compile
// due to returning a reference to a local variable
// i.e. a dangling reference as the value of the variable
// becomes out of scope at the end of the function
/*
pub(crate) fn longest2<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
 */

// ----------------------------------------------------------------
// Lifetimes - structs using references
// ----------------------------------------------------------------

// Same syntax as prior lifetime implementations
// Note: All references in a struct need a lifetime annotation
// when implemented in a struct
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // first lifetime elision rule in effect
    fn level(&self) -> i32 {
        3
    }

    // third lifetime elision rule in effect
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// example demonstrating struct reference lifetimes
// the instance of ImportantExcerpt cannot outlive
// the reference it holds in the part field
// Below is valid as 'novel' value doesn't go out
// of scope until AFTER the ImportantExcerpt goes out of scope
pub(crate) fn lifetime_struct() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could nor find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence
    };
}

// ----------------------------------------------------------------
// Lifetimes - tying it all togethet (Type Parameters, Trait Bounds, Lifetimes)
// ----------------------------------------------------------------
use std::fmt::Display;

// example of the longest function from earlier
// we now provide a value ann which can be any type that implements
// the Display trait (Via the where clause)
// Note: as lifetimes are a type of generic like type T they
// can be declared in the same angle brackets as shown belo
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

