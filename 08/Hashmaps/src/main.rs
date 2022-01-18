// section 8.3
use std::collections::HashMap;

fn main() {
    let mut person = HashMap::new();
    person.insert(String::from("name"), String::from("Shayan"));
    person.insert(String::from("age"), String::from("25")); //both keys and values for all instances should use the same type

    println!("{:?}", person);

    // use iterators and collect method to create a Hashmap
    let teams = vec!["Team A", "Team B"];
    let initial_scores = vec![1, 3];

    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("{:?}", scores);
}
