// section 7.3
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {} // both parent and child should be public if we want to access them by eat_at_restaurant function
//     }
// }

// "front_of_house" is at a same level with "eat_at_restaurant" so there is no need to use "pub" for "front_of_house" module
// pub fn eat_at_restaurant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist(); // "crate" keyword used to define absolute paths

//     // Relative path
//     front_of_house::hosting::add_to_waitlist();
// }

//----------------------------------
// "super" in module path is something like ".." in filesystem path

// fn serve_order() {}

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::serve_order(); // to have access to the outer functions, we use "super" keyword
//                               // the outer materials are accessible by inner ones. so there is no need to use "pub" for "serve_order" function
//     }

//     fn cook_order() {}
// }

//----------------------------------
// public structs and enums

mod back_of_house {
    pub struct Breakfast {
        // the struct is public but each field will remain private. so public fields should be annotated by "pub".
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // because we have a private field in "Breakfast" struct, we should use a public associated function (here named "summer")...
        // ... to be able to create an instance of "Breakfast" struct later
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        // all variants in enum getting public now
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat"); // we can change public field
                                        // seasonal_fruit is private filed and there is no access to read or change
    let order = back_of_house::Appetizer::Salad;
}
