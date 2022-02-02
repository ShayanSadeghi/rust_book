fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is \"{}\"", result);
}

fn longest(x: &str, y: &str) -> &str {
    // compiler doesn't know the lifetime and won't compile this code
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
