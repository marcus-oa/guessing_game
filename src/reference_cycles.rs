//-------------------------------------------------------------------------------------------
//---------------- ch15-06-reference-cycles ---------------------
//-------------------------------------------------------------------------------------------

use crate::reference_cycles::List::{Cons,Nil};
use std::rc::{Rc,Weak};
use std::cell::RefCell;

// an enum of List
// implemented such that you can change the reference of the item at the tail
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

pub fn ref_cycle_example() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    // count is 1 and next item is 'Nil'
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    // count is now 2, 1 and reference to &a
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    // this creates a reference cycle as we have a referencing b referencing b indefinitely
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    // Count is now 2, 2 because of the cycle!
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see a that we have s cycle
    // it will overflow the stack
    // println!("a next time = {:?}", a.tail());
}

// Example of using weak to avoid the reference cycle issue

// Tree Data Example
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

pub fn node_example() {
    // we create a leaf node with no children
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent initially = {:?}", leaf.parent.borrow().upgrade());

    // leaf initially has a strong count of one
    // for the Rc::new we created on Node
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    // we create a branch node with one child, the leaf
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // this is how we add a weak reference to branch
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    // Branch has a strong of 1 and weak of 1,
    // the strong on itself at Rc::new node creation
    // and the weak reference to the child
    println!(
        "branch strong = {}, weak = {}",
        Rc::strong_count(&branch),
        Rc::weak_count(&branch)
    );

    // we see leaf has a strong count of one
    // the strong reference on itself at creation
    // and the branch clone reference to leaf
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    // and this is how we check a weak reference,
    // the value returned is an Option such that we get Some or None
    // if the reference is out of scope

    // Notice this doesn't return an infinite loop back and forth
    // between branch and leaf because of the Weak<Node> usage
    println!("leaf parent after binding = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    /*
    All of the above allow parent nodes to point to child nodes and vice
    verse without creating reference cycles or memory leaks
     */
}



