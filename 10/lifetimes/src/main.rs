fn main() {
    let string1 = String::from("abcd");
    {
        let string2 = "xyz";
        let result = longest(string1.as_str(), string2);
        // if we try to print result out of this inner scope,
        // compiler shows an error because the lifetime of string2 is not
        // longer than this scope and result lifetime is equal to
        // shorter lifetime of string1 and string 2
        println!("The longest string is \"{}\"", result);
    }
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
