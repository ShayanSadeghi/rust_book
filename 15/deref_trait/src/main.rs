use std::ops::Deref;

struct MyBox<T>(T); // MyBox is a tuple struct that have a generic type element

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    // without implementing Deref trait, rust don't know what to do if we want to derefrence a MyBox<T> refrence
    type Target = T; // define the associated type for the Deref

    fn deref(&self) -> &Self::Target {
        //deref method gives the compiler the ability to take a value of any type that implements Deref
        &self.0 // deref will return a reference to the value we want to access with the * operator.
                // we return a reference to the value we want...
                // because if we return the value directly, the value would be moved out of self...
                // (ownership woul be taken in the direct return)
    }
}

fn main() {
    let x = 5;
    let y = &x; //simple refrence to x

    assert_eq!(5, x);
    assert_eq!(5, *y); // simple deref of y

    let x2 = 5;
    let y2 = MyBox::new(x);

    assert_eq!(5, x2);
    assert_eq!(5, *y2); //rust will run *(y2.deref())
}
