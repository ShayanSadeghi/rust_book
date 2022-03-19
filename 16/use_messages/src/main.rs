use std::sync::mpsc; // mpsc = multiple producer, single consumer
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel(); //create a new channel --  transmitter(tx), receiver(rx)

    thread::spawn(move || {
        // create a new thread
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap(); // send method returns a Result<T, E>
            thread::sleep(Duration::from_secs(1));
        }
    });

    for recieved in rx {
        // let recieved = rx.recv().unwrap(); // wait until a value is sent down the channel
        println!("Got: {}", recieved);
    }
}
