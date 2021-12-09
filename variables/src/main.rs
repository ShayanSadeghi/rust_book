fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    // if we don't use "mut", we can't change x now
    x = 6;
    println!("The value of x is {}", x);
}
