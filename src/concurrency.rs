use std::thread;
use std::time::Duration;

pub fn thread_example_one() {
    // spawns a single thread for use
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("THREAD EXAMPLE ONE:hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // uses the main thread as we haven't spawned a thread
    // for usage
    for i in 1..5 {
        println!("THREAD EXAMPLE ONE: hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // as type of handle variable is JoinHandle
    // and returns result when join is called on it.
    // Join waits for the thread to finish thus ensuring handle
    // finishes before we exist the main
    handle.join().unwrap()
}

pub fn thread_example_two() {
    // spawns a single thread for use
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("THREAD EXAMPLE TWO: hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // placing the call to join earlier blocks any other threads
    // from running until the handle thread finishes
    handle.join().unwrap();

    // uses the main thread as we haven't spawned a thread
    // for usage
    for i in 1..5 {
        println!("THREAD EXAMPLE TWO: hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

pub fn move_thread_example() {
    let v = vec![1, 2, 3];

    // move means the thread takes ownership of v
    // such that the reference println requires for v
    // will definitely be in scope
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // This won't work after move applied as error dictates we're trying
    // to use a moved value and the ownership rules won't allow it
    // drop(v);
    handle.join().unwrap()
}