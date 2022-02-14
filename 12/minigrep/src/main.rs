use std::env;
use std::error::Error;
use std::fs;
use std::process;

struct Config {
    query: String,
    filename: String,
}
impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not Enough Arguments!");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //we just use run function for its side effect (Error)
    let contents = fs::read_to_string(config.filename)?;

    println!("With text\n{}", contents);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem passing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application Error: {}", e);
        process::exit(1);
    }
}
