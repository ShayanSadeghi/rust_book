fn main() {
    let mut s1 = String::from("Hello"); //define a mutable variable
    let r1 = &s1; //we can have multiple immutable referencing
    let r2 = &s1;
    // after immutable references we can't define a new mutable reference
    println!("values are {}, {}", r1, r2);
}
