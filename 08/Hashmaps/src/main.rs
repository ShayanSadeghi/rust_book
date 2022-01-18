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

    //Access values in HashMap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    if let Some(s) = score {
        println!("{}", s);
    } else {
        println!("Wrong key");
    }

    let team_name_2 = String::from("Red");
    let score_2 = scores.get(&team_name_2);
    if let Some(s) = score_2 {
        println!("{}", s);
    } else {
        println!("Wrong key");
    }

    // iterating over Hashmap
    for (k, v) in &scores {
        println!("{}\t{}", k, v);
    }
}
