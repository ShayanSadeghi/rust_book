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

    // destructuring
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
}
