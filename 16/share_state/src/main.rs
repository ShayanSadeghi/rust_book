use std::sync::{Arc, Mutex};
use std::thread;

// as we used RefCell<T> to mutate contents inside a RC<T>, we use Mutex<T> to mutate contents inside an Arc<T>

fn main() {
    // we have to use Arc instead of Rc, because Arc is safe to use in concurrent situations
    let counter = Arc::new(Mutex::new(0)); // Arc = atomic RC
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); //mutex provides interior mutability
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
