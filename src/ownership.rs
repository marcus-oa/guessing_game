pub(crate) fn ownership_tests() {

    let s = String::from("hello");

    takes_ownership(s);

    let x = 5;

    makes_copy(x);

    println!("{}",first_word("Hello, World!"));

    fn takes_ownership(s: String) {
        println!("{}", s);
    }

    fn makes_copy(s: i32) {
        println!("{}", s);
    }

}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}