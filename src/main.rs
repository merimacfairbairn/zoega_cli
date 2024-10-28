use multimap::MultiMap;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::process::exit;
use std::{env, fs};

#[derive(Debug, Serialize, Deserialize)]
struct Pair {
    word: String,
    definitions: Vec<String>,
}

fn main() {
    let word = env::args().nth(1).unwrap_or_else(|| {
        println!("Please provide a word");
        exit(1);
    });

    let pairs_map = get_pairs_map("data/dictionary.json");
    match pairs_map.get(&word) {
        Some(definitons) => {
            definitons.into_iter().for_each(|definition| {
                println!("{}\n", definition);
            });
        }
        None => {
            let suggestions = suggest_words(&word, &pairs_map);
            if suggestions.is_empty() || word.len() <= 2 {
                println!("No matches found");
            } else {
                println!("Did you mean one of these?");
                for suggestion in suggestions {
                    println!(" - {}", suggestion);
                }
            }
        }
    };

    suggest_upper_if_exists(&word, &pairs_map);
}

fn suggest_words(word: &str, data: &MultiMap<String, Vec<String>>) -> Vec<String> {
    let regex_pattern = format!(r"(?i)^{}.*", regex::escape(&word));
    let regex = Regex::new(&regex_pattern).expect("Invalid regex pattern");

    let mut matches: Vec<(String, usize)> = data
        .keys()
        .filter(|key| regex.is_match(key))
        .map(|key| {
            let extra_chars = key.len().saturating_sub(word.len());
            (key.clone(), extra_chars)
        })
        .collect();

    matches.sort_by_key(|&(_, extra_chars)| extra_chars);
    matches.into_iter().take(5).map(|(key, _)| key).collect()
}

fn suggest_upper_if_exists(word: &str, pairs_map: &MultiMap<String, Vec<String>>) {
    let word_upper = &word.to_uppercase();
    if pairs_map.get(word_upper).is_some() && word.chars().all(|c| c.is_lowercase()) {
        println!("See capitalized version: {}", word_upper);
    }
}

fn get_pairs_map(path: &str) -> MultiMap<String, Vec<String>> {
    let contents = fs::read_to_string(path).expect("Could not load dictionary");
    let json: Vec<Pair> = serde_json::from_str(&contents).expect("Malformed JSON");

    // TODO: Is there a way to avoid cloning?
    let mut pairs_map = MultiMap::new();
    for pair in &json {
        pairs_map.insert(pair.word.clone(), pair.definitions.clone());
    }

    pairs_map
}
