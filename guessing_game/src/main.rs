use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101); // to get a number between 1-100 we also can use 1..=100
    println!("The secret numbere is {}", secret_number);
    println!("Please Enter your guess.");

    let mut guess = String::new(); // in Rust variables are immutable by default; use "mut" to make them mutable
                                   // "::" indicates that "new" is an associated function of "String" type

    io::stdin()
        .read_line(&mut guess)
        .expect("Field to read line");

    println!("You guess: {}", guess);
}
