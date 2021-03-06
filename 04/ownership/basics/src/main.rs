fn main() {
    {
        // s defined in this scope, after closing the scope, s will drop and it won't be usable;
        let s = "Hello"; // this definition doesent allow to have a mutable string (to use push or pop)
        println!("{}", s);
    }

    let mut s = String::from("Hello"); // in this way we have a mutable string (Note we should "mut" and " String::from("text") " together )
    s.push_str(", world!");
    println!("{}", s);

    let x = 5;
    let y = x; // y will copy x value, it is a copy, not a move

    println!("x value is {} and y value is {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1; //s1 will move to s2 (s1 is unavailable now)

    println!("s2: {}", s2);

    //if we want to has both variables available, we can use clone()
    let s3 = s2.clone();
    println!("s2:{} , s3: {}", s2, s3);

    take_ownership(s3); // s3 is no more available after passing it to this function

    let s2 = take_and_give_ownership(s2);
    println!("s2 value after passing and returning in a function: {}", s2);
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn take_and_give_ownership(some_string: String) -> String {
    println!("This function give back the ownership...");
    some_string // returning expression
}
