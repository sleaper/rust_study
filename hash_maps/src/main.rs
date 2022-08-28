use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_score = String::from("Blue");
    let _score = scores.get(&team_score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    hash_ownership();
    overwriting_hash_values();
}

fn hash_ownership() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point
}

fn overwriting_hash_values() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    // 10 will be overwritten with 25
    scores.insert(String::from("Blue"), 25);
    // println!("{:?}", scores);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);
}

fn hash_update_values() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
