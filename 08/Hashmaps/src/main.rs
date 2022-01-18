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

    // ownership
    let field_name = String::from("favorite_color");
    let field_value = String::from("red");
    let mut m = HashMap::new();
    m.insert(field_name, field_value); // field_name and field_value are not available now
    println!("{:#?}", m);

    let field_name_2 = String::from("Age");
    let field_value_2 = 25;
    let mut m2 = HashMap::new();
    m2.insert(field_name_2, field_value_2); // int will be copied, so we can use it later
    println!("{:#?}", m2);
    println!("{}", field_value_2);
}
