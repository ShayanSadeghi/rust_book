fn main() {
    let x = 5;
    let y = &x; //simple refrence to x

    assert_eq!(5, x);
    assert_eq!(5, *y); // simple deref of y
}
