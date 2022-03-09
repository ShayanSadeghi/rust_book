fn main() {
    let x= 5;
    let y = &mut x; // with default rust analysis, we can't borrow a immutable value mutably...
                    // and it's why we may need RefCell<T>...
                    // RefCell<T> will check mutable borrowing rules at runtime
}
