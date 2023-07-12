use std::collections::HashMap;

fn char_counter(s: &str) -> HashMap<char, i32> {
    let mut char_count = HashMap::new();
    for letter in s.chars() {
        let count = char_count.entry(letter).or_insert(0);
        *count += 1;
    }

    return char_count;
}

pub fn test() {
    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("red"), 20);

    println!("scores: {scores:?}");

    let team_name = String::from("red");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Team {team_name} scored {score}");

    println!("Full score: ");
    for (key, value) in scores {
        println!("Team {key} scored {value}");
    }

    let field_name = String::from("favorite color");
    let field_value = String::from("blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    println!("map: {map:?}");
    // this fails:
    // println!("{field_name} has the value {field_value}");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    println!("Before adding new key Yellow to {scores:?}");
    scores.entry(String::from("Yellow")).or_insert(50);
    println!("After adding new key key Yellow: {scores:?}");

    println!("Trying to re-add existing key Blue to {scores:?}");
    scores.entry(String::from("Blue")).or_insert(20);
    println!("After trying to re-add existing key Blue: {scores:?}");

    let string = "This is a test string";
    let word_count = char_counter(string);
    println!("char count = {word_count:?}");
}
