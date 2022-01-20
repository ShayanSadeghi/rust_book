use std::fs::File;

fn main() {
    let f = File::open("test.txt"); // if file doesn't exist, there is no default panic. it is a recoverable error
    let f = match f {
        Ok(file) => file, // return file if it exists in root directory
        Err(error) => panic!("Problem opening the file: {:?}", error), // panic if any error occurred
    };
}
