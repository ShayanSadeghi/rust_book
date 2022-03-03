enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a)); // here 'a' moved into b
    let c = Cons(4, Box::new(a)); // here we want to use 'a' again but it's moved before
}
