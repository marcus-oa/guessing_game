use std::fmt::{Display, Debug};

// Trait example
// Summary trait with a single
// summarize behaviour to be implemented
// by structs that use this trait
// Note: Extended with default implementation
// for structs that impl the trait
// with specifying an implementation
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// example struct
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Implementation of Summary for the NewsArticle struct
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!{"{}", self.author}
    }
    // commented out to show default behaviour
    /*
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
     */
}

// example struct
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// Implementation of Summary for the Tweet struct
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!{"@{}", self.username}
    }
    // commented out to show default implementation
    /*
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
     */
}

pub(crate) fn traits_example() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people"
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
            hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
}
// ----------------------------------------------------------------
// Traits - as parameters
// ----------------------------------------------------------------

// Example function that calls summarize on any type which
// implements the Summary trait
// Note: Also known as the 'impl trait' syntax
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize())
}

// The above function is defined with syntactic sugar for a
// longer form called trait bound show below
/*
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize())
}
 */

// multiple params which implement Summary but may
// be of different types makes use of 'impl trait' syntax
pub fn notify_different_types(item1: &impl Summary, item2: &impl Summary) {}

// for specifying the same type then 'trait bound' syntax is required
// to enforce this behaviour
pub fn notify_same_types<T: Summary>(item1: &T, item2: &T) {}

// We can also specify 'Multiple' trait bounds using
// the '+' syntax
pub fn notify_multiple_traits(item: &(impl Summary + Display)) {}

// below example uses the more verbose trait bound syntax
// pub fn notify_multiple_traits<T: Summary + Display>(item: &T) {}


// example of multiple trait bounds getting verbose
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> () {}

// clearer trait bounds with the where clause example
fn some_function_clearer<T, U>(t: &T, u: &U) -> ()
    where T: Display + Clone,
          U: Clone + Debug
{}

// example of returning types that implement a trait
// Note: The calling code doesn't necessarily know
// the function returns type some T just that it
// implements Summary
// Another note: Syntax only works for returning a single type
// i.e. we can't return a Tweet or NewsArticle from the above code
// in this block just because they both impl Summary
fn return_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}





