use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5); // Mutex<T> is a smart pointer

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    } // lock release automatically

    println!("m = {:?}", m);
}
