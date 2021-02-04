//-------------------------------------------------------------------------------------------
//---------------- ch15-01-box ---------------------
//-------------------------------------------------------------------------------------------

use crate::smart_pointers::List::{Cons, Nil};
use std::ops::Deref;

// Attempt at enum to represent a cons list data struct
// of type i32
enum List {
    Cons(i32, Box<List>),
    Nil,
}
pub fn box_example() {

    // Box example to store the i32 value '5'
    // on the heap
    let b = Box::new(5);
    println!("b = {}", b);
}

pub fn cons_example() {

    // Fails with 'Cons(i32, List)' -> Cons(1, Cons(2, Cons(3, Nil)))
    // implementation of enum
    // as it can't discern the size of the enum
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

//-------------------------------------------------------------------------------------------
//---------------- ch15-02-deref ---------------------
//-------------------------------------------------------------------------------------------
pub fn deref_example() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    // dereference operator to allow 'integer = integer' comparison
    assert_eq!(5, *y);
}

// example of using Box<T> like a reference
pub fn deref_box_example() {
    let x = 5;
    // Box::new(x) create a copy of x on the heap
    let y = Box::new(x);

    assert_eq!(5, x);
    // dereference here allows the Box to follow to the pointer
    // on stack
    assert_eq!(5, *y);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Deref implemented for struct MyBox<T>
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn my_box_example() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // fails as we haven't implemented Deref trait to enable * deref operator usage\
    assert_eq!(5, *y);
}

pub fn deref_coercion_example() {
    fn hello(name: &str) {
        println!("Hello, {}!", name);
    }

    let m = MyBox::new(String::from("Rust"));
    // example show deref coercion where
    // &MyBox<T> -> &String -> &str
    hello(&m);
}