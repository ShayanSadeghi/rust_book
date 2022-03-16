use std::thread;
use std::time::Duration;

// to say rust wait until the spawn thread finish too, we use join
fn main() {
    let v = vec![1, 2, 3]; // v is in the main thread
                           // we use 'move' to force the closure to take the ownership of the values it's using rather than allowing rust to infer that it should borrow the values
    let handle = thread::spawn(move || {
        // thread::spawn returns a JoinHandle
        println!("the vector moved from main thread is: {:?}", v);
        for i in 1..10 {
            println!("Spawn number: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // if we call handle.join here, rust will wait for the first thread to finish then continue the program(something like single thread)

    // program stopped when main thread ends
    for i in 1..5 {
        println!("Main number {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    // we can not run this, because we moved 'v' before
    // println!("the vector in the main thread: {:?}", v);

    handle.join().unwrap(); // calling join on a JoinHandle tell rust to wait for its thread to finish
}
