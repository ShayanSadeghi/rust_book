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
}
