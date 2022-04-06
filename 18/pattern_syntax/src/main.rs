fn main() {
    // Simple match
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    println!("----------------");
    // Shadowing problem
    let x = Some(5);
    // let x = None;
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched y = {:?}", y), //  This new y binding will match any value inside a Some, which is what we have in x
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x={:?}, y={:?}", x, y);
    println!("----------------");

    // Multiple patterns
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    println!("----------------");

    // Matching ranges --- just for numeric and char variables
    let x = 3;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("other values"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("first section"),
        'k'..='m' => println!("second section"),
        _ => println!("something else"),
    }
    println!("----------------");

    // Destructuring structs
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 6 };

    let Point { x: a, y: b } = p; // destructing to "a" and "b"
    println!("destructed values x={}, y={}", a, b);
    let Point { x, y } = p; // destructing to "x" and "y"
    println!("destructed values x={}, y={}", x, y);

    match p {
        Point { x, y: 0 } => println!("lies on the x axis at {}", x),
        Point { x: 0, y } => println!("lies on the y axis at {}", y),
        Point { x, y } => println!("lies on neither axis: ({}, {})", x, y),
    }
    println!("----------------");

    // Destructuring enums
    // define a new enum
    enum Message {
        Quit,
        Move { x: i32, y: i32 },    // struct
        Write(String),              //tuple
        ChangeColor(i32, i32, i32), //tuple
    }

    let msg = Message::ChangeColor(0, 100, 120);

    match msg {
        Message::Quit => {
            println!("the quit variant has no data to destructure")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }
    println!("----------------");

    // Destructuring Nested Structs and Enums
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    enum Message2 {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message2::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h, s, v
            )
        }
        _ => (),
    }
    println!("----------------");

    // Ignoring Values in Pattern
    // _

    let mut setting_value = Some(5);
    // let mut setting_value = None;
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }

    // ..
    struct Point2 {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point2 { x: 0, y: 0, z: 0 };

    match origin {
        Point2 { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    println!("----------------");

    // @ Bindings
    enum Message3 {
        Hello { id: i32 },
    }
    let msg = Message3::Hello { id: 5 };

    match msg {
        Message3::Hello {
            id: id_variable @ 3..=7, // test id be in range 3..=7 and also save it. use "@" operator to bind the value to the variable "id_variable"
        } => println!("Found an id in range: {}", id_variable),
        Message3::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message3::Hello { id } => println!("Found some other id: {}", id),
    }
}
