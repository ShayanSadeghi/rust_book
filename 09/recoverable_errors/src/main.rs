use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("test.txt"); // if file doesn't exist, there is no default panic. it is a recoverable error
    let f = match f {
        Ok(file) => file, // return file if it exists in root directory
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("test.txt") {
                // if it is not exist handle it with creating a new file
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                // if there is other errors, ex. no access to read file, handle by this arm
                panic!("Problem opening the file {:?}", other_error)
            }
        },
    };
    // check unwrap and except to do this things in a more concise way
}
