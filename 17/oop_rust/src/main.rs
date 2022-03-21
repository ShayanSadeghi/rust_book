// characteristics of an OOP programming language:
// 1. Object packages that have data --> structs and enums in Rust
// 2. Procedures that operate on data(methods) -->  impl
// 3. Inheritance --> there is no concept in Rust for inheritance, but there are some solutions that depends on our reason to use inheritance
// 4. encapsulation --> private and public fields in Rust

use oop_rust::AveragedCollection;

fn main() {
    let mut my_object = AveragedCollection::new();
    my_object.add(5);
    my_object.add(10);
    my_object.add(2);
    println!("average of 5, 10 and 2 is: {}", my_object.average());
    my_object.remove();
    println!("average of 5 and 10 is: {}", my_object.average());
}
