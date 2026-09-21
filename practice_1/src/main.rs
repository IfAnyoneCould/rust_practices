use std::collections::HashMap;

fn count_words(text: &str) -> HashMap<String, u32> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word.to_string()).or_insert(0) += 1
    }
    counts
}

fn most_common(counts: &HashMap<String, u32>) -> Option<(&String, &u32)> {
    counts.iter().max_by_key(|&(_, &count)| count)
}

fn main() {
    let text = "the quick brown fox jumped over the lazy dog";
    let counts = count_words(text);
    match most_common(&counts) {
        Some((word, count)) => println!("Most common word is '{word}', occuring {count} times"),
        None => println!("Empty string"),
    }
}
