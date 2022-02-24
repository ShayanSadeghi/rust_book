// commenting to define the purpose of the "my_crate" not just a function

//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain calculations.
//! it is just a test to describe a whole crate

// running cargo doc create html documentation from section using triple slash("///")
// the following command is just for the function that came after the doc-comments

/// Adds one to the number given.
///
/// # Examples
///
// this block run in test (cargo test)
/// ```
/// let arg = 5;
/// let answere = my_crate::add_one(arg);
///
/// assert_eq!(6, answere);
/// ```

pub fn add_one(x: i32) -> i32 {
    x + 1
}
