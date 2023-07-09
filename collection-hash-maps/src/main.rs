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

    let (median, mode) = find_median_and_mode();
    println!("{} {}", median, mode);

    let pig_laten = convert_string_to_pig_laten();
    println!("{}", pig_laten);
}

fn find_median_and_mode() -> (i32, i32) {
    // Build vector of numbers
    let numbers = vec![1, 2, 3, 4, 5, 5, 6, 7, 8];

    let median_index = numbers.len() / 2;
    let median = numbers[median_index];

    let mut numbers_count = HashMap::new();

    for number_index in &numbers {
        let count = numbers_count.entry(number_index).or_insert(0);
        let new_count = *count + 1;
        numbers_count.insert(number_index, new_count);
    }

    let options = numbers_count
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| *k);

    let highest = match options {
        Some(value) => *value,
        None => 0,
    };

    (median, highest)
}

fn convert_string_to_pig_laten() -> String {
    let vowels = ['a', 'e', 'i', 'o', 'i'];
    let message = "I really like dogs and apples".to_lowercase().to_string();
    let mut words: Vec<String> = message.split_whitespace().map(|s| s.to_string()).collect();

    for word in words.iter_mut() {
        if vowels.contains(&word.chars().nth(0).unwrap()) {
            *word = word.to_string() + "hay";
        } else {
            *word = word[1..].to_string() + &word[0..1].to_string() + "ay"
        }
    }

    words.join(" ")
}
