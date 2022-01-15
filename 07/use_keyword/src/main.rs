// use std::io::Result as IoResult
mod lib;
fn main() {
    lib::eat_at_restaurant();
    lib::hosting::add_to_waitlist(); // it is because of re-exporting
}
