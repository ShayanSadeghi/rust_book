// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {
//             println!("new customer arrived!");
//         }
//     }
// }

// mod front_of_house; // using ";" instead of code block, tells Rust to search and import code in "front_of_house.rs"

// pub use self::front_of_house::hosting; //"hosting" is now a valid name in root scope
//                                        // "pub use" re-export the module. each module import th lib, has access to hosting too.

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
// }
