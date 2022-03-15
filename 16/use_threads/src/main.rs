use std::thread;
use std::time::Duration;

// this code may have different output each time
fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("Spawn number: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // program stopped when main thread ends
    for i in 1..5 {
        println!("Main number {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}
