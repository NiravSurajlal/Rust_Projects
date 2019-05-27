use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // string keys, i32 values 
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // zip() creates vector of tuples, collect to turn vec into HashMap
    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // ownership when input into HM
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // accessing values
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    // iterate through
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

}
