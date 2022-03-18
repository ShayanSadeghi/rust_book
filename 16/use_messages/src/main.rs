use std::sync::mpsc; // mpsc = multiple producer, single consumer
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel(); //create a new channel --  transmitter(tx), receiver(rx)

    thread::spawn(move || {
        // create a new thread
        let val = String::from("hi");
        tx.send(val).unwrap(); // send method returns a Result<T, E>
    });

    let recieved = rx.recv().unwrap(); // wait until a value is sent down the channel
    println!("Got: {}", recieved);
}
