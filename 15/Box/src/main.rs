enum List {
    Cons(i32, Box<List>),
    Nil,
} // now the Cons will need the size of an i32 + the space to store the box's pointer data

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
