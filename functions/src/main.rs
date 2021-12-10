fn main() {
    println!("Hello, world!");

    time_message(5, 'h');
}

fn time_message(x: i32, label: char) {
    println!("The time is {}{}", x, label);
}
