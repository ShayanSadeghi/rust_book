use std::thread;
use std::time::Duration;

// to say rust wait until the spawn thread finish too, we use join
fn main() {
    let handle = thread::spawn(|| {
        // thread::spawn returns a JoinHandle
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

    handle.join().unwrap(); // calling join on a JoinHandle tell rust to wait for its thread to finish
}
