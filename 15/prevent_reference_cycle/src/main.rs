use std::cell::RefCell;
use std::rc::Rc;

// we use a tree (parent-children) data structure so we don't have a cycle anymore
// if a parent node dropped, its child nodes should be dropped as well
// child node doesn't refer to its parent so it can dropped easily

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    }); // leaf has no reference to to branch

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    }); // leaf is accessible through branch.children
}
