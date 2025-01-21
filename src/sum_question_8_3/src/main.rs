use std::collections::HashMap;

fn main() {
    let text = String::from("Add Sally to Engineering");
    let text_sp: Vec<&str> = text.split_whitespace().collect();
    let mut maps = HashMap::new();
    maps.insert(text_sp[1], text_sp[3]);
    println!("{maps:#?}");
}
