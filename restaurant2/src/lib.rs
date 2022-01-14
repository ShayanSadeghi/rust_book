// section 7.3
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {} // both parent and child should be public if we want to access them by eat_at_restaurant function
    }
}

// "front_of_house" is at a same level with "eat_at_restaurant" so there is no need to use "pub" for "front_of_house" module
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist(); // "crate" keyword used to define absolute paths

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

//----------------------------------
// "super" in module path is something like ".." in filesystem path

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); // to have access to the outer functions, we use "super" keyword
                              // the outer materials are accessible by inner ones. so there is no need to use "pub" for "serve_order" function
    }

    fn cook_order() {}
}
