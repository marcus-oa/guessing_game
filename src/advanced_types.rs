//-------------------------------------------------------------------------------------------
//---------------- ch19-04-advanced-types ---------------------
//-------------------------------------------------------------------------------------------

// type synonyms and type aliases
// aliases are often used to avoid
// type repetition for longer types
fn alias_example() {
    type Kilometers = i32;


    let x: i32 = 5;
    let y: Kilometers = 5;

    // works as y and x are the same type
    // albeit Kilometers is an alias of type i32
    println!("x + y = {}", x + y);
}

fn longer_types_alias() {
    type Thunk = Box<dyn Fn() + Send + 'static>;

    // See the reduced repetition below
    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {}

    fn returns_long_type() -> Thunk {
        unimplemented!()
    }
}

// The Never Type
