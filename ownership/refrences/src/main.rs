fn main() {
    let mut s1 = String::from("Hello"); //define a mutable variable

    change(&mut s1); // borrow the value and mutation access
    println!("s1 value after change is {}", s1);
}

fn change(s: &mut String) {
    // accept and string which has allowed to mutate
    s.push_str(", world!")
}
