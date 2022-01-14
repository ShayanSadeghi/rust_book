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
