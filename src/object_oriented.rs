pub struct AveragedCollection {
    // left private so no way to directly call internal data structure
    list: Vec<i32>,
    average: f64,
}

// implementing public functions which add, remove and average
// to the stored collection
impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    // left completely private (encapsulated)
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

//-------------------------------------------------------------------------------------------
//---------------- ch17-02-trait-objects ---------------------
//-------------------------------------------------------------------------------------------

// trait object implementations like this allow for multiple concrete
// types at runtime
// Note: Doing the below with generics will only allow one concrete type at
// runtime (Example below)
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run (&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// generics example
pub struct ScreenGeneric<T: Draw> {
    pub components: Vec<T>,
}

impl<T> ScreenGeneric<T>
where
    T: Draw,
{
    pub fn run (&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// Defining the Button struct to show
// the implementation of the Draw trait
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        unimplemented!()
    }
}

// Defining the SelectBox to show
// the implementation of the Draw trait
pub struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        unimplemented!()
    }
}

// Creating a screen instance to show the implementation
// of Draw components
fn screen_defined_example() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

// screen example that won't work
// String doesn't implement Draw and thus won't compile
/*
fn screen_defined_error_example() {
    let screen = Screen {
        components: vec![Box::new(String::from("Hi"))],
    };

    screen.run();
}
 */

// object safety and trait objects
// There are two rules for objct-safe traits in trait objects
// - Return type cannot be self
// - there are no generic type parameters
// The following won't compile as rule one is invalidated:
// Clone trait has a fn clone which returns 'self'
/*
pub trait Clone {
    fn clone(&self) -> Self;
}
 */

