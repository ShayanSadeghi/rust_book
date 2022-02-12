use std::env;
use std::fs;

struct Config {
    query: String,
    filename: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong during file reading");
    println!("Text is:\n{}", contents)
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}
