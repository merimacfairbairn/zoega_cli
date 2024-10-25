use multimap::MultiMap;
use serde::{Serialize, Deserialize};
use std::fs::{self};

#[derive(Debug, Serialize, Deserialize)]
struct Pair {
    word: String,
    definitions: Vec<String>,
}

fn main() {
    let word = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please provide a word");
        std::process::exit(1);
    });

    let dict_path = "data/dictionary.json".to_owned();
    let contents = fs::read_to_string(dict_path).expect("Could not load dictionary");

    let json: Vec<Pair> = serde_json::from_str(&contents).expect("Malformed JSON");

    let mut word_map = MultiMap::new();

    for pair in &json {
        word_map.insert(pair.word.clone(), pair.definitions.clone());
    }

    for i in &word_map[&word] {
        println!("{}\n", i);
    }
}
