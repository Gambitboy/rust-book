use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{}", score);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    scores.entry(String::from("Green")).or_insert(100);
    scores.entry(String::from("Blue")).or_insert(75);

    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    #[derive(Eq, Hash, PartialEq, Debug)]
    enum Multiple {
        Text(String),
        Integer(i32),
    }

    let mut multiple = HashMap::new();

    multiple.insert(Multiple::Text("Hello World".to_string()), 10);
    multiple.insert(Multiple::Integer(10), 10);

    println!("{:?}", multiple);
}