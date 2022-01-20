// section 9.2
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

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

    // propagating error
    let user_name = read_username_from_file_chaining();
    println!("{:?}", user_name)
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("my_data.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e), // early error return
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        // we do not need using return keyword, because this is last expression
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_concise() -> Result<String, io::Error> {
    let mut f = File::open("my_data.txt")?; // it there is an error, rust make an early error return.
    let mut s = String::new();
    f.read_to_string(&mut s)?; //check for any error again
    Ok(s) // returning value if everything goes well
}

fn read_username_from_file_chaining() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("my_data.txt")?.read_to_string(&mut s)?; // chaining
    Ok(s)
}
