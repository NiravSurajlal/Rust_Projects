use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // string keys, i32 values 
    scores.insert(String::from("Blue_s"), 10);
    scores.insert(String::from("Yellow_s"), 50);

    // zip() creates vector of tuples, collect to turn vec into HashMap
    let teams  = vec![String::from("Blue_s2"), String::from("Yellow_s2")];
    let initial_scores = vec![10, 50];
    let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // ownership when input into HM
    let field_name = String::from("Favorite color_m");
    let field_value = String::from("Blue_m");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // accessing values
    let team_name = String::from("Blue_s2");
    let scores2_val = scores2.get(&team_name);
    // cannot be printed because Option<&V> type
    println!("val from scores2 Blue_s2 key is {}", scores2_val.unwrap());
    println!("{:?}", scores2_val);
    // let scores2_out = get_scores_val(scores2_val);
    // println!("{}", scores2_out);

    // iterate through
    println!("");
    for (key, value) in &scores {
        println!("val from scores {} key: {}", key, value);
    }
    println!("");
    for (key, value) in &scores2 {
        println!("val from scores2 {} key: {}", key, value);
    }
    println!("");
    for (key, value) in &map {
        println!("val from map {} key: {}", key, value);
    }

    let q: i32 = 21;
    println!("{}", q);

    // insert if value is empty (associated with a key)
    let mut empty_key = HashMap::new();
    empty_key.insert(String::from("Blue"), 10);
    // only changes "Yellow" value because "Blue" & val already exists
    empty_key.entry(String::from("Yellow")).or_insert(50);
    empty_key.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", empty_key);

    // updating HM's based on previous vals
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
      let count = map.entry(word).or_insert(0);
      *count += 1;
    }
    println!("{:?}", map);

}

fn get_scores_val(x: Option<&i32>) -> Option<i32>{
    match x{
        None => None,
        Some(i) => Some(*i),
    }
} 
