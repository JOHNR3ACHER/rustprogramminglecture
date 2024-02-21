
fn main() {
    use std::collections::HashMap;

    let mut populations: HashMap<String, i32> = HashMap::new();
    populations.insert("UTRGV".to_string(), 30000);
    populations.insert("Edinburg".to_string(), 97000);

    // Accessing values
    if let Some(pop) = populations.get("UTRGV") {
        println!("UTRGV population: {}", pop);
    }

    // Iterating over key-value pairs
    for (key, value) in &populations {
        println!("{}: {}", key, value);
    }

    // Updating a value
    *populations.entry("UTRGV".to_string()).or_insert(0) = 45000;
    println!("{:?}", populations);

    // Word count example
    let text = "hello world hello";
    let mut word_count = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", word_count);
}
}