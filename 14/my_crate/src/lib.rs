// running cargo doc create html documentation from section using triple slash("///")
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
