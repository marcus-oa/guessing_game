//-------------------------------------------------------------------------------------------
//---------------- ch15-01-box ---------------------
//-------------------------------------------------------------------------------------------

use crate::smart_pointers::List::{Cons, Nil};

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