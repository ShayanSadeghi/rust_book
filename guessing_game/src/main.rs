use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please Enter your guess.");

    let mut guess = String::new(); // in Rust variables are immutable by default; use "mut" to make them mutable
                                   // "::" indicates that "new" is an associated function of "String" type

    io::stdin()
        .read_line(&mut guess)
        .expect("Field to read line");

    println!("You guess: {}", guess);
}
