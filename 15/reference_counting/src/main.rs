enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a: {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a)); // Increase refrence count by 1
    println!("count after creating b: {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a)); // Increase reerece count by 1
        println!("count after creating c: {}", Rc::strong_count(&a));
    }
    // after the scope, c is droped, so rc_count decreased by 1
    println!("count out of the scope of c: {}", Rc::strong_count(&a));
}
