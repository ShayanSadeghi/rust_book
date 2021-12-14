fn main() {
    let mut s1 = String::from("Hello"); //define a mutable variable

    let r1 = &s1; //we can have multiple immutable referencing
    let r2 = &s1;
    println!("values are {}, {}", r1, r2);
    // because out immutable references will not be used after this point, we can create a mutable reference without any error
    let r3 = &mut s1;
    println!("values are {}", r3);
}
