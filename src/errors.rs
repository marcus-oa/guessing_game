pub(crate) fn errors_examples() {
    // commented out fo further example
    // straight example of a panic being executed by the sys
    //panic!("crash and burn")

    // out of bounds panic example
    let v = vec![1,2,3];

    v[99];
}

use std::fs::File;
use std::io::{ErrorKind, Read};
pub(crate) fn errors_examples2() {

    // Example to show return type is type Result<std::fs::File, std::io::Error>
    // The case when we attach a type of u32 i.e. let f: u32
    let f = File::open("hello.txt");

    // handling error cases of file reads
    // commented out as it will error before latter examples can run
    /*
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error)
    };
     */

    // Matching on different errors example
    // expanding on the response in error cases
    let f2 = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e)
            },
            other_error => {
                panic!{"Problem opening the file: {:?}", other_error}
            }
        },
    };

    // more concise rewrite using closures
    // less match statements with the same functionality as the code above
    let f3 = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("Hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!{"Problem opening the file: {:?}", error};
        }
    });

    // unwrap shortcut. Same effect as match with Ok and Err calling panic
    let f4 = File::open("hello.txt").unwrap();

    // expect is another useful shortcut that lets us provide the panic error message
    let f5 = File::open("hello.txt").expect("Failed to open hello.txt");
}

use std::io;

// Propagating errors example
// Passing the error 'up' to the calling function
pub(crate) fn read_username_from_file() -> Result<String, io::Error> {
    // attempt to open a file
    let f = File::open("hello.txt");

    // handle the Ok case and Err case
    // This time 'return' the err to the calling function
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    // mutable s to store the read of the file
    let mut s = String::new();

    // attempt to read the file and store contents in s
    // if f throws error we throw it again
    // no explict 'return' here as last call in function
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// ? operator example for shortcuts for propagation
pub(crate) fn read_username_from_file_shortcut() -> Result<String, io::Error> {
    // works the same as match after instanting a value of type Result
    // however it wraps all the code and propagates in the same way (less verbose)
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// Same as above but even shorter hand
pub(crate) fn read_username_from_file_even_shorter() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// EVEN SHORTER!
// using the std::fs library
// such a common operation Rust provides a nice function for it
use std::fs;
pub(crate) fn read_username_from_file_fs() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
