//-------------------------------------------------------------------------------------------
//---------------- ch15-01-box ---------------------
//-------------------------------------------------------------------------------------------

use crate::smart_pointers::List::{Cons, Nil};
use std::ops::Deref;
use std::rc::Rc;

// Attempt at enum to represent a cons list data struct
// of type i32
enum List {
    // Changed from Box to rx for ch15-04-rc
    Cons(i32, Rc<List>),
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
    // Note: Changed from Box to Rc for the examples in ch15-05-rc
    let list = Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil))))));
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

//-------------------------------------------------------------------------------------------
//---------------- ch15-03-drop ---------------------
//-------------------------------------------------------------------------------------------

// Simple strut to display example
struct CustomSmartPointer {
    data: String,
}

// Implementing the Drop trait and it's single fn 'drop'
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomerSmartPointer with data `{}`", self.data)
    }
}

pub fn drop_example() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomerSmartPointers created");
    // Rust will automatically call drop here when the values 'c' and 'd'
    // go out of scope thus printing the message in the implemented drop
    // function

    // The below isn't allowed as explicit calls to the destructor drop aren't valid
    // c.drop();

    // This is valid however, as it calls the std::mem:drop function
    drop(c);
    println!("CustomSmartPointer dropped before the end of fn")
}

//-------------------------------------------------------------------------------------------
//---------------- ch15-04-rc ---------------------
//-------------------------------------------------------------------------------------------

// Conceptually the example about to b show looks like this
// b ------> |3| |
//               \
//     a ------> |5| |----->|10| |------->|Nil|
//               /
// c ------> |4| |

pub fn rc_example() {
    // required to be created as a Rc::new instance instead of Cons(x,..) to be referenced
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    // Note: Rc::clone only increases the reference count
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

//-------------------------------------------------------------------------------------------
//---------------- ch15-05-interior-mutability ---------------------
//-------------------------------------------------------------------------------------------

fn int_mut_example() {
    let x = 5;
    // borrowing rules ensure you can't borrow a immutable value mutably
    // let y = &mut x;
}

// interior mutability example below
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value : usize,
    max: usize,
}

impl <'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_from_max = self.value as f64 / self.max as f64;

        if percentage_from_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_from_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of the quota!");
        } else if percentage_from_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of the quota!");
        }

    }
}

// example to test interface
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::borrow::Borrow;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // borrow_mut gets a mutable borrow of the type within self which is a RefCell of vec
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }
    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);
        // borrow here performs an immutable borow of the RefCell sent_messages to allow calling len (unwrapping)
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

// Using Rc and RefCell to have multiple reference to immutable data which can be Mutated!
use std::cell::RefCell;
use crate::smart_pointers::List2::{Cons2,Nil2};

// example of List implemented above with RefCell wrapping value stored in Rc
#[derive(Debug)]
enum List2 {
    Cons2(Rc<RefCell<i32>>, Rc<List2>),
    Nil2
}

pub fn rc_and_refcall_example() {
    use crate::smart_pointers::List2;

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons2(Rc::clone(&value), Rc::new(Nil2)));
    let b = Cons2(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons2(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

}
