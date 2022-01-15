// section 6.1

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("We're done")
    }
}

fn main() {
    let my_message = Message::Write(String::from("Hello There!"));
    my_message.call();

    // Using Option<T>
    let some_number = Some(5); //some i32 --> Option<i32>
    let some_string = Some("a string"); // some &str --> Option<&str>

    let absent_number: Option<i32> = None; // None but it is i32
}
