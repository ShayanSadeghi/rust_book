// section 8.3
use std::collections::HashMap;

fn main() {
    let mut person = HashMap::new();
    person.insert(String::from("name"), String::from("Shayan"));
    person.insert(String::from("age"), String::from("25")); //both keys and values for all instances should use the same type

    println!("{:?}", person);
}
