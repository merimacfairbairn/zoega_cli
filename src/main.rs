use multimap::MultiMap;
use serde::{Deserialize, Serialize};
use std::fs::{self};
use std::process::exit;

#[derive(Debug, Serialize, Deserialize)]
struct Pair {
    word: String,
    definitions: Vec<String>,
}

fn main() {
    let word = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please provide a word");
        exit(1);
    });

    let dict_path = "data/dictionary.json";

    let contents = fs::read_to_string(&dict_path).expect("Could not load dictionary");
    let json: Vec<Pair> = serde_json::from_str(&contents).expect("Malformed JSON");

    let mut word_map = MultiMap::new();
    for pair in &json {
        word_map.insert(&pair.word, &pair.definitions);
    }

    match word_map.get(&word) {
        Some(definitons) => {
            definitons.into_iter().for_each(|definition| {
                println!("{}\n", definition);
            });
        }
        None => {
            println!("The given word does not exist");
            exit(1);
        }
    };
}
