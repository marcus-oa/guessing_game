pub(crate) fn string_examples() {
    // creates an empty string s to load data into
    let mut s = String::new();

    // start string with initial data
    let data = "initial contents";

    let s1 = data.to_string();

    // to_string working on string literals directly
    let s2 = "initial contents".to_string();

    // equivalent to above code (creates a string from a string literal)
    let s3 = String::from("initial contents");

    // example of appending to a string
    let mut s4 = String::from("foo");
    s4.push_str("bar");

    // example demonstrating the ownership problem being avoid
    // as push_str takes a string slice and not ownership of the param
    let mut s5 = String::from("foo");
    let s6 = "bar";
    s5.push_str(s6);
    println!("s5 is {}", s5);

    // push takes a single param
    let mut s7 = String::from("lo");
    s7.push('l');

    // combining two EXISTING strings
    let s8 = String::from("Hello, ");
    let s9 = String::from("world!");
    let s10 = s8 + &s9; // note: s1 has been moved here and can no longer be used

    // multiple string concat example become unwieldly
    let s11 = String::from("tic");
    let s12 = String::from("tac");
    let s13 = String::from("toe");

    let s = s11 + "-" + &s12 + "-" + &s13;

    // for complicated string concatentations or combination
    // we use format!
    // note: This method doesn't take ownership of any of the strings
    let s14 = String::from("tic");
    let s15 = String::from("tac");
    let s16 = String::from("toe");
    let s17 = format!("{}-{}-{}", s14, s15, s16);

    let s18 = String::from("hello");
    //let h = s18[0]; note: commented out as invalid but showing a point

    // Slicing strings
    let hello = "Здравствуйте";
    // utf-8 encoding for special characters takes
    // up 2 bytes per character so below returns Зд
    let hello_slice = &hello[0..4];

    println!("{}", hello_slice);

    // iterating over strings safely
    // .chars is the key
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // byte iteration example
    for c in "नमस्ते".bytes() {
        println!("{}", c)
    }

}