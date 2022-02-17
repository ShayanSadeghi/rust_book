use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem passing arguments: {}", err); // when we use eprintln! instead of println!, on the case of error rust returns and stderr instead of simple stdout,
                                                         // so if we try to put the result into a
                                                         // file, and there is and error, stderr
                                                         // will be shown, and if there is no
                                                         // error, we can save results to a file.

        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}
