use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1]; // the first argument (args[0]) is the name of our binary
    let filename = &args[2];

    let contents = fs::read_to_string(filename).expect("Something went wrong during file reading");
    println!("Text is:\n{}", contents)
}
