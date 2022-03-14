use std::cell::RefCell;
use std::rc::{Rc, Weak};

// we use a tree (parent-children) data structure so we don't have a cycle anymore
// if a parent node dropped, its child nodes should be dropped as well
// child node doesn't refer to its parent so it can dropped easily
// so we have to use weak references, if we want to refer to parent from a child node

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()), // leaf starts out without a parent
        children: RefCell::new(vec![]),
    }); // leaf has no reference to branch

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        }); // leaf is accessible through branch.children
            // use 'downgrade' to create a 'Weak<Node>' reference to branch

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // now child has access to parent
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
