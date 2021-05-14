use std::collections::HashMap;

fn main() {
    println!("Hash maps...");

    println!("manual construction...");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("scores: {:?}", scores);
    println!("-------------------------------------\n");

    println!("manual construction...");
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("scores: {:?}", scores);
    println!("-------------------------------------\n");

    println!("ownership...");
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    // println!("field_name: {}", field_name);
    println!("map: {:?}", map);
    println!("-------------------------------------\n");

    println!("getting a value, returns an option i.e. Some(val) or None...");
    let fav_color = map.get("Favorite color");
    println!("fav_color: {}", fav_color.unwrap());
    println!("-------------------------------------\n");

    println!("looping over values in arbitrary order...");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    println!("-------------------------------------\n");

    println!("inserting over an existing key");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 20);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    println!("-------------------------------------\n");

    println!("only insert if key not set...");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);
    scores.entry(String::from("Blue")).or_insert(30);
    scores.entry(String::from("Red")).or_insert(40);
    println!("scores:{:?}", scores);
    println!("-------------------------------------\n");

    println!("update existing value...");
    let text = "hello world hello wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
    println!("-------------------------------------\n");


}
