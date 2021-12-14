fn main() {
    let s1 = String::from("Hello, world!");

    let len = len_calc(&s1); // "&" used for referencing to s1 (pointer concept), we borrow s1 to "len_calc" function
    println!("The length of s1 is {}", { len });
}

fn len_calc(s: &String) -> usize {
    s.len()
}
