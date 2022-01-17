// section 8.2

fn main() {
    // Creating String
    let mut s = String::new(); // create new empty String
    let hello_in_persian = String::from("سلام"); // Rust strings use utf-8 encoding. creating a String and initiate it
    println!("{}", hello_in_persian);
    let my_str = "initial text".to_string();
    println!("{}", my_str);

    // updating String
    s.push_str("Shayan"); // push_str add String
    println!("{}", s);

    let mut s1 = String::from("lo");
    s1.push('l'); // push add char
    println!("{}", s1);

    //Concat String
    let s2 = String::from("Hello, ");
    let s3 = String::from("World");
    let s4 = s2 + &s3; // the "+" operator get the ownership of first element and reference to the second element
                       // s2 is no longer available
                       //    but s3 is available
    println!("s4 is {}", s4);
    println!("s3 is {}", s3);

    let s5_1 = String::from("tic");
    let s5_2 = String::from("toc");
    let s5_3 = String::from("toe");
    let s5 = format!("{}-{}-{}", s5_1, s5_2, s5_3); //"format!" macro is like "println!"
    println!("{}", s5);

    //Slicing String
    let s = String::from("شایان");
    let sub_s = &s[0..2];
    println!("{}", sub_s);

    //iterating over string
    //on chars
    for c in "شایان".chars() {
        println!("{}", c);
    }
    // on bytes
    for b in "shayan".bytes() {
        println!("{}", b)
    }
}
