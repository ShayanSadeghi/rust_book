fn main() {
    println!("Hello, world!");

    let x = plus_one(5);
    println!("The value is {}", x)
}

fn plus_one(x: i32) -> i32 {
    x + 1 //it is an expression not a statement, so we should not to use ';' at the end
}
