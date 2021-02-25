//-------------------------------------------------------------------------------------------
//---------------- ch19-01-unsafe-rst ---------------------
//-------------------------------------------------------------------------------------------

// mainly typing out unsafe rust without comments
// better to get used to the syntax than to fully
// understand usage right now as it is left field
// of what I will be doing in rust initially

pub fn raw_pointers() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

}

fn unsafe_func_and_method() {
    unsafe fn dangerous() {}

    // This call doesn't work as the unsafe function isn't wrapped
    // in an unsafe call body
    // dangerous();

    unsafe {
        dangerous()
    }
}

fn safe_abstractions_unsafe_code() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);
}


fn extern_example() {
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3))
    }
}

pub fn static_variables() {
    static HELLO_WORLD: &str = "Hello, world!";

    {
        println!("name is: {}", HELLO_WORLD);
    }

    static mut COUNTER: u32 = 0;

    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER)
    }
}