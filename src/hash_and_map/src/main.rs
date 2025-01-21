fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("favorite color");
    let field_value = String::from("blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    for (key, value) in &map {
        println!("{key}: {value}");
    }
    // println!("{field_name}");

    let mut map_insert = HashMap::new();
    map_insert.insert(String::from("blue"), 10);
    map_insert.insert(String::from("blue"), 20);
    println!("{map_insert:?}");
    map_insert.entry(String::from("yellow")).or_insert(50);
    map_insert.entry(String::from("blue")).or_insert(50);
    println!("{map_insert:?}");

    let text = "hello world wonderful world";
    let mut maps = HashMap::new();
    for word in text.split_whitespace() {
        let count = maps.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{maps:?}");
}
