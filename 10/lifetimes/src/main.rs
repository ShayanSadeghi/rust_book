fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is \"{}\"", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // lifetime names started with an apostrophe
    // this code means the lifetime of the reference returned by "longest" function is the same as
    // ...the smaller of the liftetimes of the refrences passed in
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
