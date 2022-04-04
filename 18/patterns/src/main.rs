fn main() {
    // (if let)-(else if let) example to set a background color
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("favorite color is activated, color: {}", color);
    } else if is_tuesday {
        println!("Tuesday is green day")
    } else if let Ok(age) = age {
        // "age" is a shadow variable, which we define just now
        if age > 30 {
            println!("using purple")
        } else {
            println!("using orange")
        }
    } else {
        // (if let) is not exhaustive, so if we forget this last (else) compiler wouldn't alert us, unlike the (match)
        println!("using blue")
    }

    // while let -- print in reverse order
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        // if stack.pop returns a (None), while loop will stop
        println!("{}", top)
    }

    // for
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // let--- let is a pattern!!! :))
    // let PATTERN = EXPRESSION
    let x = 5;
    let (x, y, z) = (1, 2, 3); // it's also a pattern
                               //function parameters are patterns too

    // refutable and irrefutable
    let some_option_value: Option<i32> = None;
    // the following code cause a compile time  error
    // let Some(x) = some_option_value; // "let" only accept irrefutable patterns, but "Some(x)" can be refutable. ex. have None value
    if let Some(x) = some_option_value {
        // if "some_option_value" be "None", Rust will skip this section and continue code. It is a refutable pattern
        println!("{}", x);
    }
}
