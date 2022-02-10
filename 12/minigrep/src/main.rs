use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1]; // the first argument (args[0]) is the name of our binary
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
